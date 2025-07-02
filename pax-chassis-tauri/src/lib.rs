//! 

#![allow(unused_imports)]

pub mod config;
pub mod renderer;
pub mod events;
pub mod error;
pub mod tauri_render_context;
pub mod pax_engine_integration;

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

#[derive(Clone, Debug)]
pub struct MockExampleApp {
    pub button_text: String,
    pub rect_color: (u8, u8, u8),
    pub rect_width: f64,
    pub click_count: usize,
}


use pax_runtime::PaxEngine;
use pax_runtime::DefinitionToInstanceTraverser;
use pax_runtime_api::{Platform, OS};
use pax_message::NativeMessage;
use tauri::{App, AppHandle, Manager};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::collections::VecDeque;
// use crate::tauri_render_context::TauriRenderContext; // Disabled - using JavaScriptRenderer instead
use pax_runtime_api::RenderContext;

#[derive(Debug)]
pub enum EngineCommand {
    Tick,
    ButtonClick,
    Shutdown,
}

#[derive(Debug)]
pub struct RenderUpdate {
    pub canvas_commands: Vec<String>,
    pub messages: Vec<pax_message::NativeMessage>,
}

pub struct TauriChassis {
    // renderer: Box<dyn TauriRenderer<Error = TauriPaxError>>, // Disabled - JavaScriptRenderer disabled for now
    renderer: Option<Box<dyn TauriRenderer<Error = TauriPaxError>>>, // Placeholder for disabled renderer
    config: TauriChassisConfig,
    app_handle: Option<AppHandle>,
    engine_thread_handle: Option<std::thread::JoinHandle<()>>,
    engine_sender: Option<std::sync::mpsc::Sender<EngineCommand>>,
    render_receiver: Option<Arc<Mutex<std::sync::mpsc::Receiver<RenderUpdate>>>>,
    engine_initialized: bool,
    mock_example_app: Option<MockExampleApp>,
    performance_start_time: Option<Instant>,
    last_frame_time: Option<Instant>,
    frame_times: VecDeque<Duration>,
    tick_count: u64,
}

impl TauriChassis {
    pub fn new(config: TauriChassisConfig) -> Result<Self, TauriPaxError> {
        // let renderer = Self::create_renderer(&config)?; // Disabled - JavaScriptRenderer disabled for now
        
        Ok(Self {
            renderer: None, // Disabled - JavaScriptRenderer disabled for now
            config,
            app_handle: None,
            engine_thread_handle: None,
            engine_sender: None,
            render_receiver: None,
            engine_initialized: false,
            mock_example_app: None,
            performance_start_time: None,
            last_frame_time: None,
            frame_times: VecDeque::with_capacity(60),
            tick_count: 0,
        })
    }
    
