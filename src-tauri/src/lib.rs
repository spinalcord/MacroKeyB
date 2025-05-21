// main.rs
// Import necessary libraries for error handling, serialization/deserialization, file operations, etc.
use serde::{Deserialize, Serialize};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{window, Emitter, Manager};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{SystemTime};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use macroeng::KeyboardListener;
use macroeng::LuaManager;
use tokio::task;
use tokio::time;


mod frontend;
// ====== Global Status Variables ======
// AppState struct to manage the application's global state using thread-safe primitives
struct AppState {
    blocking_active: AtomicBool,          // Flag indicating if keyboard blocking is active
    detection_in_progress: AtomicBool,    // Flag indicating if keyboard detection is in progress
    cancel_detection: AtomicBool,         // Flag to signal cancellation of detection
    blocking_thread: Mutex<Option<thread::JoinHandle<()>>>, // Handle to the blocking thread
    current_device: Mutex<Option<String>>, // Currently blocked device name
    items: Mutex<Option<Vec<Item>>>,      // Collection of macro items
    app_data_dir: Mutex<Option<PathBuf>>, // Application data directory path
    assign_mode_active: AtomicBool,   
    item_waiting_for_key: Mutex<Option<String>>,
}

// Implementation of AppState with a const constructor for static initialization
impl AppState {
    const fn new() -> Self {
        Self {
            blocking_active: AtomicBool::new(false),
            detection_in_progress: AtomicBool::new(false),
            cancel_detection: AtomicBool::new(false),
            blocking_thread: Mutex::new(None),
            current_device: Mutex::new(None),
            items: Mutex::new(None),
            app_data_dir: Mutex::new(None),
                     assign_mode_active: AtomicBool::new(false),
            item_waiting_for_key: Mutex::new(None),

        }
    }
}

// Static global state accessible throughout the application
static STATE: AppState = AppState::new();

// Definition of an Item representing a keyboard macro
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    display_text: String,  // Display name of the macro
    assigned_key: String,  // Key that triggers the macro
    id: String,            // Unique identifier for the macro
    content: String,       // Lua script content to execute
    is_selected: bool,     // Flag indicating if the item is selected in UI
}


// ====== File Operation Functions ======
// Initialize the configuration directory
fn init_config_dir() -> Result<PathBuf, String> {
    // Get the user's home directory
    let home_dir = dirs::home_dir()
        .ok_or_else(|| "Could not find home directory".to_string())?;
    
    // Define config directory path
    let config_dir = home_dir.join(".config").join("macrokeyb");
    
    // Create directory if it doesn't exist
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Could not create directory: {}", e))?;
    }
    
    Ok(config_dir)
}

// Get path to the items JSON file
fn get_items_path() -> PathBuf {
    STATE.app_data_dir.lock().unwrap()
        .as_ref()
        .map(|path| path.join("items.json"))
        .unwrap_or_else(|| {
            // Fallback path if app_data_dir is not set
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("/home/a7"))
                .join("items.json")
        })
}

// Get path to the keyboard configuration JSON file
fn get_keyb_path() -> PathBuf {
    STATE.app_data_dir.lock().unwrap()
        .as_ref()
        .map(|path| path.join("keyb.json"))
        .unwrap_or_else(|| {
            // Fallback path if app_data_dir is not set
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("/home/a7"))
                .join("keyb.json")
        })
}

// Load items from the JSON file
fn load_items() -> Vec<Item> {
    let path = get_items_path();
    
    // Return empty vector if file doesn't exist
    if !path.exists() {
        return Vec::new();
    }
    
    // Read and deserialize file contents, handling errors
    match fs::read_to_string(&path) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(items) => items,
            Err(e) => {
                eprintln!("Error deserializing items: {}", e);
                Vec::new()
            }
        },
        Err(e) => {
            eprintln!("Error reading the JSON file: {}", e);
            Vec::new()
        }
    }
}

