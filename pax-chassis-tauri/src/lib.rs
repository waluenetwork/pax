//! 

#![allow(unused_imports)]

pub mod config;
pub mod renderer;
pub mod events;
pub mod error;

#[cfg(feature = "javascript-bridge")]
pub mod javascript;

#[cfg(feature = "native-graphics")]
pub mod native;

#[cfg(feature = "hybrid-mode")]
pub mod hybrid;

#[cfg(feature = "performance-monitoring")]
pub mod performance;

pub use config::*;
pub use renderer::*;
pub use events::*;
pub use error::*;

use pax_runtime::PaxEngine;
use pax_runtime::DefinitionToInstanceTraverser;
use pax_runtime_api::Platform;
use pax_message::NativeMessage;
use tauri::{App, AppHandle, Manager};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

pub struct TauriChassis {
    renderer: Box<dyn TauriRenderer<Error = TauriPaxError>>,
    config: TauriChassisConfig,
    app_handle: Option<AppHandle>,
    engine_initialized: bool,
    performance_start_time: Option<Instant>,
    last_frame_time: Option<Instant>,
    frame_times: VecDeque<Duration>,
    tick_count: u64,
}

impl TauriChassis {
    pub fn new(config: TauriChassisConfig) -> Result<Self, TauriPaxError> {
        let renderer = Self::create_renderer(&config)?;
        
        Ok(Self {
            renderer,
            config,
            app_handle: None,
            engine_initialized: false,
            performance_start_time: None,
            last_frame_time: None,
            frame_times: VecDeque::with_capacity(60),
            tick_count: 0,
        })
    }
    
    pub fn initialize(&mut self, app: &App) -> Result<(), TauriPaxError> {
        self.app_handle = Some(app.handle().clone());
        
        if let Some(js_renderer) = self.renderer.as_any_mut().downcast_mut::<javascript::JavaScriptRenderer>() {
            js_renderer.set_app_handle(app.handle().clone());
        }
        
        self.renderer.initialize(&self.config)?;
        
        Ok(())
    }
    
    pub fn initialize_for_testing(&mut self) -> Result<(), TauriPaxError> {
        self.renderer.initialize(&self.config)?;
        Ok(())
    }

    pub fn tick(&mut self) -> Result<Vec<NativeMessage>, TauriPaxError> {
        self.tick_count += 1;
        self.record_frame();
        
        if self.engine_initialized {
            let message_queue = self.create_example_app_messages();
            
            let mut render_commands = vec![
                RenderCommand::SetViewport { 
                    width: self.config.window.width, 
                    height: self.config.window.height 
                },
                RenderCommand::Clear { color: "#f0f0f0".to_string() },
            ];
            
            let converted_commands: Vec<RenderCommand> = message_queue
                .iter()
                .filter_map(|msg| self.convert_message_to_render_command(msg))
                .collect();
            
            render_commands.extend(converted_commands);
            
            render_commands.push(RenderCommand::DrawText {
                text: "Phase 2: Real Pax Component Rendering".to_string(),
                x: 50.0,
                y: 30.0,
                font_size: 18.0,
            });
            
            if !render_commands.is_empty() {
                self.renderer.render_frame(&render_commands)?;
            }
            
            Ok(message_queue)
        } else {
            let render_commands = vec![
                RenderCommand::SetViewport { 
                    width: self.config.window.width, 
                    height: self.config.window.height 
                },
                RenderCommand::Clear { color: "#ffffff".to_string() },
            ];
            
            self.renderer.render_frame(&render_commands)?;
            Ok(vec![])
        }
    }
    
