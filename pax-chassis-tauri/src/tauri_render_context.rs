use pax_runtime_api::RenderContext;
use tauri::{AppHandle, Manager};
use std::rc::Rc;
use std::cell::RefCell;
use pax_runtime::api::Fill;

pub struct TauriRenderContext {
    app_handle: AppHandle,
    layers: usize,
    dirty_canvases: Rc<RefCell<Vec<bool>>>,
}

unsafe impl Send for TauriRenderContext {}
unsafe impl Sync for TauriRenderContext {}

impl TauriRenderContext {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            layers: 1,
            dirty_canvases: Rc::new(RefCell::new(vec![false])),
        }
    }
    
    pub fn execute_js(&self, script: &str) -> Result<(), String> {
        if let Some(window) = self.app_handle.get_webview_window("main") {
            window.eval(script).map_err(|e| format!("JavaScript execution error: {:?}", e))?;
            Ok(())
        } else {
            Err("Main window not found".to_string())
        }
    }
}

impl RenderContext for TauriRenderContext {
    fn layers(&self) -> usize {
        self.layers
    }

    fn resize_layers_to(&mut self, layer_count: usize, dirty_canvases: Rc<RefCell<Vec<bool>>>) {
        self.layers = layer_count;
        self.dirty_canvases = dirty_canvases;
        
        let script = format!(
            r#"
            const container = document.getElementById('pax-container');
            if (container) {{
                const canvases = container.querySelectorAll('canvas');
                for (let i = {}; i < canvases.length; i++) {{
                    canvases[i].remove();
                }}
                
                for (let i = canvases.length; i < {}; i++) {{
                    const newCanvas = document.createElement('canvas');
                    newCanvas.id = i.toString();
                    newCanvas.className = 'pax-layer';
                    newCanvas.width = 600;
                    newCanvas.height = 400;
                    newCanvas.style.position = 'absolute';
                    newCanvas.style.top = '0';
                    newCanvas.style.left = '0';
                    container.appendChild(newCanvas);
                }}
            }}
            "#,
            layer_count, layer_count
        );
        
        if let Err(e) = self.execute_js(&script) {
            eprintln!("Failed to resize layers: {}", e);
        }
    }

    fn clear(&mut self, layer: usize) {
        let script = format!(
            r#"
            const canvasElement = document.getElementById('{}');
            if (canvasElement) {{
                const ctx = canvasElement.getContext('2d');
                ctx.clearRect(0, 0, canvasElement.width, canvasElement.height);
            }}
            "#,
            layer
        );
        
        if let Err(e) = self.execute_js(&script) {
            eprintln!("Failed to clear layer {}: {}", layer, e);
        }
    }

    fn flush(&mut self, layer: usize, _dirty_canvases: Rc<RefCell<Vec<bool>>>) {
        let script = format!(
            r#"
            const canvasElement = document.getElementById('{}');
            if (canvasElement) {{
                const ctx = canvasElement.getContext('2d');
                ctx.save();
                ctx.restore();
            }}
            "#,
            layer
        );
        
        if let Err(e) = self.execute_js(&script) {
            eprintln!("Failed to flush layer {}: {}", layer, e);
        }
    }

    fn load_image(&mut self, _path: &str, _data: &[u8], _width: usize, _height: usize) {
    }

    fn image_loaded(&self, _path: &str) -> bool {
        false
    }

    fn resize(&mut self, _width: usize, _height: usize) {
        let script = r#"
        const canvases = document.querySelectorAll('#pax-container canvas');
        canvases.forEach(canvasElement => {
            canvasElement.width = 600;
            canvasElement.height = 400;
        });
        "#;
        
        if let Err(e) = self.execute_js(script) {
            eprintln!("Failed to resize canvases: {}", e);
        }
    }

    fn fill(&mut self, _layer: usize, _path: kurbo::BezPath, _fill: &Fill) {
        let script = format!(
            r#"
            const canvasElement = document.getElementById('{}');
            if (canvasElement) {{
                const ctx = canvasElement.getContext('2d');
                ctx.fillStyle = 'rgba(100, 150, 200, 0.8)';
                ctx.fill();
            }}
            "#,
            _layer
        );
        let _ = self.execute_js(&script);
    }

    fn stroke(&mut self, _layer: usize, _path: kurbo::BezPath, _fill: &Fill, _width: f64) {
        let script = format!(
            r#"
            const canvasElement = document.getElementById('{}');
            if (canvasElement) {{
                const ctx = canvasElement.getContext('2d');
                ctx.strokeStyle = 'rgba(100, 150, 200, 0.8)';
                ctx.lineWidth = {};
                ctx.stroke();
            }}
            "#,
            _layer, _width
        );
        let _ = self.execute_js(&script);
    }

    fn save(&mut self, _layer: usize) {
        let script = format!(
            r#"
            const canvasElement = document.getElementById('{}');
            if (canvasElement) {{
                const ctx = canvasElement.getContext('2d');
                ctx.save();
            }}
            "#,
            _layer
        );
        let _ = self.execute_js(&script);
    }

    fn restore(&mut self, _layer: usize) {
        let script = format!(
            r#"
            const canvasElement = document.getElementById('{}');
            if (canvasElement) {{
                const ctx = canvasElement.getContext('2d');
                ctx.restore();
            }}
            "#,
            _layer
        );
        let _ = self.execute_js(&script);
    }

    fn clip(&mut self, _layer: usize, _path: kurbo::BezPath) {
        let script = format!(
            r#"
            const canvasElement = document.getElementById('{}');
            if (canvasElement) {{
                const ctx = canvasElement.getContext('2d');
                ctx.clip();
            }}
            "#,
            _layer
        );
        let _ = self.execute_js(&script);
    }

    fn transform(&mut self, _layer: usize, _transform: kurbo::Affine) {
        let script = format!(
            r#"
            const canvasElement = document.getElementById('{}');
            if (canvasElement) {{
                const ctx = canvasElement.getContext('2d');
                ctx.setTransform({}, {}, {}, {}, {}, {});
            }}
            "#,
            _layer, _transform.as_coeffs()[0], _transform.as_coeffs()[1], 
            _transform.as_coeffs()[2], _transform.as_coeffs()[3], 
            _transform.as_coeffs()[4], _transform.as_coeffs()[5]
        );
        let _ = self.execute_js(&script);
    }

    fn draw_image(&mut self, _layer: usize, _path: &str, _rect: kurbo::Rect) {
        let script = format!(
            r#"
            const canvasElement = document.getElementById('{}');
            if (canvasElement) {{
                const ctx = canvasElement.getContext('2d');
                const img = new Image();
                img.onload = function() {{
                    ctx.drawImage(img, {}, {}, {}, {});
                }};
                img.src = '{}';
            }}
            "#,
            _layer, _rect.x0, _rect.y0, _rect.width(), _rect.height(), _path
        );
        let _ = self.execute_js(&script);
    }

    fn get_image_size(&mut self, _path: &str) -> Option<(usize, usize)> {
        None
    }
}
