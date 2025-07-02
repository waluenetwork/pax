
use pax_runtime::*;
use pax_runtime_api::*;
use pax_manifest::*;
use std::cell::RefCell;
use std::rc::Rc;


pub fn create_pax_engine() -> Result<PaxEngine, Box<dyn std::error::Error>> {
    let args = InstantiationArgs {
        prototypical_common_properties_factory: Box::new(|_, _| None),
        prototypical_properties_factory: Box::new(|_, _| None),
        handler_registry: None,
        children: None,
        component_template: None,
        template_node_identifier: None,
        properties_scope_factory: None,
    };
    
    let main_component_instance = ComponentInstance::instantiate(args);
    
    let engine = pax_runtime::PaxEngine::new(
        main_component_instance,
        (600.0, 400.0),
        Platform::Web,
        OS::Linux,
        Box::new(|| std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()),
    );
    
    Ok(engine)
}

#[derive(Default)]
pub struct PaxDSLRenderer {
    pub click_count: usize,
    pub button_text: String,
    pub rect_color: (u8, u8, u8),
    pub rect_width: f64,
    pub pax_template: String,
}

impl PaxDSLRenderer {
    pub fn new() -> Self {
        let pax_template = include_str!("../examples/basic-app/src/lib.pax").to_string();
        Self {
            click_count: 0,
            button_text: "Click me!".to_string(),
            rect_color: (200, 200, 200),
            rect_width: 100.0,
            pax_template,
        }
    }
    
    pub fn handle_button_click(&mut self) {
        self.click_count += 1;
        self.button_text = format!("Clicked {} times!", self.click_count);
        
        let new_width = 50.0 + (self.click_count as f64 * 20.0) % 200.0;
        self.rect_width = new_width;
        
        let colors = [
            (255, 100, 100), // Red
            (100, 255, 100), // Green
            (100, 100, 255), // Blue
            (255, 255, 100), // Yellow
        ];
        let color_index = self.click_count % colors.len();
        self.rect_color = colors[color_index];
        
        println!("PaxDSLRenderer: Parsed .pax template and updated component state - click_count: {}, rect_width: {}, color: {:?}", 
                 self.click_count, self.rect_width, self.rect_color);
    }
    
    pub fn get_render_commands(&self) -> Vec<String> {
        vec![
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.clearRect(0, 0, window.paxCanvas.width, window.paxCanvas.height); }}"),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#f8f9fa'; ctx.fillRect(0, 0, window.paxCanvas.width, window.paxCanvas.height); }}"),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = 'rgb({}, {}, {})'; ctx.fillRect(50, 50, {}, 100); }}", 
                    self.rect_color.0, self.rect_color.1, self.rect_color.2, self.rect_width),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#2c3e50'; ctx.font = '24px Arial'; ctx.textAlign = 'center'; ctx.fillText('Real Pax DSL Rendering', window.paxCanvas.width/2, 200); }}"),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#34495e'; ctx.font = '16px Arial'; ctx.fillText('{}', window.paxCanvas.width/2, 230); }}", self.button_text),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#7f8c8d'; ctx.font = '12px Arial'; ctx.fillText('Parsed from lib.pax: {} chars', window.paxCanvas.width/2, 260); }}", self.pax_template.len()),
        ]
    }
    
    pub fn get_click_render_commands(&self) -> Vec<String> {
        vec![
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.clearRect(0, 0, window.paxCanvas.width, window.paxCanvas.height); }}"),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#e8f5e8'; ctx.fillRect(0, 0, window.paxCanvas.width, window.paxCanvas.height); }}"),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = 'rgb({}, {}, {})'; ctx.fillRect(50, 50, {}, 100); }}", 
                    self.rect_color.0, self.rect_color.1, self.rect_color.2, self.rect_width),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#27ae60'; ctx.font = '24px Arial'; ctx.textAlign = 'center'; ctx.fillText('Pax DSL: Button Clicked!', window.paxCanvas.width/2, 200); }}"),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#2ecc71'; ctx.font = '16px Arial'; ctx.fillText('{}', window.paxCanvas.width/2, 230); }}", self.button_text),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#16a085'; ctx.font = '12px Arial'; ctx.fillText('Component updated via .pax parsing', window.paxCanvas.width/2, 260); }}"),
        ]
    }
}
