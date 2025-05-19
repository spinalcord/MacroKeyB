use std::fs::OpenOptions;
use std::fmt::{Debug, Display};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::os::unix::io::AsRawFd;
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use serde_json;
use std::sync::atomic::{AtomicBool, Ordering};
use ctrlc;

pub use crate::event_handler;

// Structure for Linux input events
#[repr(C)]
struct InputEvent {
    tv_sec: libc::time_t,
    tv_usec: libc::suseconds_t,
    type_: u16,
    code: u16,
    value: i32,
}

// Constants for event types
const EV_KEY: u16 = 1;
const KEY_RELEASE: i32 = 0;
const KEY_PRESS: i32 = 1;
const KEY_REPEAT: i32 = 2;
const INPUT_DEVICES_PATH: &str = "/dev/input";

// Structure for device information
#[derive(Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub event_num: usize,
    pub device_path: String,
    pub device_name: String,
    pub usb_port: String,
}

impl Debug for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceInfo")
         .field("event_num", &self.event_num)
         .field("device_path", &self.device_path)
         .field("device_name", &self.device_name)
         .field("usb_port", &self.usb_port)
         .finish()
    }
}

// Global status variables
static MONITOR_RUNNING: AtomicBool = AtomicBool::new(false);
static GRABBED_FDS: Mutex<Vec<i32>> = Mutex::new(Vec::new());

// Reusable event data for better performance
struct EventReader {
    reader: BufReader<File>,
    buffer: [u8; std::mem::size_of::<InputEvent>()],
}

impl EventReader {
    fn new(file: File) -> io::Result<Self> {
        // Set non-blocking mode
        let fd = file.as_raw_fd();
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFL, 0) };
        unsafe { 
            if libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) < 0 {
                return Err(io::Error::last_os_error());
            }
        }
        
        Ok(EventReader {
            reader: BufReader::new(file),
            buffer: [0u8; std::mem::size_of::<InputEvent>()],
        })
    }
    
    fn read_event(&mut self) -> io::Result<Option<InputEvent>> {
        match self.reader.read_exact(&mut self.buffer) {
            Ok(_) => {
                let event = unsafe { std::ptr::read(self.buffer.as_ptr() as *const InputEvent) };
                Ok(Some(event))
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e)
        }
    }
}

// Key handling
pub enum KeyState {
    Down,
    Press,
    Up,
}

impl Display for KeyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyState::Down => write!(f, "[Key Down]"),
            KeyState::Press => write!(f, "[Key Press]"),
            KeyState::Up => write!(f, "[Key Up]"),
        }
    }
}

pub struct KeyInfo {
    pub name: String,
    pub state: KeyState,
}

// Helper function to convert key codes to readable text
fn key_to_string(code: u16) -> String {
    match code {
        1 => "ESC".into(),
        2..=11 => (code - 1).to_string(),
        12 => "0".into(),
        13 => "-".into(),
        14 => "BACKSPACE".into(),
        15 => "TAB".into(),
        16..=25 => ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"][code as usize - 16].into(),
        26 => "[".into(),
        27 => "]".into(),
        28 => "ENTER".into(),
        29 => "CTRL".into(),
        30..=38 => ["A", "S", "D", "F", "G", "H", "J", "K", "L"][code as usize - 30].into(),
        39 => ";".into(),
        40 => "'".into(),
        41 => "`".into(),
        42 => "LSHIFT".into(),
        43 => "\\".into(),
        44..=53 => ["Z", "X", "C", "V", "B", "N", "M", ",", ".", "/"][code as usize - 44].into(),
        54 => "RSHIFT".into(),
        55 => "NUM*".into(),
        56 => "ALT".into(),
        57 => "SPACE".into(),
        58 => "CAPSLOCK".into(),
        59..=68 => format!("F{}", code - 58),
        87 => "F11".into(),
        88 => "F12".into(),
        96 => "ENTER".into(),
        97 => "CTRL".into(),
        98 => "NUM/".into(),
        99 => "PrtSc".into(),
        100 => "ALT".into(),
        102 => "HOME".into(),
        103 => "UP".into(),
        104 => "PGUP".into(),
        105 => "LEFT".into(),
        106 => "RIGHT".into(),
        107 => "END".into(),
        108 => "DOWN".into(),
        109 => "PGDOWN".into(),
        110 => "INS".into(),
        111 => "DEL".into(),
        125 => "WIN".into(),
        272 => "MOUSE-LEFT".into(),
        273 => "MOUSE-RIGHT".into(),
        274 => "MOUSE-MIDDLE".into(),
        _ => format!("KEY_{}", code),
    }
}

