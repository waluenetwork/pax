
use crate::{TauriEvent, PaxEvent, TauriChassisConfig, TauriPaxError};

pub trait TauriRenderer: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    
    fn initialize(&mut self, config: &TauriChassisConfig) -> Result<(), Self::Error>;
    
    fn render_frame(&mut self, commands: &[RenderCommand]) -> Result<(), Self::Error>;
    
    fn handle_event(&mut self, event: TauriEvent) -> Result<Option<PaxEvent>, Self::Error>;
    
    fn shutdown(&mut self) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone)]
pub enum RenderCommand {
    DrawRect { 
        x: f64, 
        y: f64, 
        width: f64, 
        height: f64, 
        color: String 
    },
    
    DrawText { 
        text: String, 
        x: f64, 
        y: f64, 
        font_size: f64 
    },
    
    Clear { color: String },
    
    SetViewport { width: u32, height: u32 },
    
}

#[derive(Debug, Clone, Default)]
pub struct RenderStats {
    pub frame_time_ms: f64,
    
    pub draw_calls: u32,
    
    pub memory_usage: u64,
    
    pub gpu_utilization: f32,
}