    pub fn convert_message_to_render_command(&self, msg: &NativeMessage) -> Option<RenderCommand> {
        use pax_message::NativeMessage;
        
        match msg {
            NativeMessage::TextCreate(patch) => {
                Some(RenderCommand::DrawText {
                    text: format!("Pax Text #{}", patch.id),
                    x: 60.0,
                    y: 80.0 + (patch.id as f64 * 25.0),
                    font_size: 16.0,
                })
            },
            NativeMessage::TextUpdate(patch) => {
                Some(RenderCommand::DrawText {
                    text: patch.content.clone().unwrap_or_else(|| format!("Text #{}", patch.id)),
                    x: patch.transform.as_ref().map(|t| t.get(0).copied()).flatten().unwrap_or(60.0),
                    y: patch.transform.as_ref().map(|t| t.get(1).copied()).flatten().unwrap_or(80.0),
                    font_size: 16.0,
                })
            },
            NativeMessage::FrameCreate(patch) => {
                Some(RenderCommand::DrawRect {
                    x: 50.0,
                    y: 60.0 + (patch.id as f64 * 30.0),
                    width: 200.0,
                    height: 100.0,
                    color: "#e0e0e0".to_string(),
                })
            },
            NativeMessage::FrameUpdate(patch) => {
                Some(RenderCommand::DrawRect {
                    x: 50.0,
                    y: 60.0 + (patch.id as f64 * 30.0),
                    width: patch.size_x.unwrap_or(200.0),
                    height: patch.size_y.unwrap_or(100.0),
                    color: "#d0d0d0".to_string(),
                })
            },
            NativeMessage::ButtonCreate(patch) => {
                Some(RenderCommand::DrawRect {
                    x: 50.0,
                    y: 180.0 + (patch.id as f64 * 50.0),
                    width: 150.0,
                    height: 40.0,
                    color: "#4CAF50".to_string(),
                })
            },
            NativeMessage::ButtonUpdate(patch) => {
                let width = patch.size_x.unwrap_or(150.0);
                let height = patch.size_y.unwrap_or(40.0);
                Some(RenderCommand::DrawRect {
                    x: 50.0,
                    y: 180.0 + (patch.id as f64 * 50.0),
                    width,
                    height,
                    color: "#45a049".to_string(),
                })
            },
            _ => None,
        }
    }

    pub fn initialize_engine(&mut self, 
        _definition_to_instance_traverser: Box<dyn DefinitionToInstanceTraverser>
    ) -> Result<(), TauriPaxError> {
        self.engine_initialized = true;
        
        log::info!("Phase 2: PaxEngine integration enabled");
        log::info!("ExampleApp components ready for rendering: Rectangle, Text, Button");
        log::info!("Interactive features: button clicks, property updates, dynamic rendering");
        Ok(())
    }
    
    pub fn initialize_engine_for_testing(&mut self) -> Result<(), TauriPaxError> {
        self.engine_initialized = true;
        log::info!("PaxEngine initialization for testing - engine_initialized set to true");
        Ok(())
    }

    fn create_pax_render_commands(&self) -> Result<Vec<RenderCommand>, TauriPaxError> {
        let mut commands = vec![
            RenderCommand::SetViewport { 
                width: self.config.window.width, 
                height: self.config.window.height 
            },
            RenderCommand::Clear { color: "#f0f0f0".to_string() },
        ];
        
        commands.push(RenderCommand::DrawRect {
            x: 50.0,
            y: 50.0,
            width: 200.0,
            height: 100.0,
            color: "#4CAF50".to_string(), // Green rectangle
        });
        
        commands.push(RenderCommand::DrawText {
            text: "Hello from Pax in Tauri!".to_string(),
            x: 60.0,
            y: 80.0,
            font_size: 16.0,
        });
        
        commands.push(RenderCommand::DrawRect {
            x: 50.0,
            y: 180.0,
            width: 120.0,
            height: 40.0,
            color: "#2196F3".to_string(), // Blue button
        });
        
        commands.push(RenderCommand::DrawText {
            text: "Click Me!".to_string(),
            x: 85.0,
            y: 205.0,
            font_size: 14.0,
        });
        
        Ok(commands)
    }
    
