#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pax_chassis_tauri::{TauriChassis, TauriChassisConfig};
use tauri::Manager;

fn main() {
    env_logger::init();
    
    let config = TauriChassisConfig::default();
    
    tauri::Builder::default()
        .setup(|app| {
            println!("Setting up Tauri-Pax application...");
            
            let mut chassis = TauriChassis::new(config)?;
            chassis.initialize(app)?;
            
            chassis.start_performance_monitoring();
            
            app.manage(chassis);
            
            println!("Tauri-Pax application setup complete!");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