// Save items to JSON file with atomic write
fn save_items_to_file(items: &[Item]) -> Result<(), String> {
    let path = get_items_path();
    let temp_path = path.with_extension("json.tmp");
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(format!("Error creating directory: {}", e));
            }
        }
    }
    
    // Serialize items to JSON
    let json = match serde_json::to_string_pretty(items) {
        Ok(json) => json,
        Err(e) => return Err(format!("Error serializing items: {}", e)),
    };
    
    // First write to temporary file for atomic update
    if let Err(e) = fs::write(&temp_path, &json) {
        return Err(format!("Error writing temporary file: {}", e));
    }
    
    // Rename or copy and delete to ensure atomic update
    if let Err(e) = fs::rename(&temp_path, &path) {
        // If rename fails, try direct write and delete temp file
        if let Err(e2) = fs::write(&path, &json) {
            return Err(format!("Error writing file after rename failure: {}. Original error: {}", e2, e));
        }
        let _ = fs::remove_file(&temp_path); // Ignore errors on temp file deletion
    }
    
    Ok(())
}

// Initialize items at program start
fn init_items() {
    let mut items_lock = STATE.items.lock().unwrap();
    if items_lock.is_none() {
        *items_lock = Some(load_items());
    }
}

// ====== Keyboard Helper Functions ======
// Add standard key listener to handle keypresses
fn add_standard_key_listener(keyb: &KeyboardListener::Instance) {
    // Clear previous listeners
    keyb.on_key.clear_listeners();
    
    // Add new listener to process key events
    keyb.on_key.add_listener(|info| {
        match info.state {
            KeyboardListener::KeyState::Down => {println!("Key down: {} {}", info.name, info.state); handle_key(&info.name, &info.state)}, 
            KeyboardListener::KeyState::Press => (),
            KeyboardListener::KeyState::Up => {println!("Key up: {} {}", info.name, info.state); /*handle_key(&info.name, &info.state);*/}, 
        }
    });
}

// Imports for formatted timestamps

// Helper function to generate a formatted timestamp
fn get_formatted_timestamp() -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    // ISO 8601 Format: "2025-05-17T13:45:30Z"
    datetime.format("Date: %Y-%m-%d, Time: %H:%M:%S%.3fZ").to_string()
}

