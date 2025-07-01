
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TauriPaxError {
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Rendering error: {0}")]
    Rendering(String),
    
    #[error("Event handling error: {0}")]
    Event(String),
    
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Generic error: {0}")]
    Generic(#[from] anyhow::Error),
}

pub type TauriPaxResult<T> = Result<T, TauriPaxError>;
