
#[derive(Debug, Clone)]
pub enum TauriEvent {
    Click { x: f64, y: f64 },
    
    KeyPress { key: String },
    
    MouseMove { x: f64, y: f64 },
    
    WindowResize { width: u32, height: u32 },
    
    WindowFocus { focused: bool },
    
    Unknown,
}

#[derive(Debug, Clone)]
pub enum PaxEvent {
    Click { x: f64, y: f64 },
    
    KeyPress { key: String },
    
    MouseMove { x: f64, y: f64 },
    
    ViewportChange { width: u32, height: u32 },
    
    FocusChange { focused: bool },
}

impl TauriEvent {
    pub fn to_pax_event(&self) -> Option<PaxEvent> {
        match self {
            TauriEvent::Click { x, y } => Some(PaxEvent::Click { x: *x, y: *y }),
            TauriEvent::KeyPress { key } => Some(PaxEvent::KeyPress { key: key.clone() }),
            TauriEvent::MouseMove { x, y } => Some(PaxEvent::MouseMove { x: *x, y: *y }),
            TauriEvent::WindowResize { width, height } => {
                Some(PaxEvent::ViewportChange { width: *width, height: *height })
            }
            TauriEvent::WindowFocus { focused } => {
                Some(PaxEvent::FocusChange { focused: *focused })
            }
            TauriEvent::Unknown => None,
        }
    }
}