fn handle_key(key_name: &str, key_state: &KeyboardListener::KeyState) {
    // Only react to Key-Down events
    if let KeyboardListener::KeyState::Down = key_state {
        println!("Processing key down: {}", key_name);
        
        // Check if we are in assignment mode
        if STATE.assign_mode_active.load(Ordering::SeqCst) {
            // Process key assignment
            let item_id = {
                let mut item_id_guard = STATE.item_waiting_for_key.lock().unwrap();
                item_id_guard.take() // Take ID and set Option to None
            };
            
            if let Some(id) = item_id {
                // Assign the key to the item
                match assign_key_to_item(&id, key_name.to_string()) {
                    Ok(_) => {
                        let timestamp = get_formatted_timestamp();
                        println!("Key '{}' was assigned to item with ID '{}'", key_name, id);
                        // also send an event with a timestamp here if needed
                        if let Err(e) = frontend::send_event("key-assigned", &format!("{{\"status\":\"success\",\"itemId\":\"{}\",\"key\":\"{}\",\"timestamp\":\"{}\"}}", id, key_name, timestamp)) {
                            eprintln!("Error sending Key-Assigned event: {}", e);
                        }
                    },
                    Err(e) => {
                        println!("Error during assignment: {}", e);
                        // Reset item ID so the UI can display the error
                        *STATE.item_waiting_for_key.lock().unwrap() = Some(format!("ERROR:{}", e));
                        
                        // Send error message with timestamp
                        let timestamp = get_formatted_timestamp();
                        if let Err(send_err) = frontend::send_event("key-assign-error", &format!("{{\"status\":\"error\",\"itemId\":\"{}\",\"key\":\"{}\",\"error\":\"{}\",\"timestamp\":\"{}\"}}",
                            id, 
                            key_name, 
                            e.to_string().replace("\"", "\\\"").replace("\n", "\\n"),
                            timestamp)) {
                            eprintln!("Fehler beim Senden des Key-Assign-Error-Events: {}", send_err);
                        }
                    }
                }
                
                // Zuweisungsmodus beenden
                STATE.assign_mode_active.store(false, Ordering::SeqCst);
            } else {
                // Should not happen - we are in assignment mode without item ID
                eprintln!("Assignment mode active, but no item ID available");
                STATE.assign_mode_active.store(false, Ordering::SeqCst);
                
                // Send error message with timestamp
                let timestamp = get_formatted_timestamp();
                if let Err(e) = frontend::send_event("assign-mode-error", &format!("{{\"status\":\"error\",\"message\":\"Assignment mode active without item ID\",\"timestamp\":\"{}\"}}", timestamp)) {
                    eprintln!("Error sending Assign-Mode-Error event: {}", e);
                }
            }
            
            return; // No further processing in assignment mode
        }
        
        // Normal processing when not in assignment mode
        let script_content = {
            let items_guard = STATE.items.lock().unwrap();
            let items = match &*items_guard {
                Some(items) => items,
                None => {
                    println!("No items available");
                    
                    // Send error message with timestamp
                    let timestamp = get_formatted_timestamp();
                    if let Err(e) = frontend::send_event("items-error", &format!("{{\"status\":\"error\",\"message\":\"No items available\",\"timestamp\":\"{}\"}}", timestamp)) {
                        eprintln!("Error sending Items-Error event: {}", e);
                    }
                    return;
                }
            };
            
            if let Some(item) = items.iter().find(|item| item.assigned_key == key_name) {
                println!("Found matching item: {} with key {}", item.display_text, item.assigned_key);
                Some((item.id.clone(), item.display_text.clone(), item.content.clone()))
            } else {
                println!("No item found with key: {}", key_name);
                
                // Send error message with timestamp
                let timestamp = get_formatted_timestamp();
                if let Err(e) = frontend::send_event("key-not-found", &format!("{{\"status\":\"error\",\"key\":\"{}\",\"message\":\"No item found with this key\",\"timestamp\":\"{}\"}}", key_name, timestamp)) {
                    eprintln!("Error sending Key-Not-Found event: {}", e);
                }
                None
            }
        };
        
        // Lua-Skript ausführen und Fehler an Frontend senden
        if let Some((item_id, item_name, content)) = script_content {
            match LuaManager::new() {
                Ok(lua_script) => {
                    println!("Executing Lua script for item: {}", item_name);
                    // Pass the item name to an extended run_script method
                    match lua_script.run_script_with_name(&content, key_name) {
                        Ok(_) => {
                            println!("Lua script executed successfully");
                            // Send success message with timestamp to frontend
                            let timestamp = get_formatted_timestamp();
                            if let Err(e) = frontend::send_event("lua-execution", &format!("{{\"status\":\"success\",\"itemId\":\"{}\",\"itemName\":\"{}\",\"timestamp\":\"{}\"}}", item_id, item_name, timestamp)) {
                                eprintln!("Error sending success event: {}", e);
                            }
                        },
                        Err(e) => {
                            let error_msg = format!("Error executing Lua script: {}", e);
                            eprintln!("{}", error_msg);
                            
                            // Send error message with timestamp to frontend
                            let timestamp = get_formatted_timestamp();
                            let error_payload = format!("{{\"status\":\"error\",\"itemId\":\"{}\",\"itemName\":\"{}\",\"error\":\"{}\",\"timestamp\":\"{}\"}}",
                                                      item_id, 
                                                      item_name, 
                                                      e.to_string().replace("\"", "\\\"").replace("\n", "\\n"),
                                                      timestamp);
                            
                            if let Err(send_err) = frontend::send_event("lua-error", &error_payload) {
                                eprintln!("Error sending error event: {}", send_err);
                            }
                        }
                    }
                },
                Err(e) => {
                    let error_msg = format!("Error creating Lua manager: {}", e);
                    eprintln!("{}", error_msg);
                    
                    // Send error message when creating the Lua manager with timestamp to frontend
                    let timestamp = get_formatted_timestamp();
                    let error_payload = format!("{{\"status\":\"error\",\"itemId\":\"{}\",\"itemName\":\"{}\",\"error\":\"Lua manager initialization failed: {}\",\"timestamp\":\"{}\"}}",
                                              item_id, 
                                              item_name, 
                                              e.to_string().replace("\"", "\\\"").replace("\n", "\\n"),
                                              timestamp);
                    
                    if let Err(send_err) = frontend::send_event("lua-error", &error_payload) {
                        eprintln!("Error sending manager error event: {}", send_err);
                    }
                }
            }
        }
    }
}