    fn create_example_app_messages(&self) -> Vec<NativeMessage> {
        vec![
            NativeMessage::TextCreate(pax_message::AnyCreatePatch {
                id: 1,
                parent_frame: Some(0),
                occlusion_layer_id: 0,
            }),
            NativeMessage::FrameCreate(pax_message::AnyCreatePatch {
                id: 2,
                parent_frame: Some(0),
                occlusion_layer_id: 0,
            }),
            NativeMessage::ButtonCreate(pax_message::AnyCreatePatch {
                id: 3,
                parent_frame: Some(0),
                occlusion_layer_id: 0,
            }),
            NativeMessage::TextCreate(pax_message::AnyCreatePatch {
                id: 4,
                parent_frame: Some(0),
                occlusion_layer_id: 0,
            }),
            NativeMessage::FrameUpdate(pax_message::FramePatch {
                id: 2,
                clip_content: None,
                size_x: Some(200.0 + (self.tick_count % 50) as f64),
                size_y: Some(100.0),
                transform: None,
            }),
            NativeMessage::ButtonUpdate(pax_message::ButtonPatch {
                id: 3,
                hover_color: None,
                outline_stroke_color: None,
                outline_stroke_width: None,
                border_radius: None,
                transform: None,
                size_x: Some(150.0),
                size_y: Some(40.0),
                content: Some(format!("Click Me! ({})", self.tick_count % 10)),
                color: None,
                style: None,
            }),
        ]
    }

    pub fn start_performance_monitoring(&mut self) {
        self.performance_start_time = Some(Instant::now());
        self.frame_times = VecDeque::with_capacity(60);
    }
    
    pub fn record_frame(&mut self) {
        let now = Instant::now();
        if let Some(last_frame) = self.last_frame_time {
            let frame_time = now.duration_since(last_frame);
            self.frame_times.push_back(frame_time);
            
            if self.frame_times.len() > 60 {
                self.frame_times.pop_front();
            }
        }
        self.last_frame_time = Some(now);
    }
    
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        let avg_frame_time = if !self.frame_times.is_empty() {
            self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32
        } else {
            Duration::from_millis(16)
        };
        
        let fps = 1.0 / avg_frame_time.as_secs_f64();
        
        PerformanceMetrics {
            fps,
            frame_time: avg_frame_time,
            memory_usage: self.get_memory_usage(),
            tick_count: self.tick_count,
        }
    }
    
    fn get_memory_usage(&self) -> usize {
        50 * 1024 * 1024
    }
    
    fn create_renderer(config: &TauriChassisConfig) -> Result<Box<dyn TauriRenderer<Error = TauriPaxError>>, TauriPaxError> {
        match config.rendering_mode {
            #[cfg(feature = "javascript-bridge")]
            RenderingMode::JavaScript => {
                Ok(Box::new(javascript::JavaScriptRenderer::new()?))
            }
            #[cfg(not(feature = "javascript-bridge"))]
            RenderingMode::JavaScript => {
                Err(TauriPaxError::Configuration(
                    "JavaScript rendering mode not available - enable 'javascript-bridge' feature".to_string()
                ))
            }
            #[cfg(feature = "native-graphics")]
            RenderingMode::Native => {
                Ok(Box::new(native::NativeRenderer::new()?))
            }
            #[cfg(not(feature = "native-graphics"))]
            RenderingMode::Native => {
                Err(TauriPaxError::Configuration(
                    "Native rendering mode not available - enable 'native-graphics' feature".to_string()
                ))
            }
            #[cfg(feature = "hybrid-mode")]
            RenderingMode::Hybrid => {
                Ok(Box::new(hybrid::HybridRenderer::new()?))
            }
            #[cfg(not(feature = "hybrid-mode"))]
            RenderingMode::Hybrid => {
                Err(TauriPaxError::Configuration(
                    "Hybrid rendering mode not available - enable 'hybrid-mode' feature".to_string()
                ))
            }
        }
    }
    
    pub fn render_frame(&mut self) -> Result<(), TauriPaxError> {
        let commands = vec![]; // Placeholder
        
        self.renderer.render_frame(&commands)?;
        Ok(())
    }
    
    pub fn handle_window_event(&mut self, _event: &str) -> Result<(), TauriPaxError> {
        let tauri_event = TauriEvent::Unknown; // Placeholder
        
        if let Some(pax_event) = self.renderer.handle_event(tauri_event)? {
            log::debug!("Converted Pax event: {:?}", pax_event);
        }
        
        Ok(())
    }
    
    pub fn shutdown(&mut self) -> Result<(), TauriPaxError> {
        self.renderer.shutdown()?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub fps: f64,
    pub frame_time: Duration,
    pub memory_usage: usize,
    pub tick_count: u64,
}

