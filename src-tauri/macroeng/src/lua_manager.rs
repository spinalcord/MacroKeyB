use std::{error::Error, sync::Mutex};
use std::process::Command;
use std::sync::Arc;
use std::collections::HashMap;
use crate::keyboard_trigger::KeyboardTrigger;
use mlua::prelude::*;
use clipboard::{ClipboardContext, ClipboardProvider};

pub struct LuaManager {
    script: Arc<KeyboardTrigger>,
    lua: Lua,
    key_map: HashMap<String, u8>,
    clipboard_ctx: Arc<Mutex<ClipboardContext>>, // NEW
}
impl LuaManager {
pub fn new() -> Result<Self, Box<dyn Error>> {
        let script = Arc::new(KeyboardTrigger::new()?);
        let lua = Lua::new();
        let key_map = Self::create_key_map();

        // Initialize ClipboardContext here once
        let clipboard_provider = ClipboardProvider::new().map_err(|e| {
            // Convert the clipboard error to a Box<dyn Error>
            // The exact error handling depends on your error enum,
            // or if you simply stringify a generic error.
            // Here's an example of how it could be done if clipboard::Error implements Display:
            format!("Error initializing global clipboard: {}", e)
        })?;
        let clipboard_ctx = Arc::new(Mutex::new(clipboard_provider));


        let lua_script = LuaManager {
            script,
            lua,
            key_map,
            clipboard_ctx, // NEW
        };
        lua_script.register_lua_functions()?;

        Ok(lua_script)
    }

    // Creates a mapping from string names to keycodes
    fn create_key_map() -> HashMap<String, u8> {
        let mut map = HashMap::new();
        
        // Control keys
        map.insert("ctrl".to_string(), 37u8);
        map.insert("alt".to_string(), 64u8);
        map.insert("shift".to_string(), 50u8);
        map.insert("enter".to_string(), 36u8);
        map.insert("space".to_string(), 65u8);
        map.insert("tab".to_string(), 23u8);
        map.insert("esc".to_string(), 9u8);
        
        // Letters
        map.insert("a".to_string(), 38u8);
        map.insert("b".to_string(), 56u8);
        map.insert("c".to_string(), 54u8);
        map.insert("d".to_string(), 40u8);
        map.insert("e".to_string(), 26u8);
        map.insert("f".to_string(), 41u8);
        map.insert("g".to_string(), 42u8);
        map.insert("h".to_string(), 43u8);
        map.insert("i".to_string(), 31u8);
        map.insert("j".to_string(), 44u8);
        map.insert("k".to_string(), 45u8);
        map.insert("l".to_string(), 46u8);
        map.insert("m".to_string(), 58u8);
        map.insert("n".to_string(), 57u8);
        map.insert("o".to_string(), 32u8);
        map.insert("p".to_string(), 33u8);
        map.insert("q".to_string(), 24u8);
        map.insert("r".to_string(), 27u8);
        map.insert("s".to_string(), 39u8);
        map.insert("t".to_string(), 28u8);
        map.insert("u".to_string(), 30u8);
        map.insert("v".to_string(), 55u8);
        map.insert("w".to_string(), 25u8);
        map.insert("x".to_string(), 53u8);
        map.insert("y".to_string(), 29u8);
        map.insert("z".to_string(), 52u8);

        // Digits
        map.insert("0".to_string(), 19u8);
        map.insert("1".to_string(), 10u8);
        map.insert("2".to_string(), 11u8);
        map.insert("3".to_string(), 12u8);
        map.insert("4".to_string(), 13u8);
        map.insert("5".to_string(), 14u8);
        map.insert("6".to_string(), 15u8);
        map.insert("7".to_string(), 16u8);
        map.insert("8".to_string(), 17u8);
        map.insert("9".to_string(), 18u8);

        // More keys can be added as needed
        
        map
    }