// Main instance for keyboard monitoring
pub struct Instance {
    pub on_key: Arc<event_handler::EventHandler<KeyInfo>>,
}

impl Instance {
    pub fn new() -> &'static Self {
        static INSTANCE: OnceLock<Instance> = OnceLock::new();
        INSTANCE.get_or_init(|| { 
            Self::admit_sudo();
            Instance { 
                on_key: Arc::new(event_handler::EventHandler::new())
            }
        });
        INSTANCE.get().unwrap()
    }

    fn admit_sudo() {
        let has_root_access = OpenOptions::new()
            .read(true)
            .write(false)
            .open("/dev/input/event0")
            .is_ok();
    
        if !has_root_access {
            panic!("Keyboard Listener needs root rights to work correctly");
        }
    }
    
    // Device management functions
    pub fn get_device_list(&self) -> Vec<DeviceInfo> {
        let mut device_list = Vec::with_capacity(32);
        
        if let Ok(entries) = fs::read_dir(INPUT_DEVICES_PATH) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    let file_name = file_name.to_string_lossy();
                    
                    if file_name.starts_with("event") {
                        if let Ok(event_num) = file_name.trim_start_matches("event").parse::<usize>() {
                            if let Some(device_info) = self.get_device_info(&path, event_num) {
                                if device_info.usb_port.to_lowercase().contains("usb") { 
                                    device_list.push(device_info);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        device_list.sort_by_key(|d| d.event_num);
        device_list
    }
    
    fn get_device_info(&self, path: &Path, event_num: usize) -> Option<DeviceInfo> {
        let device_path = path.to_string_lossy().into_owned();
        let sysfs_path = PathBuf::from(format!("/sys/class/input/event{}", event_num));
        
        // Get device name
        let device_name = fs::read_to_string(sysfs_path.join("device/name"))
            .map(|name| name.trim().to_owned())
            .unwrap_or_else(|_| String::from("Unknown Device"));
        
        // Get USB port info
        let usb_port = self.extract_usb_info(event_num, &sysfs_path);
        
        Some(DeviceInfo {
            event_num,
            device_path,
            device_name,
            usb_port,
        })
    }
    
    fn extract_usb_info(&self, event_num: usize, sysfs_path: &Path) -> String {
        // Try to get USB path from device link
        let device_path_link = sysfs_path.join("device");
        
        // First try: from device link
        if let Ok(dev_path) = fs::read_link(&device_path_link) {
            let path_str = dev_path.to_string_lossy();
            if path_str.contains("usb") {
                return self.extract_usb_path(&path_str);
            }
        }
        
        // Second try: from uevent
        if let Ok(file) = File::open(sysfs_path.join("device/uevent")) {
            let reader = BufReader::new(file);
            for line in reader.lines().filter_map(Result::ok) {
                if line.starts_with("PHYSDEVPATH=") && line.contains("usb") {
                    return self.extract_usb_path(&line[12..]);
                }
            }
        }
        
        // Third try: from phys file
        if let Ok(phys_content) = fs::read_to_string(sysfs_path.join("device/phys")) {
            let usb_port = phys_content.trim().to_owned();
            if !usb_port.is_empty() {
                return usb_port;
            }
        }
        
        // Last try: from input ID
        let input_id = self.get_input_id(event_num);
        if !input_id.is_empty() {
            return input_id;
        }
        
        "unknown".to_string()
    }
    
    fn extract_usb_path(&self, path: &str) -> String {
        // Extract relevant USB path parts
        for part in path.split('/') {
            if part.contains("usb-") || (part.contains(':') && part.contains('.')) {
                return part.to_string();
            }
        }
        
        // If multiple USB parts found, try to combine them
        let parts: Vec<&str> = path.split('/').collect();
        for (i, part) in parts.iter().enumerate() {
            if part.starts_with("usb") && i + 1 < parts.len() && parts[i+1].contains('-') {
                return format!("{}/{}", part, parts[i+1]);
            }
        }
        
        "unknown".to_string()
    }
    
    fn get_input_id(&self, event_num: usize) -> String {
        let path = format!("/sys/class/input/event{}/device/id/input", event_num);
        
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                // Save the OsString value and then convert it
                let filename = entry.file_name();
                let input_id = filename.to_string_lossy();
                if input_id.starts_with("input") {
                    return input_id.into_owned();
                }
            }
        }
        
        String::new()
    }
    
    // Device selection functions
    pub fn select_devices_manually(&self, device_list: &[DeviceInfo]) -> io::Result<Vec<DeviceInfo>> {
        let mut selected_devices = Vec::new();
        
        loop {
            println!("\nEnter the numbers of devices to monitor (comma-separated, e.g. '1,3,5'), or 'all' for all devices:");
            println!("Enter 'q' to quit.");
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            if input.to_lowercase() == "q" {
                break;
            }
            
            if input.to_lowercase() == "all" {
                selected_devices = device_list.to_vec();
                break;
            }
            
            for num_str in input.split(',') {
                if let Ok(num) = num_str.trim().parse::<usize>() {
                    if num > 0 && num <= device_list.len() {
                        selected_devices.push(device_list[num-1].clone());
                    }
                }
            }
            
            if !selected_devices.is_empty() {
                break;
            }
            
            println!("No valid devices selected. Please try again.");
        }
        
        Ok(selected_devices)
    }
    
pub fn auto_detect_input_device(&self, device_list: &[DeviceInfo]) -> io::Result<Option<DeviceInfo>> {
    // Filter input devices if possible
    let input_devices: Vec<DeviceInfo> = device_list.iter()
        .filter(|d| d.device_name.to_lowercase().contains("keyboard"))
        .cloned()
        .collect();

    let devices_to_check = if input_devices.is_empty() {
        device_list.to_vec()
    } else {
        input_devices
    }; 

    // Flag for cancellation
    let cancel_flag = Arc::new(AtomicBool::new(false));
    
    println!("\nAutomatic device detection started.");
    println!("Press any key on the desired input device...");
    println!("(Press Ctrl+C to cancel)");

    // Communication channel for device detection
    let (tx, rx) = mpsc::channel();
    
    // whether a Ctrl+C handler has already been registered
    static CTRLC_HANDLER_INITIALIZED: AtomicBool = AtomicBool::new(false);
    
    // Register the Ctrl+C handler only if necessary
    if !CTRLC_HANDLER_INITIALIZED.swap(true, Ordering::SeqCst) {
        let cancel_flag_clone = cancel_flag.clone();
        
        // Wir versuchen den Handler zu setzen, ignorieren aber Fehler
        let _ = ctrlc::set_handler(move || {
            cancel_flag_clone.store(true, Ordering::SeqCst);
        });
    } else {
        // If we are here, the handler is already set up
        // We don't need to do anything
    }

    let mut handles = Vec::new();

    // Start threads for each device to be checked
    for device in &devices_to_check {
        let device_clone = device.clone();
        let tx_clone = tx.clone();
        let cancel_flag_clone = cancel_flag.clone();

        let handle = thread::spawn(move || {
            let path = &device_clone.device_path;

            // Try to open the device
            let file = match File::open(path) {
                Ok(f) => f,
                Err(_) => return Ok(()),
            };

            // Set Non-Blocking mode
            let fd = file.as_raw_fd();
            let flags = unsafe { libc::fcntl(fd, libc::F_GETFL, 0) };
            if unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0 {
                return Err(io::Error::last_os_error());
            }

            // Set up Event Reader
            let mut reader = BufReader::new(file);
            let mut buffer = [0u8; std::mem::size_of::<InputEvent>()];
            
            // Debounce setting for key presses
            let debounce_time = Duration::from_millis(200);
            let mut last_event_time = std::time::Instant::now();

            // Main loop for detecting keyboard input
            loop {
                // Check if cancellation was requested
                if cancel_flag_clone.load(Ordering::SeqCst) {
                    return Ok(());
                }

                // Short pause to reduce CPU usage
                thread::sleep(Duration::from_millis(15));

                // Try to read an event
                match reader.read_exact(&mut buffer) {
                    Ok(_) => {
                        let event = unsafe { std::ptr::read(buffer.as_ptr() as *const InputEvent) };

                        // Only consider key presses
                        if event.type_ == EV_KEY && event.value == KEY_PRESS {
                            let now = std::time::Instant::now();
                            // Check for debounce time
                            if now.duration_since(last_event_time) >= debounce_time {
                                // Found! Send the device over the channel
                                let _ = tx_clone.send(device_clone.clone());
                                return Ok(());
                            }
                            last_event_time = now;
                        }
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {},
                    Err(e) => return Err(e),
                }
            }
        });

        handles.push(handle);
    }

    // Timeout for detection
    let timeout = Duration::from_secs(30);
    let start = std::time::Instant::now();
    let mut detected_device = None;

    // Warte auf Ergebnis oder Timeout
    loop {
        // Check for cancellation
        if cancel_flag.load(Ordering::SeqCst) {
            println!("Detection canceled.");
            break;
        }

        // Check if a device was detected
        if let Ok(device_info) = rx.try_recv() {
            detected_device = Some(device_info);
            break;
        }

        // Check for timeout
        if start.elapsed() > timeout {
            println!("Timeout: No input detected.");
            break;
        }

        // Short pause
        thread::sleep(Duration::from_millis(50));
    }

    // Clean up
    drop(handles);
    thread::sleep(Duration::from_millis(500));
    
    // Empty the channel
    while rx.try_recv().is_ok() {}

    // Gib erkanntes Gerät zurück
    Ok(detected_device)
}

    pub fn detect_input_from_device(&self, device: &DeviceInfo, tx: mpsc::Sender<DeviceInfo>) -> io::Result<()> {
        // Try to open device
        let file = match File::open(&device.device_path) {
            Ok(f) => f,
            Err(_) => return Ok(()), // Skip unavailable devices
        };
        
        // Create event reader
        let mut event_reader = EventReader::new(file)?;
        let debounce_time = Duration::from_millis(200);
        let last_event_time = std::time::Instant::now();
        
        loop {
            thread::sleep(Duration::from_millis(15));
            
            match event_reader.read_event() {
                Ok(Some(event)) if event.type_ == EV_KEY && event.value == KEY_PRESS => {
                    let now = std::time::Instant::now();
                    if now.duration_since(last_event_time) >= debounce_time {
                        let _ = tx.send(device.clone());
                        return Ok(());
                    }
                },
                Ok(_) => {}, // Skip other events
                Err(e) if e.kind() != io::ErrorKind::WouldBlock => return Err(e),
                _ => {}
            }
        }
    }
    
    // Device blocking and monitoring
    pub fn block_input_device(&self, device: &DeviceInfo) -> io::Result<()> {
        println!("\nInputs from {} are being blocked and only displayed in the console.", device.device_name);
        println!("Press Ctrl+C to exit.\n");
        
        // Open device
        let file = File::open(&device.device_path)?;
        let file_fd = file.as_raw_fd();
        
        // Grab device (requires root)
        let eviocgrab = 1074021776; // EVIOCGRAB constant
        let mut grab_success = true;
        
        // Wait for pending events to process
        thread::sleep(Duration::from_millis(300));
        
        // Clear pending events
        let mut event_reader = EventReader::new(file)?;
        while let Ok(Some(_)) = event_reader.read_event() {}
        
        // Try to grab device
        unsafe {
            if libc::ioctl(file_fd, eviocgrab, 1) < 0 {
                eprintln!("Warning: Could not completely block device input (EVIOCGRAB failed).");
                eprintln!("         This only works with root privileges.");
                grab_success = false;
            } else {
                // Add file descriptor to grabbed list
                if let Ok(mut fds) = GRABBED_FDS.lock() {
                    fds.push(file_fd);
                }
            }
        }
        
        // Set monitoring status
        MONITOR_RUNNING.store(true, Ordering::SeqCst);
        
        // Create channel for event communication
        let (tx, rx) = mpsc::channel();
        let on_key_down = Arc::clone(&self.on_key);
        
        // Shared map for key status
        let blocked_keys = Arc::new(Mutex::new(HashMap::<u16, bool>::with_capacity(128)));
        let blocked_keys_clone = Arc::clone(&blocked_keys);
        
        // Start thread to read input (without self-reference)
        let event_reader_mtx = Arc::new(Mutex::new(event_reader));
        let reader_clone = Arc::clone(&event_reader_mtx);
        
        thread::spawn(move || {
            loop {
                if !MONITOR_RUNNING.load(Ordering::SeqCst) {
                    break;
                }
                
                thread::sleep(Duration::from_millis(10));
                
                let event_result = {
                    let mut reader = reader_clone.lock().unwrap();
                    reader.read_event()
                };
                
                match event_result {
                    Ok(Some(event)) if event.type_ == EV_KEY => {
                        // Update key status
                        if let Ok(mut keys) = blocked_keys_clone.lock() {
                            keys.insert(event.code, event.value != 0);
                        }
                        
                        // Send event to main thread
                        let _ = tx.send((event.code, event.value));
                    },
                    Ok(_) => {}, // Skip other events
                    Err(e) if e.kind() != io::ErrorKind::WouldBlock => {
                        eprintln!("Error reading from device: {}", e);
                        break;
                    },
                    _ => {}
                }
            }
        });
        
        println!("Blocking active! Keyboard inputs are now being intercepted and displayed in the console.\n");
        
        // Read events from channel
        loop {
            if !MONITOR_RUNNING.load(Ordering::SeqCst) {
                break;
            }
            
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok((key_code, state)) => {
                    let key_state = match state {
                        0 => KeyState::Up,
                        1 => KeyState::Down,
                        2 => KeyState::Press,
                        _ => KeyState::Press,
                    };
            
                    let key_info = KeyInfo {
                        name: key_to_string(key_code), 
                        state: key_state
                    };
                    
                    on_key_down.trigger(&key_info);
                },
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
        
        // Release device when done
        if grab_success {
            unsafe {
                let _ = libc::ioctl(file_fd, eviocgrab, 0);
                
                if let Ok(mut fds) = GRABBED_FDS.lock() {
                    fds.retain(|&fd| fd != file_fd);
                }
            }
        }
        
        Ok(())
    }
    
    pub fn stop_monitor_now(&self) {
        println!("Stopping monitoring and releasing all blocked devices...");
        
        // Signal monitoring to stop
        MONITOR_RUNNING.store(false, Ordering::SeqCst);
        
        // Release all grabbed devices
        let eviocgrab = 1074021776; // EVIOCGRAB constant
        
        if let Ok(mut fds) = GRABBED_FDS.lock() {
            for &fd in fds.iter() {
                unsafe {
                    let result = libc::ioctl(fd, eviocgrab, 0);
                    if result >= 0 {
                        println!("Device with fd {} released", fd);
                    } else {
                        eprintln!("Error releasing device with fd {}: {}", 
                                  fd, io::Error::last_os_error());
                    }
                }
            }
            
            fds.clear();
        }
        
        thread::sleep(Duration::from_millis(300));
    }
    
    // File operations
    pub fn save_devices_to_json(&self, devices: &[DeviceInfo], file_path: &str) -> io::Result<()> {
        println!("Saving {} device(s) to {}", devices.len(), file_path);
        
        let json_str = serde_json::to_string_pretty(devices)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        let mut file = File::create(file_path)?;
        file.write_all(json_str.as_bytes())?;
        
        println!("Devices successfully saved!");
        Ok(())
    }
    
    pub fn load_devices_from_json(&self, file_path: &str) -> io::Result<Vec<DeviceInfo>> {
        println!("Loading devices from {}", file_path);
        
        if !Path::new(file_path).exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File {} not found", file_path)
            ));
        }
        
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let devices: Vec<DeviceInfo> = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        println!("{} device(s) successfully loaded!", devices.len());
        Ok(devices)
    }
    
    pub fn display_saved_devices(&self, devices: &[DeviceInfo]) {
        println!("\nSaved input devices:");
        
        for (i, device) in devices.iter().enumerate() {
            println!("[{}] {} - {} (USB Port: {})",
                i + 1,
                device.device_path,
                device.device_name,
                device.usb_port
            );
        }
    }
}