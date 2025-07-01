
use crate::{TauriRenderer, RenderCommand, TauriEvent, PaxEvent, TauriChassisConfig, TauriPaxError};
use tauri::Manager;

pub struct JavaScriptRenderer {
    initialized: bool,
    config: Option<TauriChassisConfig>,
    app_handle: Option<tauri::AppHandle>,
    canvas_context_id: Option<String>,
}

impl JavaScriptRenderer {
    pub fn new() -> Result<Self, TauriPaxError> {
        Ok(Self {
            initialized: false,
            config: None,
            app_handle: None,
            canvas_context_id: None,
        })
    }

    fn execute_js(&self, script: &str) -> Result<(), TauriPaxError> {
        if let Some(app_handle) = &self.app_handle {
            if let Some(window) = app_handle.get_webview_window("main") {
                window.eval(script).map_err(|e| TauriPaxError::Rendering(
                    format!("Failed to execute JavaScript: {}", e)
                ))?;
            }
        }
        Ok(())
    }

    fn init_canvas(&self) -> Result<(), TauriPaxError> {
        let script = r#"
            if (!window.paxCanvas) {
                const canvas = document.createElement('canvas');
                canvas.id = 'pax-canvas';
                canvas.style.position = 'absolute';
                canvas.style.top = '0';
                canvas.style.left = '0';
                canvas.style.width = '100%';
                canvas.style.height = '100%';
                document.body.appendChild(canvas);
                window.paxCanvas = canvas;
                window.paxCtx = canvas.getContext('2d');
            }
        "#;
        self.execute_js(script)
    }

    fn render_command_to_js(&self, command: &RenderCommand) -> String {
        match command {
            RenderCommand::DrawRect { x, y, width, height, color } => {
                format!(
                    "window.paxCtx.fillStyle = '{}'; window.paxCtx.fillRect({}, {}, {}, {});",
                    color, x, y, width, height
                )
            }
            RenderCommand::DrawText { text, x, y, font_size } => {
                format!(
                    "window.paxCtx.font = '{}px Arial'; window.paxCtx.fillText('{}', {}, {});",
                    font_size, text, x, y
                )
            }
            RenderCommand::Clear { color } => {
                format!(
                    "window.paxCtx.fillStyle = '{}'; window.paxCtx.fillRect(0, 0, window.paxCanvas.width, window.paxCanvas.height);",
                    color
                )
            }
            RenderCommand::SetViewport { width, height } => {
                format!(
                    "window.paxCanvas.width = {}; window.paxCanvas.height = {};",
                    width, height
                )
            }
        }
    }

    pub fn set_app_handle(&mut self, app_handle: tauri::AppHandle) {
        self.app_handle = Some(app_handle);
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
        
        self.init_canvas()?;
        
        self.initialized = true;
        log::info!("JavaScript renderer initialized successfully");
        
        Ok(())
    }
    
    fn render_frame(&mut self, commands: &[RenderCommand]) -> Result<(), Self::Error> {
        self.ensure_initialized()?;
        
        log::debug!("Rendering frame with {} commands", commands.len());
        
        let mut js_commands = Vec::new();
        
        for command in commands {
            let js_command = self.render_command_to_js(command);
            js_commands.push(js_command);
            
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
        
        if !js_commands.is_empty() {
            let batch_script = js_commands.join("\n");
            self.execute_js(&batch_script)?;
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
        self.app_handle = None;
        self.canvas_context_id = None;
        
        log::info!("JavaScript renderer shutdown complete");
        
        Ok(())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Default for JavaScriptRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create default JavaScript renderer")
    }
}