fn is_key_already_assigned(key: &str, exclude_item_id: &str) -> bool {
    let items_guard = STATE.items.lock().unwrap();
    
    match &*items_guard {
        Some(items) => items.iter()
            .any(|item| item.assigned_key == key && item.id != exclude_item_id),
        None => false,
    }
}

// Helper function to assign a key to an item
fn assign_key_to_item(item_id: &str, key: String) -> Result<(), String> {
    init_items();
    
    // Check if the key is already assigned to another item
    if is_key_already_assigned(&key, item_id) {
        return Err(format!("Key '{}' is already assigned to another item", key));
    }
    
    match &mut *STATE.items.lock().unwrap() {
        Some(items) => {
            if let Some(item) = items.iter_mut().find(|item| item.id == item_id) {
                // Update the assigned key
                item.assigned_key = key;
                
                // Save the changes immediately to the file
                save_items_to_file(items).map_err(|e| format!("Error saving items: {}", e))?;
                
                Ok(())
            } else {
                Err(format!("Item with ID {} not found", item_id))
            }
        },
        None => Err("No items available".to_string()),
    }
}
// Add new Tauri commands

// Start assignment mode for a specific item
#[tauri::command]
fn start_assign_mode(id: String) -> Result<(), String> {
    println!("Starte Key-Zuweisungsmodus für Item ID: {}", id);
    
    // Check if keyboard blocking is active
    if !STATE.blocking_active.load(Ordering::SeqCst) {
        return Err("No keyboard active. Please detect an input device first.".to_string());
    }
    
    // Find and check item
    let item_exists = {
        let items_guard = STATE.items.lock().unwrap();
        match &*items_guard {
            Some(items) => items.iter().any(|item| item.id == id),
            None => false,
        }
    };
    
    if !item_exists {
        return Err(format!("Item with ID {} not found", id));
    }
    
    // Activate assignment mode
    *STATE.item_waiting_for_key.lock().unwrap() = Some(id);
    STATE.assign_mode_active.store(true, Ordering::SeqCst);
    
    println!("Assignment mode active, waiting for key press");
    Ok(())
}
// Cancel assignment mode
#[tauri::command]
fn cancel_assign_mode() -> Result<(), String> {
    println!("Breche Key-Zuweisungsmodus ab");
    
    // Deactivate assignment mode
    STATE.assign_mode_active.store(false, Ordering::SeqCst);
    *STATE.item_waiting_for_key.lock().unwrap() = None;
    
    println!("Assignment mode deactivated");
    Ok(())
}

// Get assignment mode status
#[tauri::command]
fn get_assign_mode_status() -> (bool, Option<String>) {
    let is_active = STATE.assign_mode_active.load(Ordering::SeqCst);
    let item_id = STATE.item_waiting_for_key.lock().unwrap().clone();
    
    (is_active, item_id)
}

// Start blocking a device
fn start_blocking_device(device: &KeyboardListener::DeviceInfo) -> Result<(), String> {
    // Save device information
    let device_name = device.device_name.clone();
    *STATE.current_device.lock().unwrap() = Some(device_name.clone());
    
    // Set flag for active blocking
    STATE.blocking_active.store(true, Ordering::SeqCst);
    
    // Clone device for thread
    let dev_clone = device.clone();
    let thread_device_name = device_name.clone(); // Extra clone for thread
    
    // Create blocking thread
    let handle = thread::spawn(move || {
        println!("Starting blocking for device: {}", thread_device_name);
        
        // Check for immediate termination
        if !STATE.blocking_active.load(Ordering::SeqCst) {
            println!("Blocking cancelled before it started for device: {}", thread_device_name);
            return;
        }
        
        let keyb = KeyboardListener::Instance::new();
        if let Err(e) = keyb.block_input_device(&dev_clone) {
            eprintln!("Error blocking device: {}", e);
        }
        
        // Reached when blocking stops
        println!("Blocking thread for {} has ended", thread_device_name);
        STATE.blocking_active.store(false, Ordering::SeqCst);
    });
    
    // Save thread handle
    *STATE.blocking_thread.lock().unwrap() = Some(handle);
    
    println!("Successfully started blocking keyboard: {}", device_name);
    Ok(())
}

