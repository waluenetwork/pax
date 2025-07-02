#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pax_chassis_tauri::{TauriChassis, TauriChassisConfig};
use tauri::{Manager, State};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


#[tauri::command]
fn handle_button_click(chassis_state: State<Arc<Mutex<TauriChassis>>>) -> Result<String, String> {
    match chassis_state.lock() {
        Ok(mut chassis) => {
            if let Err(e) = chassis.send_button_click() {
                return Err(format!("Failed to send button click to PaxEngine: {:?}", e));
            }
            
            match chassis.tick() {
                Ok(messages) => {
                    if let Err(e) = chassis.render() {
                        return Err(format!("Render error: {:?}", e));
                    }
                    Ok(format!("✅ Real Rust command executed! PaxEngine messages: {}, Canvas controlled by Pax runtime", messages.len()))
                }
                Err(e) => Err(format!("Error processing button click: {:?}", e))
            }
        }
        Err(e) => Err(format!("Failed to lock chassis: {:?}", e))
    }
}

fn main() {
    env_logger::init();
    
    let config = TauriChassisConfig::default();
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_button_click])
        .setup(|app| {
            println!("Setting up Tauri-Pax application...");
            
            let mut chassis = TauriChassis::new(config)?;
            chassis.initialize(app)?;
            
            chassis.start_performance_monitoring();
            
            let chassis_arc = Arc::new(Mutex::new(chassis));
            let chassis_clone = chassis_arc.clone();
            
            app.manage(chassis_arc);
            
            thread::spawn(move || {
                println!("Starting Pax rendering loop...");
                loop {
                    if let Ok(mut chassis) = chassis_clone.lock() {
                        match chassis.tick() {
                            Ok(messages) => {
                                if !messages.is_empty() {
                                    println!("Processed {} Pax messages", messages.len());
                                }
                                if let Err(e) = chassis.render() {
                                    eprintln!("Render error: {:?}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("Tick error: {:?}", e);
                            }
                        }
                    }
                    thread::sleep(Duration::from_millis(16));
                }
            });
            
            println!("Tauri-Pax application ready!");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
