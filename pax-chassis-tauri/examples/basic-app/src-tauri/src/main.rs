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
            match chassis.handle_button_click() {
                Ok(_) => {
                    if let Some(app) = chassis.get_mock_example_app_state() {
                        Ok(format!("Button clicked! Count: {}, Width: {}, Color: {:?}", 
                                  app.click_count, app.rect_width, app.rect_color))
                    } else {
                        Ok("Button clicked but no ExampleApp found".to_string())
                    }
                }
                Err(e) => Err(format!("Error handling button click: {:?}", e))
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
            println!("Setting up Tauri-Pax application with Phase 3 interactive testing...");
            
            let mut chassis = TauriChassis::new(config)?;
            chassis.initialize(app)?;
            
            println!("✅ Using MockExampleApp for Phase 3 interactive testing");
            println!("Interactive functionality: Button clicks will update properties and visual state");
            
            chassis.start_performance_monitoring();
            
            let chassis_arc = Arc::new(Mutex::new(chassis));
            let chassis_clone = chassis_arc.clone();
            
            app.manage(chassis_arc);
            
            thread::spawn(move || {
                println!("Starting Phase 3 Pax rendering loop with MockExampleApp...");
                loop {
                    if let Ok(mut chassis) = chassis_clone.lock() {
                        match chassis.tick() {
                            Ok(messages) => {
                                if !messages.is_empty() {
                                    println!("Phase 3: Processed {} Pax messages", messages.len());
                                }
                            }
                            Err(e) => {
                                eprintln!("Phase 3 rendering error: {:?}", e);
                            }
                        }
                    }
                    thread::sleep(Duration::from_millis(16)); // ~60 FPS
                }
            });
            
            println!("Phase 3: Tauri-Pax application with MockExampleApp ready!");
            println!("MockExampleApp components: Rectangle, Text, Button with interactive testing");
            println!("Interactive testing enabled - button clicks should update properties!");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