    // Helper function to convert string to keycode
    fn key_to_code(&self, key: &str) -> Result<u8, String> {
        self.key_map.get(&key.to_lowercase())
            .copied()
            .ok_or_else(|| format!("Unknown key: '{}'", key))
    }

    fn format_lua_error(err: mlua::Error, param_name: &str) -> String {
        // script_name here is the name passed to .set_name(),
        // e.g., "Key A" in your case.
        let mut error_message = format!("Error to assigned key '{}':\n", param_name);

        match err {
            mlua::Error::SyntaxError { message, .. } => {
                // The `message` from mlua is typically: "[string "SCRIPT_NAME"]:LINE: error message"
                // or "SCRIPT_NAME:LINE: error message" when loaded from a file.
                // Since you are using .set_name(), it will have the form "[string "SCRIPT_NAME"]...".

                // We create the pattern we want to remove.
                // E.g., "[string "Key A"]:"
                let prefix_to_remove = format!("[string \"{}\"]:", param_name);
                
                let cleaned_message = if let Some(stripped_message) = message.strip_prefix(&prefix_to_remove) {
                    // If the prefix was successfully removed, we get e.g. "5: syntax error near <eof>"
                    // We could also apply .trim_start() here in case of unexpected whitespace,
                    // but typically this is not necessary after the colon.
                    stripped_message.to_string()
                } else {
                    // Fallback: If the prefix didn't look exactly like that (unlikely for SyntaxError from mlua).
                    // One could try to remove only `[string "SCRIPT_NAME"]` here
                    // and then manually handle the colon and whitespace.
                    // For now, we'll keep the original message if stripping fails.
                    message
                };
                
                error_message.push_str(&format!("Syntax Error: {}", cleaned_message));
            }
            mlua::Error::RuntimeError(ref _lua_err) => { // _lua_err unused because err.to_string() is used
                // The Display Impl of mlua::Error already includes the traceback.
                error_message.push_str(&format!("{}", err)); // This includes the traceback
            }
            mlua::Error::CallbackError { traceback, cause } => {
                error_message.push_str(&format!("Error in Rust callback:\nCause: {}\nLua Traceback:\n{}", cause, traceback));
            }
            // Handle other mlua::Error variants as needed
            _ => {
                // Standard formatting for other errors
                error_message.push_str(&format!("{}", err));
            }
        }
        error_message
    }

    pub fn run_script_with_name(&self, content: &str, name: &str) -> Result<(), String> {
        self.lua.load(content)
            .set_name(name)
            .exec()
            .map_err(|e| Self::format_lua_error(e, name))
    }

