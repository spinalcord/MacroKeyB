// src-tauri/src/frontend.rs
use std::sync::{Mutex, OnceLock};
use tauri::Emitter;

#[derive(Clone, serde::Serialize)]
pub struct EventPayload {
    pub message: String,
}

static APP_HANDLE: OnceLock<Mutex<Option<tauri::AppHandle>>> = OnceLock::new();

fn get_app_handle() -> &'static Mutex<Option<tauri::AppHandle>> {
    APP_HANDLE.get_or_init(|| Mutex::new(None))
}

pub fn init(app_handle: tauri::AppHandle) {
    *get_app_handle().lock().unwrap() = Some(app_handle);
}

pub fn send_event(event_name: &str, nachricht: &str) -> Result<(), String> {
    match get_app_handle().lock().unwrap().clone() {
        Some(app_handle) => {
            let payload = EventPayload { message: nachricht.to_string() };
            // Verwende emit (nicht emit_all)
            app_handle.emit(event_name, payload)
                .map_err(|e| format!("Fehler beim Senden des Events: {}", e))
        },
        None => Err("App-Handle nicht initialisiert".to_string())
    }
}