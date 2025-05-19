// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Überprüfen, ob die Anwendung mit root-Rechten ausgeführt wird
    #[cfg(unix)]
    if !is_root() {
        eprintln!("Use: 'sudo -E npm run tauri dev' to run this application");
        std::process::exit(1);
    }

    // Rest deines Programms
    macrokeyb_lib::run();
}

#[cfg(unix)]
fn is_root() -> bool {
    use std::process::Command;
    
    let output = Command::new("id")
        .arg("-u")
        .output()
        .expect("Error 'id -u'");
    
    let uid = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u32>()
        .expect("Cannot parse UID");
    
    uid == 0
}