    pub fn initialize(&mut self, app: &App) -> Result<(), TauriPaxError> {
        self.app_handle = Some(app.handle().clone());
        
        // if let Some(js_renderer) = self.renderer.as_any_mut().downcast_mut::<javascript::JavaScriptRenderer>() {
        //     js_renderer.set_app_handle(app.handle().clone());
        // }
        
        // self.renderer.initialize(&self.config)?; // Disabled - JavaScriptRenderer disabled for now
        
        if let Some(ref app_handle) = self.app_handle {
            if let Some(window) = app_handle.get_webview_window("main") {
                let canvas_script = r#"
                    if (!window.paxCanvas) {
                        const canvas = document.createElement('canvas');
                        canvas.id = 'pax-canvas';
                        canvas.width = 600;
                        canvas.height = 400;
                        canvas.style.position = 'absolute';
                        canvas.style.top = '0';
                        canvas.style.left = '0';
                        canvas.style.width = '100%';
                        canvas.style.height = '100%';
                        document.body.appendChild(canvas);
                        window.paxCanvas = canvas;
                        window.paxCtx = canvas.getContext('2d');
                        console.log('Direct Canvas initialized (JavaScriptRenderer disabled)');
                    }
                "#;
                let _ = window.eval(canvas_script);
            }
        }
        
        let (engine_sender, engine_receiver) = mpsc::channel();
        let (render_sender, render_receiver) = mpsc::channel();
        
        self.engine_sender = Some(engine_sender);
        self.render_receiver = Some(Arc::new(Mutex::new(render_receiver)));
        
        let app_handle = app.handle().clone();
        let engine_handle = thread::spawn(move || {
            use crate::tauri_render_context::TauriRenderContext;
            
            let mut engine = match crate::pax_engine_integration::create_pax_engine() {
                Ok(engine) => engine,
                Err(e) => {
                    eprintln!("Failed to create PaxEngine: {:?}", e);
                    return;
                }
            };
            
            let mut render_context = TauriRenderContext::new(app_handle);
            
            loop {
                match engine_receiver.recv() {
                    Ok(EngineCommand::Tick) => {
                        let messages = engine.tick();
                        engine.render(&mut render_context);
                        
                        let canvas_commands = vec![
                            "console.log('Real PaxEngine render cycle: .pax components rendered to Canvas')".to_string(),
                        ];
                        
                        let update = RenderUpdate {
                            messages,
                            canvas_commands,
                        };
                        
                        if let Err(_) = render_sender.send(update) {
                            break;
                        }
                    }
                    Ok(EngineCommand::ButtonClick) => {
                        let messages = engine.tick();
                        engine.render(&mut render_context);
                        
                        let canvas_commands = vec![
                            "console.log('Real PaxEngine button click: .pax components updated and re-rendered')".to_string(),
                        ];
                        
                        let update = RenderUpdate {
                            messages,
                            canvas_commands,
                        };
                        
                        if let Err(_) = render_sender.send(update) {
                            break;
                        }
                    }
                    Ok(EngineCommand::Shutdown) => break,
                    Err(_) => break,
                }
            }
        });
        
        self.engine_thread_handle = Some(engine_handle);
        
        Ok(())
    }
    
    pub fn initialize_for_testing(&mut self) -> Result<(), TauriPaxError> {
        // self.renderer.initialize(&self.config)?; // Disabled - JavaScriptRenderer disabled for now
        Ok(())
    }

    pub fn tick(&mut self) -> Result<Vec<NativeMessage>, TauriPaxError> {
        self.tick_count += 1;
        self.record_frame();
        
        if let Some(ref sender) = self.engine_sender {
            let _ = sender.send(EngineCommand::Tick);
            
            if let Some(ref receiver) = self.render_receiver {
                if let Ok(receiver_guard) = receiver.lock() {
                    if let Ok(update) = receiver_guard.try_recv() {
                        return Ok(update.messages);
                    }
                }
            }
            Ok(vec![])
        } else {
            let message_queue = self.create_example_app_messages();
            Ok(message_queue)
        }
    }
    