    // Optimized registration: More efficient error handling
    fn register_lua_functions(&self) -> Result<(), Box<dyn Error>> {
        let globals = self.lua.globals();

        // Register Lua functions
        let script_ref = self.script.clone();
        let key_map_ref = self.key_map.clone();

        // Press function - now accepts both strings and u8
        globals.set("press", self.lua.create_function(move |_, key: LuaValue| {
            match key {
                LuaValue::String(s) => {
                    let key_str = s.to_str().map_err(|e| mlua::Error::external(e))?;
                    let keycode = key_map_ref.get(&key_str.to_lowercase())
                        .ok_or_else(|| mlua::Error::external(format!("Unknown key: '{}'", key_str)))?;
                    script_ref.press(*keycode).map_err(|e| mlua::Error::external(e))
                },
                LuaValue::Integer(i) => {
                    if i < 0 || i > 255 {
                        return Err(mlua::Error::external(format!("Keycode must be between 0 and 255: {}", i)));
                    }
                    script_ref.press(i as u8).map_err(|e| mlua::Error::external(e))
                },
                _ => Err(mlua::Error::external("Key must be a string or a keycode (number)")),
            }
        })?)?;

    

        // Release function - also supports strings
        let script_ref = self.script.clone();
        let key_map_ref = self.key_map.clone();
        globals.set("release", self.lua.create_function(move |_, key: LuaValue| {
            match key {
                LuaValue::String(s) => {
                    let key_str = s.to_str().map_err(|e| mlua::Error::external(e))?;
                    let keycode = key_map_ref.get(&key_str.to_lowercase())
                        .ok_or_else(|| mlua::Error::external(format!("Unknown key: '{}'", key_str)))?;
                    script_ref.release(*keycode).map_err(|e| mlua::Error::external(e))
                },
                LuaValue::Integer(i) => {
                    if i < 0 || i > 255 {
                        return Err(mlua::Error::external(format!("Keycode must be between 0 and 255: {}", i)));
                    }
                    script_ref.release(i as u8).map_err(|e| mlua::Error::external(e))
                },
                _ => Err(mlua::Error::external("Key must be a string or a keycode (number)")),
            }
        })?)?;


// Exec Bash function - improved for GUI applications
// Exec Bash function - with "output" option
globals.set("exec_bash", self.lua.create_function(move |lua_ctx, params: LuaMultiValue| {
    use std::process::{Command, Stdio};
    use std::env;
    use std::os::unix::fs::PermissionsExt;
    use std::fs;
    
    // Extract parameters
    let mut params_iter = params.into_iter();
    
    // First parameter: command (mandatory)
    let command = match params_iter.next() {
        Some(LuaValue::String(s)) => s.to_str()
            .map_err(|e| mlua::Error::external(format!("Invalid command: {}", e)))?
            .to_string(),
        _ => return Err(mlua::Error::external("Command must be specified"))
    };
    
    // Second parameter: options array (optional)
    let mut options = Vec::new();
    if let Some(LuaValue::Table(t)) = params_iter.next() {
        for pair in t.pairs::<i32, LuaValue>() {
            let (_, option_value) = pair.map_err(|e| mlua::Error::external(format!("Invalid option: {}", e)))?;
            if let LuaValue::String(s) = option_value {
                let option_str = s.to_str()
                    .map_err(|e| mlua::Error::external(format!("Invalid option: {}", e)))?
                    .to_lowercase();
                options.push(option_str);
            }
        }
    }
    
    // Evaluate options
    let return_output = options.contains(&"output".to_string());
    let wait = options.contains(&"wait".to_string()) || return_output;
    let background = options.contains(&"background".to_string()) && !return_output;
    let use_root = options.contains(&"root".to_string());
    let capture_output = !background || return_output;
    
    // Prepare environment variables for X11
    let display = env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string());
    let xauth_path = env::var("XAUTHORITY")
        .or_else(|_| {
            let user = env::var("USER").unwrap_or_else(|_| "nobody".to_string());
            let path = format!("/home/{}/.Xauthority", user);
            if fs::metadata(&path).is_ok() {
                Ok(path)
            } else {
                Err(())
            }
        })
        .unwrap_or_else(|_| "/tmp/.Xauthority".to_string());

    // Prepare command
    let mut cmd;
    let uid = unsafe { libc::geteuid() };
    
    if use_root {
        // Execute with root privileges
        cmd = Command::new("sudo");
        cmd.arg("-n"); // No password prompt
        cmd.env("DISPLAY", &display);
        if fs::metadata(&xauth_path).is_ok() {
            cmd.env("XAUTHORITY", &xauth_path);
        }
        cmd.arg("bash").arg("-c").arg(&command);
    } else if uid == 0 {
        // Running as root, switch to normal user
        let username = env::var("SUDO_USER")
            .or_else(|_| env::var("USER"))
            .unwrap_or_else(|_| "nobody".to_string());
        
        cmd = Command::new("sudo");
        cmd.arg("-u").arg(&username);
        cmd.env("DISPLAY", &display);
        if fs::metadata(&xauth_path).is_ok() {
            cmd.env("XAUTHORITY", &xauth_path);
        }
        cmd.arg("bash").arg("-c").arg(&command);
    } else {
        // Normal execution
        cmd = Command::new("bash");
        cmd.arg("-c").arg(&command);
        cmd.env("DISPLAY", &display);
        if fs::metadata(&xauth_path).is_ok() {
            cmd.env("XAUTHORITY", &xauth_path);
        }
    }
    
    // Configure stdio based on capture_output
    if !capture_output {
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
    }
    
    // Execute command
    if wait {
        if capture_output {
            let output = cmd.output()
                .map_err(|e| mlua::Error::external(format!("Error executing command: {}", e)))?;
            
            if return_output {
                let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
                while stdout.ends_with(|c: char| c.is_whitespace()) {
                    stdout.pop();
                }
                return Ok(LuaValue::String(lua_ctx.create_string(&stdout)?));
            } else {
                let result_table = lua_ctx.create_table()?;
                result_table.set("stdout", String::from_utf8_lossy(&output.stdout).to_string())?;
                result_table.set("stderr", String::from_utf8_lossy(&output.stderr).to_string())?;
                result_table.set("status", output.status.code().unwrap_or(-1))?;
                return Ok(LuaValue::Table(result_table));
            }
        } else {
            let status = cmd.status()
                .map_err(|e| mlua::Error::external(format!("Error executing command: {}", e)))?;
            
            let result_table = lua_ctx.create_table()?;
            result_table.set("status", status.code().unwrap_or(-1))?;
            return Ok(LuaValue::Table(result_table));
        }
    } else {
        let result_table = lua_ctx.create_table()?;
        match cmd.spawn() {
            Ok(_) => {
                result_table.set("success", true)?;
            },
            Err(e) => {
                result_table.set("success", false)?;
                result_table.set("error", format!("{}", e))?;
            }
        }
        return Ok(LuaValue::Table(result_table));
    }
})?)?;

let clipboard_ctx_clone = Arc::clone(&self.clipboard_ctx);

