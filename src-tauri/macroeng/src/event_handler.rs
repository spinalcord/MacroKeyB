// event_handler.rs
use std::sync::{Arc, RwLock};

// Type alias for a callback with a generic argument T
type EventCallback<T> = Box<dyn Fn(&T) + Send + Sync>;

// Event handler structure for storing callbacks with argument support
pub struct EventHandler<T> {
    callbacks: RwLock<Vec<EventCallback<T>>>,
}

impl<T: 'static> EventHandler<T> {
    // Create a new EventHandler
    pub fn new() -> Self {
        EventHandler {
            callbacks: RwLock::new(Vec::new()),
        }
    }

    pub fn clear_listeners(&self) {
        self.callbacks.write().unwrap().clear();
    }
    
    // Add a callback to the event
    pub fn add_listener<F>(&self, callback: F)
    where
        F: Fn(&T) + 'static + Send + Sync,
    {
        self.callbacks.write().unwrap().push(Box::new(callback));
    }

    // Trigger the event with an argument
    pub fn trigger(&self, arg: &T) {
        for callback in self.callbacks.read().unwrap().iter() {
            callback(arg);
        }
    }
}