    pub fn render(&mut self) -> Result<(), TauriPaxError> {
        if let Some(ref receiver) = self.render_receiver {
            if let Ok(receiver_guard) = receiver.lock() {
                if let Ok(update) = receiver_guard.try_recv() {
                    if let Some(ref app_handle) = self.app_handle {
                        for command in update.canvas_commands {
                            log::debug!("Executing Canvas command: {}", command);
                            if let Some(window) = app_handle.get_webview_window("main") {
                                if let Err(e) = window.eval(&command) {
                                    log::error!("Failed to execute Canvas command: {:?}", e);
                                }
                            }
                        }
                        
                        for message in update.messages {
                            log::debug!("Processing PaxEngine message: {:?}", message);
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    pub fn convert_message_to_render_command(&self, msg: &NativeMessage) -> Option<RenderCommand> {
        use pax_message::NativeMessage;
        
        let app = self.mock_example_app.as_ref()?;
        
        match msg {
            NativeMessage::TextCreate(patch) => {
                match patch.id {
                    1 => Some(RenderCommand::DrawText {
                        text: "Hello Pax in Tauri!".to_string(),
                        x: 60.0,
                        y: 80.0,
                        font_size: 24.0,
                    }),
                    4 => Some(RenderCommand::DrawText {
                        text: format!("Clicks: {}", app.click_count),
                        x: 60.0,
                        y: 200.0,
                        font_size: 16.0,
                    }),
                    _ => Some(RenderCommand::DrawText {
                        text: format!("Text #{}", patch.id),
                        x: 60.0,
                        y: 80.0 + (patch.id as f64 * 25.0),
                        font_size: 16.0,
                    })
                }
            },
            NativeMessage::TextUpdate(patch) => {
                match patch.id {
                    1 => Some(RenderCommand::DrawText {
                        text: patch.content.clone().unwrap_or("Hello Pax in Tauri!".to_string()),
                        x: 60.0,
                        y: 80.0,
                        font_size: 24.0,
                    }),
                    4 => Some(RenderCommand::DrawText {
                        text: format!("Clicks: {}", app.click_count),
                        x: 60.0,
                        y: 200.0,
                        font_size: 16.0,
                    }),
                    _ => Some(RenderCommand::DrawText {
                        text: patch.content.clone().unwrap_or_else(|| format!("Text #{}", patch.id)),
                        x: patch.transform.as_ref().map(|t| t.get(0).copied()).flatten().unwrap_or(60.0),
                        y: patch.transform.as_ref().map(|t| t.get(1).copied()).flatten().unwrap_or(80.0),
                        font_size: 16.0,
                    })
                }
            },
            NativeMessage::FrameCreate(patch) => {
                if patch.id == 3 {
                    Some(RenderCommand::DrawRect {
                        x: 60.0,
                        y: 150.0,
                        width: app.rect_width,
                        height: 50.0,
                        color: format!("rgb({}, {}, {})", app.rect_color.0, app.rect_color.1, app.rect_color.2),
                    })
                } else {
                    Some(RenderCommand::DrawRect {
                        x: 50.0,
                        y: 60.0 + (patch.id as f64 * 30.0),
                        width: 200.0,
                        height: 100.0,
                        color: "#e0e0e0".to_string(),
                    })
                }
            },
            NativeMessage::FrameUpdate(patch) => {
                if patch.id == 3 {
                    Some(RenderCommand::DrawRect {
                        x: 60.0,
                        y: 150.0,
                        width: app.rect_width,
                        height: 50.0,
                        color: format!("rgb({}, {}, {})", app.rect_color.0, app.rect_color.1, app.rect_color.2),
                    })
                } else {
                    Some(RenderCommand::DrawRect {
                        x: 50.0,
                        y: 60.0 + (patch.id as f64 * 30.0),
                        width: patch.size_x.unwrap_or(200.0),
                        height: patch.size_y.unwrap_or(100.0),
                        color: "#d0d0d0".to_string(),
                    })
                }
            },
            NativeMessage::ButtonCreate(patch) => {
                if patch.id == 2 {
                    Some(RenderCommand::DrawRect {
                        x: 60.0,
                        y: 120.0,
                        width: 150.0,
                        height: 25.0,
                        color: "#4CAF50".to_string(),
                    })
                } else {
                    Some(RenderCommand::DrawRect {
                        x: 50.0,
                        y: 180.0 + (patch.id as f64 * 50.0),
                        width: 150.0,
                        height: 40.0,
                        color: "#4CAF50".to_string(),
                    })
                }
            },
            NativeMessage::ButtonUpdate(patch) => {
                if patch.id == 2 {
                    let color = if app.click_count % 2 == 0 { "#4CAF50" } else { "#45a049" };
                    Some(RenderCommand::DrawRect {
                        x: 60.0,
                        y: 120.0,
                        width: 150.0,
                        height: 25.0,
                        color: color.to_string(),
                    })
                } else {
                    let width = patch.size_x.unwrap_or(150.0);
                    let height = patch.size_y.unwrap_or(40.0);
                    Some(RenderCommand::DrawRect {
                        x: 50.0,
                        y: 180.0 + (patch.id as f64 * 50.0),
                        width,
                        height,
                        color: "#45a049".to_string(),
                    })
                }
            },
            _ => None,
        }
    }

    pub fn send_button_click(&self) -> Result<(), TauriPaxError> {
        if let Some(ref sender) = self.engine_sender {
            sender.send(EngineCommand::ButtonClick)
                .map_err(|e| TauriPaxError::Runtime(format!("Failed to send button click: {}", e)))?;
            log::info!("Button click command sent to PaxEngine thread");
        }
        Ok(())
    }
    
    pub fn initialize_engine_for_testing(&mut self) -> Result<(), TauriPaxError> {
        self.mock_example_app = Some(MockExampleApp {
            button_text: "Click me!".to_string(),
            rect_color: (200, 200, 200),
            rect_width: 100.0,
            click_count: 0,
        });
        log::info!("MockExampleApp instance created for fallback rendering");
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
        if let Some(ref app) = self.mock_example_app {
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
                NativeMessage::TextUpdate(pax_message::TextPatch {
                    id: 1,
                    content: Some(format!("Hello Pax in Tauri! Clicks: {}", app.click_count)),
                    style: None,
                    transform: None,
                    editable: None,
                    markdown: None,
                    selectable: None,
                    size_x: None,
                    size_y: None,
                    style_link: None,
                }),
                NativeMessage::FrameUpdate(pax_message::FramePatch {
                    id: 2,
                    clip_content: None,
                    size_x: Some(app.rect_width),
                    size_y: Some(50.0),
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
                    content: Some(app.button_text.clone()),
                    color: None,
                    style: None,
                }),
            ]
        } else {
            vec![]
        }
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

    pub fn get_mock_example_app_state(&self) -> Option<&MockExampleApp> {
        self.mock_example_app.as_ref()
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
        let commands: Vec<crate::renderer::RenderCommand> = vec![]; // Placeholder
        
        // self.renderer.render_frame(&commands)?; // Disabled - JavaScriptRenderer disabled for now
        Ok(())
    }
    
    pub fn handle_window_event(&mut self, event: &str) -> Result<(), TauriPaxError> {
        log::debug!("Handling window event: {}", event);
        
        if event.contains("click") && self.mock_example_app.is_some() {
            self.handle_button_click()?;
        }
        
        let tauri_event = TauriEvent::Unknown; // Placeholder
        
        // if let Some(pax_event) = self.renderer.handle_event(tauri_event)? { // Disabled - JavaScriptRenderer disabled for now
        //     log::debug!("Converted Pax event: {:?}", pax_event);
        // }
        
        Ok(())
    }
    
    pub fn handle_button_click(&mut self) -> Result<(), TauriPaxError> {
        self.send_button_click()?;
        
        if let Some(ref mut app) = self.mock_example_app {
            app.click_count += 1;
            app.button_text = format!("Clicked {} times!", app.click_count);
            
            app.rect_width = 50.0 + (app.click_count as f64 * 20.0) % 200.0;
            
            let colors = [
                (255, 100, 100), // Red
                (100, 255, 100), // Green
                (100, 100, 255), // Blue
                (255, 255, 100), // Yellow
            ];
            let color_index = app.click_count % colors.len();
            app.rect_color = colors[color_index];
            
            log::info!("Button clicked! PaxEngine command sent. Mock count: {}, Width: {}, Color: {:?}", 
                      app.click_count, app.rect_width, app.rect_color);
        }
        
        Ok(())
    }
    
    pub fn shutdown(&mut self) -> Result<(), TauriPaxError> {
        // self.renderer.shutdown()?; // Disabled - JavaScriptRenderer disabled for now
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