        globals.set("clipboard", self.lua.create_function(move |lua_ctx, arg: Option<String>| {
            // Lock the mutex to get access to the ClipboardContext
            let mut ctx_guard = clipboard_ctx_clone.lock().map_err(|e| {
                // Error if the mutex is "poisoned"
                mlua::Error::external(format!("Error accessing clipboard (Mutex lock failed): {}", e))
            })?;

            match arg {
                // Write mode: pass string
                Some(text) => {
                    ctx_guard.set_contents(text)
                        .map_err(|e| mlua::Error::external(format!("Error writing to clipboard: {}", e)))?;
                    Ok(LuaValue::Nil)
                },
                // Read mode: no argument
                None => {
                    let clipboard_content = ctx_guard.get_contents()
                        .map_err(|e| mlua::Error::external(format!("Error retrieving clipboard content: {}", e)))?;
                    Ok(LuaValue::String(lua_ctx.create_string(&clipboard_content)?))
                }
            }
        })?)?;

        // Tap function - supports strings and increasing complexity
        let script_ref = self.script.clone();
        let key_map_ref = self.key_map.clone();
        globals.set("tap", self.lua.create_function(move |_, params: LuaMultiValue| {
            let mut params_iter = params.into_iter();
            
            // First parameter: key (string or u8)
            let key = params_iter.next().ok_or_else(||
                mlua::Error::external("At least one key must be specified")
            )?;
            
            // Second parameter: Optional delay
            let delay = params_iter.next().map(|v| -> mlua::Result<Option<u64>> {
                match v {
                    LuaValue::Integer(i) if i >= 0 => Ok(Some(i as u64)),
                    LuaValue::Nil => Ok(None),
                    _ => Err(mlua::Error::external("Delay must be a positive number"))
                }
            }).transpose()?.flatten();
            
            // Determine keycode
            let keycode = match key {
                LuaValue::String(s) => {
                    let key_str = s.to_str().map_err(|e| mlua::Error::external(e))?;
                    *key_map_ref.get(&key_str.to_lowercase())
                        .ok_or_else(|| mlua::Error::external(format!("Unknown key: '{}'", key_str)))?
                },
                LuaValue::Integer(i) => {
                    if i < 0 || i > 255 {
                        return Err(mlua::Error::external(format!("Keycode must be between 0 and 255: {}", i)));
                    }
                    i as u8
                },
                _ => return Err(mlua::Error::external("Key must be a string or a keycode (number)")),
            };
            
            script_ref.tap(keycode, delay).map_err(|e| mlua::Error::external(e))
        })?)?;

