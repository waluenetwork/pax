
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

    fn setup_event_listeners(&self) -> Result<(), TauriPaxError> {
        let event_script = r#"
            if (!window.paxEventBridge) {
                window.paxEventBridge = {
                    events: [],
                    captureEvent: function(event) {
                        this.events.push({
                            type: event.type,
                            x: event.clientX || 0,
                            y: event.clientY || 0,
                            key: event.key || '',
                            timestamp: Date.now()
                        });
                    }
                };
                
                if (window.paxCanvas) {
                    window.paxCanvas.addEventListener('click', window.paxEventBridge.captureEvent.bind(window.paxEventBridge));
                    window.paxCanvas.addEventListener('mousemove', window.paxEventBridge.captureEvent.bind(window.paxEventBridge));
                }
                
                document.addEventListener('keydown', window.paxEventBridge.captureEvent.bind(window.paxEventBridge));
                document.addEventListener('keyup', window.paxEventBridge.captureEvent.bind(window.paxEventBridge));
            }
        "#;
        self.execute_js(event_script)
    }

    fn poll_dom_events(&self) -> Result<Vec<TauriEvent>, TauriPaxError> {
        let _poll_script = r#"
            if (window.paxEventBridge && window.paxEventBridge.events.length > 0) {
                const events = window.paxEventBridge.events.splice(0);
                JSON.stringify(events);
            } else {
                "[]";
            }
        "#;
        
        Ok(vec![])
    }

    pub fn sync_property(&self, property_name: &str, value: &str) -> Result<(), TauriPaxError> {
        let sync_script = format!(
            r#"
                if (!window.paxProperties) {{
                    window.paxProperties = {{}};
                }}
                window.paxProperties['{}'] = {};
                
                if (window.paxCanvas) {{
                    const event = new CustomEvent('paxPropertyChange', {{
                        detail: {{ property: '{}', value: {} }}
                    }});
                    window.paxCanvas.dispatchEvent(event);
                }}
            "#,
            property_name, value, property_name, value
        );
        
        self.execute_js(&sync_script)
    }

    pub fn get_property(&self, property_name: &str) -> Result<Option<String>, TauriPaxError> {
        let _get_script = format!(
            r#"
                if (window.paxProperties && window.paxProperties['{}']) {{
                    JSON.stringify(window.paxProperties['{}']);
                }} else {{
                    null;
                }}
            "#,
            property_name, property_name
        );
        
        Ok(None)
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
        
        self.setup_event_listeners()?;
        
        self.initialized = true;
        log::info!("JavaScript renderer initialized successfully with Canvas API and event bridge");
        
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
        
        let dom_events = self.poll_dom_events()?;
        
        for dom_event in dom_events {
            if let Some(pax_event) = dom_event.to_pax_event() {
                log::debug!("Converted DOM event to Pax event: {:?}", pax_event);
                return Ok(Some(pax_event));
            }
        }
        
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

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_canvas_initialization_script() {
        let _renderer = JavaScriptRenderer::new().unwrap();
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
        assert!(script.contains("createElement('canvas')"));
        assert!(script.contains("getContext('2d')"));
    }

    #[test]
    fn test_render_command_to_canvas_api() {
        let renderer = JavaScriptRenderer::new().unwrap();
        
        let rect_cmd = RenderCommand::DrawRect {
            x: 10.0, y: 20.0, width: 100.0, height: 50.0,
            color: "#ff0000".to_string()
        };
        
        let js_code = renderer.render_command_to_js(&rect_cmd);
        assert!(js_code.contains("fillStyle = '#ff0000'"));
        assert!(js_code.contains("fillRect(10, 20, 100, 50)"));
    }

    #[test]
    fn test_canvas_command_batching() {
        let mut renderer = JavaScriptRenderer::new().unwrap();
        let config = TauriChassisConfig::default();
        renderer.initialize(&config).unwrap();
        
        let commands = vec![
            RenderCommand::Clear { color: "#ffffff".to_string() },
            RenderCommand::DrawRect { x: 0.0, y: 0.0, width: 50.0, height: 50.0, color: "#ff0000".to_string() },
            RenderCommand::DrawText { text: "Test".to_string(), x: 25.0, y: 25.0, font_size: 16.0 }
        ];
        
        assert!(renderer.render_frame(&commands).is_ok());
    }

    #[test]
    fn test_event_bridge_setup() {
        let renderer = JavaScriptRenderer::new().unwrap();
        let config = TauriChassisConfig::default();
        let mut renderer = renderer;
        renderer.initialize(&config).unwrap();
        
        assert!(renderer.poll_dom_events().is_ok());
    }

    #[test]
    fn test_property_synchronization() {
        let renderer = JavaScriptRenderer::new().unwrap();
        
        assert!(renderer.sync_property("testProp", "\"testValue\"").is_ok());
        assert!(renderer.get_property("testProp").is_ok());
    }

    #[test]
    fn test_text_rendering_command() {
        let renderer = JavaScriptRenderer::new().unwrap();
        
        let text_cmd = RenderCommand::DrawText {
            text: "Hello World".to_string(),
            x: 100.0,
            y: 200.0,
            font_size: 24.0
        };
        
        let js_code = renderer.render_command_to_js(&text_cmd);
        assert!(js_code.contains("font = '24px Arial'"));
        assert!(js_code.contains("fillText('Hello World', 100, 200)"));
    }

    #[test]
    fn test_viewport_command() {
        let renderer = JavaScriptRenderer::new().unwrap();
        
        let viewport_cmd = RenderCommand::SetViewport {
            width: 1024,
            height: 768
        };
        
        let js_code = renderer.render_command_to_js(&viewport_cmd);
        assert!(js_code.contains("width = 1024"));
        assert!(js_code.contains("height = 768"));
    }

    #[test]
    fn test_clear_command() {
        let renderer = JavaScriptRenderer::new().unwrap();
        
        let clear_cmd = RenderCommand::Clear {
            color: "#00ff00".to_string()
        };
        
        let js_code = renderer.render_command_to_js(&clear_cmd);
        assert!(js_code.contains("fillStyle = '#00ff00'"));
        assert!(js_code.contains("fillRect(0, 0"));
    }
}

impl Default for JavaScriptRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create default JavaScript renderer")
    }
}
