use std::error::Error;
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
}

impl LuaManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let script = Arc::new(KeyboardTrigger::new()?);
        let lua = Lua::new();
        let key_map = Self::create_key_map();

        let lua_script = LuaManager { script, lua, key_map };
        lua_script.register_lua_functions()?;

        Ok(lua_script)
    }

    // Erstellt eine Zuordnung von String-Namen zu Keycodes
    fn create_key_map() -> HashMap<String, u8> {
        let mut map = HashMap::new();
        
        // Steuerungstasten
        map.insert("ctrl".to_string(), 37u8);
        map.insert("alt".to_string(), 64u8);
        map.insert("shift".to_string(), 50u8);
        map.insert("enter".to_string(), 36u8);
        map.insert("space".to_string(), 65u8);
        map.insert("tab".to_string(), 23u8);
        map.insert("esc".to_string(), 9u8);
        
        // Buchstaben
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

        // Ziffern
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

        // Weitere Tasten können nach Bedarf hinzugefügt werden
        
        map
    }

    // Hilfsfunktion zum Konvertieren von String zu Keycode
    fn key_to_code(&self, key: &str) -> Result<u8, String> {
        self.key_map.get(&key.to_lowercase())
            .copied()
            .ok_or_else(|| format!("Unbekannte Taste: '{}'", key))
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

        // Press function - akzeptiert jetzt sowohl Strings als auch u8
        globals.set("press", self.lua.create_function(move |_, key: LuaValue| {
            match key {
                LuaValue::String(s) => {
                    let key_str = s.to_str().map_err(|e| mlua::Error::external(e))?;
                    let keycode = key_map_ref.get(&key_str.to_lowercase())
                        .ok_or_else(|| mlua::Error::external(format!("Unbekannte Taste: '{}'", key_str)))?;
                    script_ref.press(*keycode).map_err(|e| mlua::Error::external(e))
                },
                LuaValue::Integer(i) => {
                    if i < 0 || i > 255 {
                        return Err(mlua::Error::external(format!("Keycode muss zwischen 0 und 255 sein: {}", i)));
                    }
                    script_ref.press(i as u8).map_err(|e| mlua::Error::external(e))
                },
                _ => Err(mlua::Error::external("Taste muss ein String oder ein Keycode (Zahl) sein")),
            }
        })?)?;

    

        // Release function - unterstützt auch Strings
        let script_ref = self.script.clone();
        let key_map_ref = self.key_map.clone();
        globals.set("release", self.lua.create_function(move |_, key: LuaValue| {
            match key {
                LuaValue::String(s) => {
                    let key_str = s.to_str().map_err(|e| mlua::Error::external(e))?;
                    let keycode = key_map_ref.get(&key_str.to_lowercase())
                        .ok_or_else(|| mlua::Error::external(format!("Unbekannte Taste: '{}'", key_str)))?;
                    script_ref.release(*keycode).map_err(|e| mlua::Error::external(e))
                },
                LuaValue::Integer(i) => {
                    if i < 0 || i > 255 {
                        return Err(mlua::Error::external(format!("Keycode muss zwischen 0 und 255 sein: {}", i)));
                    }
                    script_ref.release(i as u8).map_err(|e| mlua::Error::external(e))
                },
                _ => Err(mlua::Error::external("Taste muss ein String oder ein Keycode (Zahl) sein")),
            }
        })?)?;


// Exec Bash function - verbessert für GUI-Anwendungen
// Exec Bash function - mit "output"-Option
globals.set("exec_bash", self.lua.create_function(move |lua_ctx, params: LuaMultiValue| {
    use std::process::{Command, Stdio};
    use std::env;
    
    // Parameter extrahieren
    let mut params_iter = params.into_iter();
    
    // Erster Parameter: Befehl (obligatorisch)
    let command = match params_iter.next() {
        Some(LuaValue::String(s)) => s.to_str()
            .map_err(|e| mlua::Error::external(format!("Ungültiger Befehl: {}", e)))?
            .to_string(),
        _ => return Err(mlua::Error::external("Befehl muss angegeben werden"))
    };
    
    // Zweiter Parameter: Optionen-Array (optional)
    let mut options = Vec::new();
    if let Some(LuaValue::Table(t)) = params_iter.next() {
        for pair in t.pairs::<i32, LuaValue>() {
            let (_, option_value) = pair.map_err(|e| mlua::Error::external(format!("Ungültige Option: {}", e)))?;
            if let LuaValue::String(s) = option_value {
                let option_str = s.to_str()
                    .map_err(|e| mlua::Error::external(format!("Ungültige Option: {}", e)))?
                    .to_lowercase();
                options.push(option_str);
            }
        }
    }
    
    // Optionen auswerten
    let return_output = options.contains(&"output".to_string());
    let wait = options.contains(&"wait".to_string()) || return_output; // output impliziert wait
    let background = options.contains(&"background".to_string()) && !return_output; // output überschreibt background
    let use_root = options.contains(&"root".to_string());
    let capture_output = !background || return_output;
    
    // Command vorbereiten
    let mut cmd;
    
    if use_root {
        // Mit Root-Rechten ausführen
        cmd = Command::new("sudo");
        cmd.arg("-n"); // Keine Passwortabfrage
        cmd.arg("bash").arg("-c").arg(&command);
    } else {
        // Überprüfen, ob wir als Root laufen und zum normalen Benutzer wechseln müssen
        let uid = unsafe { libc::geteuid() };
        
        if uid == 0 {
            // Wir laufen als Root, müssen zum normalen Benutzer wechseln
            // SUDO_USER sollte den ursprünglichen Benutzer enthalten
            let username = env::var("SUDO_USER").unwrap_or_else(|_| 
                env::var("USER").unwrap_or_else(|_| "nobody".to_string())
            );
            
            // Mit su den Befehl als dieser Benutzer ausführen
            cmd = Command::new("su");
            cmd.args(&["-", &username, "-c", &command]);
        } else {
            // Wir laufen bereits als normaler Benutzer
            cmd = Command::new("bash");
            cmd.arg("-c").arg(&command);
        }
    }
    
    // Stdio konfigurieren basierend auf capture_output
    if !capture_output {
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
    }
    
    // Command ausführen
    if wait {
        // Warten und Ausgabe erfassen, wenn gewünscht
        if capture_output {
            let output = cmd.output()
                .map_err(|e| mlua::Error::external(format!("Fehler beim Ausführen des Befehls: {}", e)))?;
            
            if return_output {
                // Direkt stdout als String zurückgeben, ohne Zeilenumbrüche am Ende
                let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
                // Whitespace am Ende entfernen (Zeilenumbrüche, etc.)
                while stdout.ends_with('\n') || stdout.ends_with('\r') || stdout.ends_with(' ') || stdout.ends_with('\t') {
                    stdout.pop();
                }
                return Ok(LuaValue::String(lua_ctx.create_string(&stdout)?));
            } else {
                // Normale Tabelle zurückgeben
                let result_table = lua_ctx.create_table()?;
                result_table.set("stdout", String::from_utf8_lossy(&output.stdout).to_string())?;
                result_table.set("stderr", String::from_utf8_lossy(&output.stderr).to_string())?;
                result_table.set("status", output.status.code().unwrap_or(-1))?;
                return Ok(LuaValue::Table(result_table));
            }
        } else {
            // Warten, aber keine Ausgabe erfassen
            let status = cmd.status()
                .map_err(|e| mlua::Error::external(format!("Fehler beim Ausführen des Befehls: {}", e)))?;
            
            let result_table = lua_ctx.create_table()?;
            result_table.set("stdout", "")?;
            result_table.set("stderr", "")?;
            result_table.set("status", status.code().unwrap_or(-1))?;
            return Ok(LuaValue::Table(result_table));
        }
    } else {
        // Nicht warten - im Hintergrund starten
        let result_table = lua_ctx.create_table()?;
        match cmd.spawn() {
            Ok(_) => {
                result_table.set("success", true)?;
                result_table.set("message", format!("Befehl '{}' im Hintergrund gestartet", command))?;
            },
            Err(e) => {
                result_table.set("success", false)?;
                result_table.set("message", format!("Fehler beim Starten des Befehls: {}", e))?;
            }
        }
        return Ok(LuaValue::Table(result_table));
    }
})?)?;

// Fügen Sie dies zu Cargo.toml hinzu:
// [dependencies]
// clipboard = "0.5"


// Dann aktualisieren Sie die clipboard-Funktion:
globals.set("clipboard", self.lua.create_function(move |lua_ctx, _: ()| {
    // Direkter Zugriff auf die Zwischenablage über die clipboard-Crate
    let mut ctx: ClipboardContext = match ClipboardProvider::new() {
        Ok(ctx) => ctx,
        Err(e) => return Err(mlua::Error::external(format!("Fehler beim Initialisieren der Zwischenablage: {}", e)))
    };
    
    let clipboard_content = match ctx.get_contents() {
        Ok(content) => content,
        Err(e) => return Err(mlua::Error::external(format!("Fehler beim Abrufen des Clipboard-Inhalts: {}", e)))
    };
    
    // Gib den Inhalt als Lua-String zurück
    Ok(LuaValue::String(lua_ctx.create_string(&clipboard_content)?))
})?)?;
        // Tap function - unterstützt Strings und steigende Komplexität
        let script_ref = self.script.clone();
        let key_map_ref = self.key_map.clone();
        globals.set("tap", self.lua.create_function(move |_, params: LuaMultiValue| {
            let mut params_iter = params.into_iter();
            
            // Erste Parameter: Taste (String oder u8)
            let key = params_iter.next().ok_or_else(|| 
                mlua::Error::external("Mindestens eine Taste muss angegeben werden")
            )?;
            
            // Zweiter Parameter: Optional delay
            let delay = params_iter.next().map(|v| -> mlua::Result<Option<u64>> {
                match v {
                    LuaValue::Integer(i) if i >= 0 => Ok(Some(i as u64)),
                    LuaValue::Nil => Ok(None),
                    _ => Err(mlua::Error::external("Verzögerung muss eine positive Zahl sein"))
                }
            }).transpose()?.flatten();
            
            // Keycode ermitteln
            let keycode = match key {
                LuaValue::String(s) => {
                    let key_str = s.to_str().map_err(|e| mlua::Error::external(e))?;
                    *key_map_ref.get(&key_str.to_lowercase())
                        .ok_or_else(|| mlua::Error::external(format!("Unbekannte Taste: '{}'", key_str)))?
                },
                LuaValue::Integer(i) => {
                    if i < 0 || i > 255 {
                        return Err(mlua::Error::external(format!("Keycode muss zwischen 0 und 255 sein: {}", i)));
                    }
                    i as u8
                },
                _ => return Err(mlua::Error::external("Taste muss ein String oder ein Keycode (Zahl) sein")),
            };
            
            script_ref.tap(keycode, delay).map_err(|e| mlua::Error::external(e))
        })?)?;

        // Wait function - bleibt unverändert
        let script_ref = self.script.clone();
        globals.set("wait", self.lua.create_function(move |_, duration: u64| {
            script_ref.wait(duration)
                .map_err(|e| mlua::Error::external(e))
        })?)?;

        // Combo function - unterstützt jetzt Stringlisten
        let script_ref = self.script.clone();
        let key_map_ref = self.key_map.clone();
        globals.set("combo", self.lua.create_function(move |_, params: LuaMultiValue| {
            let mut params_iter = params.into_iter();
            
            // Erste Parameter: Tasten (Tabelle/Array oder einzelne Werte)
            let keys_param = params_iter.next().ok_or_else(|| 
                mlua::Error::external("Tasten müssen angegeben werden")
            )?;
            
            // Zweiter Parameter: Optional delay
            let delay = params_iter.next().map(|v| -> mlua::Result<Option<u64>> {
                match v {
                    LuaValue::Integer(i) if i >= 0 => Ok(Some(i as u64)),
                    LuaValue::Nil => Ok(None),
                    _ => Err(mlua::Error::external("Verzögerung muss eine positive Zahl sein"))
                }
            }).transpose()?.flatten();
            
            // Keycodes sammeln
            let keycodes: Vec<u8> = match keys_param {
                LuaValue::Table(t) => {
                    let mut codes = Vec::new();
                    for pair in t.pairs::<i32, LuaValue>() {
                        let (_, key_value) = pair?;
                        let code = match key_value {
                            LuaValue::String(s) => {
                                let key_str = s.to_str().map_err(|e| mlua::Error::external(e))?;
                                *key_map_ref.get(&key_str.to_lowercase())
                                    .ok_or_else(|| mlua::Error::external(format!("Unbekannte Taste: '{}'", key_str)))?
                            },
                            LuaValue::Integer(i) => {
                                if i < 0 || i > 255 {
                                    return Err(mlua::Error::external(format!("Keycode muss zwischen 0 und 255 sein: {}", i)));
                                }
                                i as u8
                            },
                            _ => return Err(mlua::Error::external("Taste in der Tabelle muss ein String oder ein Keycode (Zahl) sein")),
                        };
                        codes.push(code);
                    }
                    codes
                },
                LuaValue::String(s) => {
                    let key_str = s.to_str().map_err(|e| mlua::Error::external(e))?;
                    vec![*key_map_ref.get(&key_str.to_lowercase())
                        .ok_or_else(|| mlua::Error::external(format!("Unbekannte Taste: '{}'", key_str)))?]
                },
                LuaValue::Integer(i) => {
                    if i < 0 || i > 255 {
                        return Err(mlua::Error::external(format!("Keycode muss zwischen 0 und 255 sein: {}", i)));
                    }
                    vec![i as u8]
                },
                _ => return Err(mlua::Error::external("Tasten müssen eine Tabelle, ein String oder ein Keycode (Zahl) sein")),
            };
            
            if keycodes.is_empty() {
                return Err(mlua::Error::external("Keine gültigen Tasten angegeben"));
            }
            
            script_ref.combo(&keycodes, delay).map_err(|e| mlua::Error::external(e))
        })?)?;

    // Flush function - bleibt unverändert
        let script_ref = self.script.clone();
        globals.set("flush", self.lua.create_function(move |_, ()| {
            script_ref.flush()
                .map_err(|e| mlua::Error::external(e))
        })?)?;

        // Tastennamen als Lua-Konstanten registrieren
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