// ====== Tauri Command Functions ======
// Load previously detected keyboard from config
#[tauri::command]
fn load_emited_keyboard() -> Result<String, String> {
    println!("Loading previously detected keyboard from JSON...");
    
    // Check if detection is already in progress
    if STATE.detection_in_progress.load(Ordering::SeqCst) {
        return Err("Detection already in progress".to_string());
    }
    
    // First release blocked devices
    if STATE.blocking_active.load(Ordering::SeqCst) {
        if let Err(e) = release_blocked_devices() {
            eprintln!("Error releasing devices: {}", e);
        } else {
            println!("Successfully released previously blocked devices");
        }
    }
    
    // Reset cancellation flag
    STATE.cancel_detection.store(false, Ordering::SeqCst);
    
    let keyb = KeyboardListener::Instance::new();
    let keyb_path = get_keyb_path();
    
    // Load devices from JSON
    match keyb.load_devices_from_json(keyb_path.to_str().unwrap_or("/home/a7/keyb.json")) {
        Ok(devices) => {
            if devices.is_empty() {
                return Err("No saved devices found in config file".to_string());
            }
            
            // Use first device
            let dev = &devices[0];
            println!("Successfully loaded keyboard: {}", dev.device_name);
            
            // Add standard key listener
            add_standard_key_listener(&keyb);
            
            // Start blocking
            match start_blocking_device(dev) {
                Ok(_) => Ok(format!("{} (Blocking active)", dev.device_name)),
                Err(e) => Err(format!("Error starting blocking: {}", e)),
            }
        },
        Err(e) => Err(format!("Error loading device config: {}", e))
    }
}

// Release all blocked devices
#[tauri::command]
fn release_blocked_devices() -> Result<(), String> {
    println!("Releasing all blocked devices...");
    
    // Check if blocking is active
    if !STATE.blocking_active.load(Ordering::SeqCst) {
        println!("No active blocking to release");
        return Ok(());
    }
    
    // Set flag that blocking is no longer active (BEFORE stopping)
    STATE.blocking_active.store(false, Ordering::SeqCst);
    
    // Stop monitor
    let keyb = KeyboardListener::Instance::new();
    keyb.stop_monitor_now();
    
    // Extract blocking thread handle
    let thread_handle = {
        let mut lock = STATE.blocking_thread.lock().unwrap();
        lock.take()
    };
    
    // Wait for blocking thread to terminate
    if let Some(handle) = thread_handle {
        match handle.join() {
            Ok(_) => println!("Blocking thread joined successfully"),
            Err(_) => eprintln!("Error joining blocking thread"),
        }
    }
    
    // Clear current device
    *STATE.current_device.lock().unwrap() = None;
    
    println!("All devices released successfully");
    Ok(())
}

