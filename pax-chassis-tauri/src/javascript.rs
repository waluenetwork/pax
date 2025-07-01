
use crate::{TauriRenderer, RenderCommand, TauriEvent, PaxEvent, TauriChassisConfig, TauriPaxError};

pub struct JavaScriptRenderer {
    initialized: bool,
    
    config: Option<TauriChassisConfig>,
}

impl JavaScriptRenderer {
    pub fn new() -> Result<Self, TauriPaxError> {
        Ok(Self {
            initialized: false,
            config: None,
        })
    }
    
    fn ensure_initialized(&self) -> Result<(), TauriPaxError> {
        if !self.initialized {
            return Err(TauriPaxError::Rendering(
                "JavaScript renderer not initialized".to_string()
            ));
        }
        Ok(())
    }
}

impl TauriRenderer for JavaScriptRenderer {
    type Error = TauriPaxError;
    
    fn initialize(&mut self, config: &TauriChassisConfig) -> Result<(), Self::Error> {
        log::info!("Initializing JavaScript renderer");
        
        config.validate().map_err(|e| TauriPaxError::Configuration(e))?;
        
        self.config = Some(config.clone());
        
        
        self.initialized = true;
        log::info!("JavaScript renderer initialized successfully");
        
        Ok(())
    }
    
    fn render_frame(&mut self, commands: &[RenderCommand]) -> Result<(), Self::Error> {
        self.ensure_initialized()?;
        
        
        log::debug!("Rendering frame with {} commands", commands.len());
        
        for command in commands {
            match command {
                RenderCommand::DrawRect { x, y, width, height, color } => {
                    log::trace!("Drawing rectangle at ({}, {}) {}x{} color: {}", x, y, width, height, color);
                }
                RenderCommand::DrawText { text, x, y, font_size } => {
                    log::trace!("Drawing text '{}' at ({}, {}) size: {}", text, x, y, font_size);
                }
                RenderCommand::Clear { color } => {
                    log::trace!("Clearing with color: {}", color);
                }
                RenderCommand::SetViewport { width, height } => {
                    log::trace!("Setting viewport to {}x{}", width, height);
                }
            }
        }
        
        Ok(())
    }
    
    fn handle_event(&mut self, event: TauriEvent) -> Result<Option<PaxEvent>, Self::Error> {
        self.ensure_initialized()?;
        
        let pax_event = event.to_pax_event();
        
        if let Some(ref pax_event) = pax_event {
            log::debug!("Converted Tauri event to Pax event: {:?}", pax_event);
        } else {
            log::trace!("Tauri event could not be converted: {:?}", event);
        }
        
        Ok(pax_event)
    }
    
    fn shutdown(&mut self) -> Result<(), Self::Error> {
        if !self.initialized {
            return Ok(());
        }
        
        log::info!("Shutting down JavaScript renderer");
        
        
        self.initialized = false;
        self.config = None;
        
        log::info!("JavaScript renderer shutdown complete");
        
        Ok(())
    }
}

impl Default for JavaScriptRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create default JavaScript renderer")
    }
}