        // Wait function - remains unchanged
        let script_ref = self.script.clone();
        globals.set("wait", self.lua.create_function(move |_, duration: u64| {
            script_ref.wait(duration)
                .map_err(|e| mlua::Error::external(e))
        })?)?;

        // Combo function - now supports string lists
        let script_ref = self.script.clone();
        let key_map_ref = self.key_map.clone();
        globals.set("combo", self.lua.create_function(move |_, params: LuaMultiValue| {
            let mut params_iter = params.into_iter();
            
            // First parameter: keys (table/array or individual values)
            let keys_param = params_iter.next().ok_or_else(||
                mlua::Error::external("Keys must be specified")
            )?;
            
            // Second parameter: Optional delay
            let delay = params_iter.next().map(|v| -> mlua::Result<Option<u64>> {
                match v {
                    LuaValue::Integer(i) if i >= 0 => Ok(Some(i as u64)),
                    LuaValue::Nil => Ok(None),
                    _ => Err(mlua::Error::external("Delay must be a positive number"))
                }
            }).transpose()?.flatten();
            
            // Collect keycodes
            let keycodes: Vec<u8> = match keys_param {
                LuaValue::Table(t) => {
                    let mut codes = Vec::new();
                    for pair in t.pairs::<i32, LuaValue>() {
                        let (_, key_value) = pair?;
                        let code = match key_value {
                            LuaValue::String(s) => {
                                let key_str = s.to_str().map_err(|e| mlua::Error::external(e))?;
                                *key_map_ref.get(&key_str.to_lowercase())
                                    .ok_or_else(|| mlua::Error::external(format!("Unknown key: '{}'", key_str)))?
                            },
                            LuaValue::Integer(i) => {
                                if i < 0 || i > 255 {
                                    return Err(mlua::Error::external(format!("Keycode must be between 0 and 255: {}", i)));
                                }
                                i as u8
                            },
                            _ => return Err(mlua::Error::external("Key in table must be a string or a keycode (number)")),
                        };
                        codes.push(code);
                    }
                    codes
                },
                LuaValue::String(s) => {
                    let key_str = s.to_str().map_err(|e| mlua::Error::external(e))?;
                    vec![*key_map_ref.get(&key_str.to_lowercase())
                        .ok_or_else(|| mlua::Error::external(format!("Unknown key: '{}'", key_str)))?]
                },
                LuaValue::Integer(i) => {
                    if i < 0 || i > 255 {
                        return Err(mlua::Error::external(format!("Keycode must be between 0 and 255: {}", i)));
                    }
                    vec![i as u8]
                },
                _ => return Err(mlua::Error::external("Keys must be a table, a string, or a keycode (number)")),
            };
            
            if keycodes.is_empty() {
                return Err(mlua::Error::external("No valid keys specified"));
            }
            
            script_ref.combo(&keycodes, delay).map_err(|e| mlua::Error::external(e))
        })?)?;

    // Flush function - remains unchanged
        let script_ref = self.script.clone();
        globals.set("flush", self.lua.create_function(move |_, ()| {
            script_ref.flush()
                .map_err(|e| mlua::Error::external(e))
        })?)?;

        // Register key names as Lua constants
        for (key_name, key_code) in &self.key_map {
            let uppercase_name = format!("KEY_{}", key_name.to_uppercase());
            globals.set(uppercase_name, *key_code)?;
        }

        Ok(())
    }

    pub fn run_script(&self, lua_code: &str) -> LuaResult<()> {
        self.lua.load(lua_code).exec()
    }
}