// Wait for and detect keyboard input
#[tauri::command]
async fn wait_for_keypress() -> Result<String, String> {
    println!("Starting keyboard detection...");
    
    // Check if detection is already in progress
    if STATE.detection_in_progress.load(Ordering::SeqCst) {
        return Err("Detection already in progress".to_string());
    }
    
    // First release blocked devices
    if STATE.blocking_active.load(Ordering::SeqCst) {
        if let Err(e) = release_blocked_devices() {
            eprintln!("Error releasing devices: {}", e);
        } else {
            println!("Successfully released previously blocked devices");
        }
    }
    
    // Reset cancellation flag and set detection flag
    STATE.cancel_detection.store(false, Ordering::SeqCst);
    STATE.detection_in_progress.store(true, Ordering::SeqCst);
    
    // Get path to keyboard config file
    let keyb_path = get_keyb_path();
    let keyb_path_str = keyb_path.to_str().unwrap_or("/home/a7/keyb.json").to_string();
    
    // Perform device detection in separate thread
    let handle = task::spawn_blocking(move || {
        let keyb = KeyboardListener::Instance::new();
        let device_list = keyb.get_device_list();
        
        // Check for cancellation
        if STATE.cancel_detection.load(Ordering::SeqCst) {
            return Err("Detection cancelled".to_string());
        }
        
        // Start auto-detection
        match keyb.auto_detect_input_device(device_list.as_slice()) {
            Ok(opt) => {
                // Check for cancellation again after detection
                if STATE.cancel_detection.load(Ordering::SeqCst) {
                    return Ok("Detection cancelled".to_string());
                }
                
                if let Some(dev) = opt {
                    // Detection successful
                    println!("Successfully detected keyboard: {}", dev.device_name);
                    
                    // Add standard key listener
                    add_standard_key_listener(&keyb);
                    
                    // Save device information to file
                    if let Err(e) = keyb.save_devices_to_json(&[dev.clone()], &keyb_path_str) {
                        eprintln!("Error saving device config: {}", e);
                    }
                    
                    // Start blocking
                    match start_blocking_device(&dev) {
                        Ok(_) => Ok(format!("{} (Blocking active)", dev.device_name)),
                        Err(e) => Ok(format!("Error starting blocking: {}", e)),
                    }
                } else {
                    Ok("No input detected".into())
                }
            },
            Err(e) => Err(e.to_string()),
        }
    });

    // Add timeout
    let result = match time::timeout(time::Duration::from_secs(180), handle).await {
        Ok(result) => {
            match result {
                Ok(device_result) => device_result,
                Err(e) => Err(format!("Error in detection task: {}", e)),
            }
        },
        Err(_) => {
            // Handle timeout
            STATE.cancel_detection.store(true, Ordering::SeqCst);
            Err("Timeout during detection".to_string())
        }
    };
    
    // Detection is complete
    STATE.detection_in_progress.store(false, Ordering::SeqCst);
    
    result
}

// Cancel keyboard detection process
#[tauri::command]
async fn cancel_keypress_detection() -> Result<(), String> {
    println!("Cancelling keyboard detection and blocking");
    
    // Set cancellation flag
    STATE.cancel_detection.store(true, Ordering::SeqCst);
    
    // Release blocking if active
    if STATE.blocking_active.load(Ordering::SeqCst) {
        println!("blocking was active");
        STATE.blocking_active.store(false, Ordering::SeqCst);
        
        // Stop monitor
        let keyb = KeyboardListener::Instance::new();
        keyb.stop_monitor_now();
    }
    
    // Wait briefly for threads to respond to cancellation flag
    time::sleep(time::Duration::from_millis(200)).await;
    
    // Clear detection flag
    STATE.detection_in_progress.store(false, Ordering::SeqCst);
    
    // Clear current device
    *STATE.current_device.lock().unwrap() = None;
    
    // Clean up thread handle without waiting
    let _ = STATE.blocking_thread.lock().unwrap().take();

    println!("Keyboard detection and blocking cancelled successfully");
    
    // Release devices and load new keyboard status (ignore errors)
    let _ = release_blocked_devices();
    let _ = load_emited_keyboard();
    
    Ok(())
}

// ====== Item Management Functions ======
// Select an item by ID
#[tauri::command]
fn select_item(id: String) -> Result<(), String> {
    init_items();
    
    match &mut *STATE.items.lock().unwrap() {
        Some(items) => {
            // Set all items to not selected except the one with matching ID
            for item in items.iter_mut() {
                item.is_selected = item.id == id;
            }
            Ok(())
        },
        None => Err("No items available".to_string()),
    }
}

// Get list of all items
#[tauri::command]
fn get_list() -> Vec<(String, String, String, String, bool)> {
    init_items();
    
    match &*STATE.items.lock().unwrap() {
        Some(items) => items
            .iter()
            .map(|item| (
                item.display_text.clone(),
                item.assigned_key.clone(),
                item.id.clone(),
                item.content.clone(),
                item.is_selected,
            ))
            .collect(),
        None => Vec::new(),
    }
}

