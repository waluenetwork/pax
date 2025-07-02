#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pax_chassis_tauri::{TauriChassis, TauriChassisConfig};
use tauri::Manager;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    env_logger::init();
    
    let config = TauriChassisConfig::default();
    
    tauri::Builder::default()
        .setup(|app| {
            println!("Setting up Tauri-Pax application with Phase 2 rendering...");
            
            let mut chassis = TauriChassis::new(config)?;
            chassis.initialize(app)?;
            
            chassis.initialize_engine_for_testing()?;
            
            chassis.start_performance_monitoring();
            
            let chassis_arc = Arc::new(Mutex::new(chassis));
            let chassis_clone = chassis_arc.clone();
            
            app.manage(chassis_arc);
            
            thread::spawn(move || {
                println!("Starting Phase 2 Pax rendering loop...");
                loop {
                    if let Ok(mut chassis) = chassis_clone.lock() {
                        match chassis.tick() {
                            Ok(messages) => {
                                if !messages.is_empty() {
                                    println!("Phase 2: Processed {} Pax messages", messages.len());
                                }
                            }
                            Err(e) => {
                                eprintln!("Phase 2 rendering error: {:?}", e);
                            }
                        }
                    }
                    thread::sleep(Duration::from_millis(16)); // ~60 FPS
                }
            });
            
            println!("Phase 2: Tauri-Pax application with real component rendering ready!");
            println!("ExampleApp components: Rectangle, Text, Button with interactive features");
            println!("Rendering loop started - Pax components should now be visible!");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
