
use crate::{TauriRenderer, RenderCommand, TauriEvent, PaxEvent, TauriChassisConfig, TauriPaxError};

pub struct NativeRenderer {
}

impl NativeRenderer {
    pub fn new() -> Result<Self, TauriPaxError> {
        Err(TauriPaxError::Configuration(
            "Native renderer not yet implemented - will be available in Phase 2".into()
        ))
    }
}

impl TauriRenderer for NativeRenderer {
    type Error = TauriPaxError;
    
    fn initialize(&mut self, _config: &TauriChassisConfig) -> Result<(), Self::Error> {
        Err(TauriPaxError::Configuration(
            "Native renderer not yet implemented".into()
        ))
    }
    
    fn render_frame(&mut self, _commands: &[RenderCommand]) -> Result<(), Self::Error> {
        Err(TauriPaxError::Configuration(
            "Native renderer not yet implemented".into()
        ))
    }
    
    fn handle_event(&mut self, _event: TauriEvent) -> Result<Option<PaxEvent>, Self::Error> {
        Err(TauriPaxError::Configuration(
            "Native renderer not yet implemented".into()
        ))
    }
    
    fn shutdown(&mut self) -> Result<(), Self::Error> {
        Err(TauriPaxError::Configuration(
            "Native renderer not yet implemented".into()
        ))
    }
}