// Add a new item
#[tauri::command]
fn add_item() -> (String, String, String, String, bool) {
    init_items();
    
    // Generate unique UUID
    let unique_id = Uuid::new_v4().to_string();
    
    let new_item = Item {
        display_text: "New Item".to_string(),
        assigned_key: "new".to_string(),
        id: unique_id.clone(),
        content: "".to_string(),
        is_selected: false,
    };
    
    let result = (
        new_item.display_text.clone(),
        new_item.assigned_key.clone(),
        new_item.id.clone(),
        new_item.content.clone(),
        new_item.is_selected,
    );
    
    let mut items_lock = STATE.items.lock().unwrap();
    match items_lock.as_mut() {
        Some(items) => {
            items.push(new_item);
        },
        None => {
            let mut items = Vec::new();
            items.push(new_item);
            *items_lock = Some(items);
        }
    }
    
    result
}

// Rename an item
#[tauri::command]
fn rename_item(id: String, new_name: String) -> Result<(), String> {
    init_items();
    
    match &mut *STATE.items.lock().unwrap() {
        Some(items) => {
            if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                item.display_text = new_name;
                Ok(())
            } else {
                Err("Item with the specified ID not found".to_string())
            }
        },
        None => Err("No items available".to_string()),
    }
}

// Delete an item
#[tauri::command]
fn delete_item(id: String) -> Result<(), String> {
    init_items();
    
    match &mut *STATE.items.lock().unwrap() {
        Some(items) => {
            let initial_len = items.len();
            items.retain(|item| item.id != id);
            
            if items.len() < initial_len {
                Ok(())
            } else {
                Err("Item with the specified ID not found".to_string())
            }
        },
        None => Err("No items available".to_string()),
    }
}

// Save all items (from UI)
#[tauri::command]
fn save_items(items: Vec<(String, String, String, String, bool)>) -> Result<(), String> {
    // Convert back to Item structures
    let items: Vec<Item> = items
        .into_iter()
        .map(|(display_text, assigned_key, id, content, is_selected)| Item {
            display_text,
            assigned_key,
            id,
            content,
            is_selected,
        })
        .collect();
    
    // Save to static variable
    *STATE.items.lock().unwrap() = Some(items.clone());
    
    // Save to file
    save_items_to_file(&items)
}

// Update content of an item
#[tauri::command]
fn update_item_content(id: String, content: String) -> Result<(), String> {
    init_items();
    
    match &mut *STATE.items.lock().unwrap() {
        Some(items) => {
            if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                item.content = content;
                Ok(())
            } else {
                Err("Item with the specified ID not found".to_string())
            }
        },
        None => Err("No items available".to_string()),
    }
}

// Get current status of keyboard locks
#[tauri::command]
fn get_keyboard_status() -> (bool, bool, Option<String>) {
    let is_blocking = STATE.blocking_active.load(Ordering::SeqCst);
    let is_detecting = STATE.detection_in_progress.load(Ordering::SeqCst);
    let device = STATE.current_device.lock().unwrap().clone();
    
    (is_blocking, is_detecting, device)
}



// Initialize and run the application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Ensure no blocking or detection is active at startup
    STATE.blocking_active.store(false, Ordering::SeqCst);
    STATE.detection_in_progress.store(false, Ordering::SeqCst);
    STATE.cancel_detection.store(false, Ordering::SeqCst);
    
    // Set application data directory
    match init_config_dir() {
        Ok(config_dir) => {
            *STATE.app_data_dir.lock().unwrap() = Some(config_dir);
            println!("Configuration directory: {:?}", STATE.app_data_dir.lock().unwrap());
        },
        Err(e) => {
            eprintln!("Error initializing configuration directory: {}", e);
            *STATE.app_data_dir.lock().unwrap() = std::env::current_dir().ok()
                .or_else(|| Some(PathBuf::from("/home/a7")));
        }
    }
    
    // Initialize Tauri application with command handlers
    tauri::Builder::default()
     .setup(|app| {
            // Event-Manager initialisieren
            frontend::init(app.handle().clone());


            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_list,
            add_item,
            rename_item,
            delete_item,
            save_items,
            update_item_content,
            select_item,
            wait_for_keypress,
            cancel_keypress_detection,
            load_emited_keyboard,
            release_blocked_devices,
            get_keyboard_status,
            start_assign_mode,
            cancel_assign_mode,
            get_assign_mode_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}