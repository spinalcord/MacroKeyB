use x11rb::protocol::xtest::ConnectionExt as XTestConnectionExt;
use std::fmt;
use std::error::Error;
use x11rb::connection::Connection;
use std::{thread, time::Duration};


// X11 Event Types
const KEY_PRESS: u8 = 2;
const KEY_RELEASE: u8 = 3;

// Error handling
#[derive(Debug)]
pub enum KeySimError {
    X11ReplyError(x11rb::errors::ReplyError),
    X11ConnectionError(x11rb::errors::ConnectionError),
    LuaError(mlua::Error),
    OtherError(String),
}

impl fmt::Display for KeySimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeySimError::X11ReplyError(e) => write!(f, "X11 reply error: {}", e),
            KeySimError::X11ConnectionError(e) => write!(f, "X11 connection error: {}", e),
            KeySimError::LuaError(e) => write!(f, "Lua error: {}", e),
            KeySimError::OtherError(s) => write!(f, "Error: {}", s),
        }
    }
}

impl Error for KeySimError {}

impl From<x11rb::errors::ReplyError> for KeySimError {
    fn from(error: x11rb::errors::ReplyError) -> Self {
        KeySimError::X11ReplyError(error)
    }
}

impl From<x11rb::errors::ConnectionError> for KeySimError {
    fn from(error: x11rb::errors::ConnectionError) -> Self {
        KeySimError::X11ConnectionError(error)
    }
}

impl From<mlua::Error> for KeySimError {
    fn from(error: mlua::Error) -> Self {
        KeySimError::LuaError(error)
    }
}

impl From<String> for KeySimError {
    fn from(error: String) -> Self {
        KeySimError::OtherError(error)
    }
}

impl From<&str> for KeySimError {
    fn from(error: &str) -> Self {
        KeySimError::OtherError(error.to_string())
    }
}

// Main structure for X11 connection and key inputs
pub struct KeyboardTrigger {
    conn: x11rb::rust_connection::RustConnection,
}

impl KeyboardTrigger {
    pub fn new() -> Result<Self, KeySimError> {
        let (conn, _) = x11rb::connect(None)
            .map_err(|e| KeySimError::OtherError(format!("Failed to connect to X11 server: {}", e)))?;
        Ok(KeyboardTrigger { conn })
    }

    // Optimized method: Batch sending for multiple events
    fn send_key_events(&self, events: &[(u8, u8)]) -> Result<(), KeySimError> {
        for &(event_type, keycode) in events {
            self.conn.xtest_fake_input(event_type, keycode, 0, 0, 0, 0, 0)?;
        }
        self.conn.flush()?;
        Ok(())
    }

    // Presses a key
    pub fn press(&self, keycode: u8) -> Result<(), KeySimError> {
        self.send_key_events(&[(KEY_PRESS, keycode)])
    }

    // Releases a key
    pub fn release(&self, keycode: u8) -> Result<(), KeySimError> {
        self.send_key_events(&[(KEY_RELEASE, keycode)])
    }

    // Optimized method: Faster key tap with less delay
    pub fn tap(&self, keycode: u8, delay_ms: Option<u64>) -> Result<(), KeySimError> {
        let delay = delay_ms.unwrap_or(50);
        let events = [(KEY_PRESS, keycode), (KEY_RELEASE, keycode)];
        self.send_key_events(&events[..1])?; // Only the press
        if delay > 0 {
            thread::sleep(Duration::from_millis(delay));
        }
        self.send_key_events(&events[1..])?; // Only the release
        Ok(())
    }

    // Waits for a specific duration
    pub fn wait(&self, duration: u64) -> Result<(), KeySimError> {
        thread::sleep(Duration::from_millis(duration));
        Ok(())
    }

    // Improved key combination with minimal flushes
    pub fn combo(&self, keycodes: &[u8], delay_ms: Option<u64>) -> Result<(), KeySimError> {
        if keycodes.is_empty() {
            return Err("No keys specified in the combination".into());
        }

        let delay = delay_ms.unwrap_or(100);

        // Create events for pressing all keys
        let mut press_events = Vec::with_capacity(keycodes.len());
        for &keycode in keycodes {
            press_events.push((KEY_PRESS, keycode));
        }

        // Create events for releasing all keys (in reverse order)
        let mut release_events = Vec::with_capacity(keycodes.len());
        for &keycode in keycodes.iter().rev() {
            release_events.push((KEY_RELEASE, keycode));
        }

        // A short pause before the combination to avoid issues
        thread::sleep(Duration::from_millis(20));

        // Send all key presses
        self.send_key_events(&press_events)?;

        // Wait the specified time
        thread::sleep(Duration::from_millis(delay));

        // Send all key releases
        self.send_key_events(&release_events)?;

        // A short pause after the combination
        thread::sleep(Duration::from_millis(20));

        Ok(())
    }

    // Flushes the buffer
    pub fn flush(&self) -> Result<(), KeySimError> {
        self.conn.flush()?;
        Ok(())
    }
}

// Implement Drop for clean cleanup
impl Drop for KeyboardTrigger {
    fn drop(&mut self) {
        // Ensure all pending events are sent
        let _ = self.conn.flush();
    }
}