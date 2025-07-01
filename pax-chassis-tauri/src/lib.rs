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

// use pax_runtime::api::{Platform, RenderContext};
use pax_runtime::PaxEngine;
use std::rc::Rc;
use std::cell::RefCell;
use tauri::{App, AppHandle, Manager};

pub struct TauriChassis {
    renderer: Box<dyn TauriRenderer>,
    config: TauriChassisConfig,
    engine: Option<Rc<RefCell<PaxEngine>>>,
    app_handle: Option<AppHandle>,
}

impl TauriChassis {
    pub fn new(config: TauriChassisConfig) -> Result<Self, TauriPaxError> {
        let renderer = Self::create_renderer(&config)?;
        
        Ok(Self {
            renderer,
            config,
            engine: None,
            app_handle: None,
        })
    }
    
    pub fn initialize(&mut self, app: &App) -> Result<(), TauriPaxError> {
        self.app_handle = Some(app.handle());
        self.renderer.initialize(&self.config)?;
        
        
        Ok(())
    }
    
    fn create_renderer(config: &TauriChassisConfig) -> Result<Box<dyn TauriRenderer>, TauriPaxError> {
        match config.rendering_mode {
            #[cfg(feature = "javascript-bridge")]
            RenderingMode::JavaScript => {
                Ok(Box::new(javascript::JavaScriptRenderer::new()?))
            }
            #[cfg(feature = "native-graphics")]
            RenderingMode::Native => {
                Ok(Box::new(native::NativeRenderer::new()?))
            }
            #[cfg(feature = "hybrid-mode")]
            RenderingMode::Hybrid => {
                Ok(Box::new(hybrid::HybridRenderer::new()?))
            }
            _ => Err(TauriPaxError::Configuration(
                "Requested rendering mode not available with current feature flags".into()
            )),
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
        assert!(chassis.engine.is_none());
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
        assert!(!config.window.fullscreen);
    }
    
    #[test]
    fn test_rendering_mode_availability() {
        let js_mode = RenderingMode::JavaScript;
        assert_eq!(js_mode.is_available(), cfg!(feature = "javascript-bridge"));
        
        let native_mode = RenderingMode::Native;
        assert_eq!(native_mode.is_available(), cfg!(feature = "native-graphics"));
        
        let hybrid_mode = RenderingMode::Hybrid;
        assert_eq!(hybrid_mode.is_available(), cfg!(feature = "hybrid-mode"));
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
