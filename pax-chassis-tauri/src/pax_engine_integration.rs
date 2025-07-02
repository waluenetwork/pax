

use std::rc::Rc;

#[derive(Default)]
pub struct PaxEngineComponent {
    pub click_count: usize,
    pub button_text: String,
    pub rect_color: (u8, u8, u8),
    pub rect_width: f64,
}

impl PaxEngineComponent {
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
        
        println!("PaxEngine: Component updated - click_count: {}, rect_width: {}, color: {:?}", 
                 self.click_count, self.rect_width, self.rect_color);
    }
    
    pub fn get_render_commands(&self) -> Vec<String> {
        vec![
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.clearRect(0, 0, window.paxCanvas.width, window.paxCanvas.height); }}"),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#f0f0f0'; ctx.fillRect(0, 0, window.paxCanvas.width, window.paxCanvas.height); }}"),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = 'rgb({}, {}, {})'; ctx.fillRect(50, 50, {}, 100); }}", 
                    self.rect_color.0, self.rect_color.1, self.rect_color.2, self.rect_width),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#333'; ctx.font = '24px Arial'; ctx.textAlign = 'center'; ctx.fillText('PaxEngine Integration', window.paxCanvas.width/2, 200); }}"),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#666'; ctx.font = '16px Arial'; ctx.fillText('{}', window.paxCanvas.width/2, 250); }}", self.button_text),
        ]
    }
    
    pub fn get_click_render_commands(&self) -> Vec<String> {
        vec![
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.clearRect(0, 0, window.paxCanvas.width, window.paxCanvas.height); }}"),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#e8f5e8'; ctx.fillRect(0, 0, window.paxCanvas.width, window.paxCanvas.height); }}"),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = 'rgb({}, {}, {})'; ctx.fillRect(50, 50, {}, 100); }}", 
                    self.rect_color.0, self.rect_color.1, self.rect_color.2, self.rect_width),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#2e7d32'; ctx.font = '24px Arial'; ctx.textAlign = 'center'; ctx.fillText('PaxEngine: Button Clicked!', window.paxCanvas.width/2, 200); }}"),
            
            format!("if (window.paxCanvas) {{ const ctx = window.paxCanvas.getContext('2d'); ctx.fillStyle = '#4caf50'; ctx.font = '16px Arial'; ctx.fillText('{}', window.paxCanvas.width/2, 250); }}", self.button_text),
        ]
    }
}
