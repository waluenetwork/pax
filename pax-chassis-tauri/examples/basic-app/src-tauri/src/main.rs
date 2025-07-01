#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pax_chassis_tauri::{setup_tauri_pax, TauriChassisConfig};

fn main() {
    env_logger::init();
    
    let config = TauriChassisConfig::default();
    
    tauri::Builder::default()
        .setup(setup_tauri_pax(config))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