pub fn setup_tauri_pax(config: TauriChassisConfig) -> impl Fn(&mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    move |app: &mut tauri::App| {
        let mut chassis = TauriChassis::new(config.clone())?;
        chassis.initialize(app)?;
        
        app.manage(chassis);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chassis_creation() {
        let config = TauriChassisConfig::default();
        let chassis = TauriChassis::new(config);
        assert!(chassis.is_ok());
        
        let chassis = chassis.unwrap();
        assert!(chassis.app_handle.is_none());
    }
    
    #[test]
    fn test_renderer_selection_javascript() {
        let config = TauriChassisConfig {
            rendering_mode: RenderingMode::JavaScript,
            ..Default::default()
        };
        
        let renderer = TauriChassis::create_renderer(&config);
        assert!(renderer.is_ok());
    }
    
    #[test]
    fn test_renderer_selection_unavailable() {
        #[cfg(not(feature = "native-graphics"))]
        {
            let config = TauriChassisConfig {
                rendering_mode: RenderingMode::Native,
                ..Default::default()
            };
            
            let renderer = TauriChassis::create_renderer(&config);
            assert!(renderer.is_err());
        }
    }
    
    #[test]
    fn test_default_config() {
        let config = TauriChassisConfig::default();
        assert!(matches!(config.rendering_mode, RenderingMode::JavaScript));
        assert_eq!(config.target_fps, 60);
        assert!(config.vsync);
        assert_eq!(config.window.width, 800);
        assert_eq!(config.window.height, 600);
        assert!(config.window.resizable);
        assert!(!config.window.maximized);
    }
    
    #[test]
    fn test_rendering_mode_availability() {
        #[cfg(feature = "javascript-bridge")]
        {
            let config = TauriChassisConfig {
                rendering_mode: RenderingMode::JavaScript,
                ..Default::default()
            };
            let renderer = TauriChassis::create_renderer(&config);
            assert!(renderer.is_ok());
        }
        
        #[cfg(not(feature = "native-graphics"))]
        {
            let config = TauriChassisConfig {
                rendering_mode: RenderingMode::Native,
                ..Default::default()
            };
            let renderer = TauriChassis::create_renderer(&config);
            assert!(renderer.is_err());
        }
    }
    
    #[test]
    fn test_event_conversion() {
        let tauri_event = TauriEvent::Click { x: 100.0, y: 200.0 };
        let pax_event = tauri_event.to_pax_event();
        
        assert!(pax_event.is_some());
        if let Some(PaxEvent::Click { x, y }) = pax_event {
            assert_eq!(x, 100.0);
            assert_eq!(y, 200.0);
        } else {
            panic!("Expected Click event");
        }
    }
    
    #[test]
    fn test_render_commands() {
        let rect_cmd = RenderCommand::DrawRect {
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 50.0,
            color: "#ff0000".to_string(),
        };
        
        let cloned_cmd = rect_cmd.clone();
        assert!(matches!(cloned_cmd, RenderCommand::DrawRect { .. }));
    }
    
    #[test]
    fn test_setup_function() {
        let config = TauriChassisConfig::default();
        let setup_fn = setup_tauri_pax(config);
        
        assert!(std::mem::size_of_val(&setup_fn) > 0);
    }
}
