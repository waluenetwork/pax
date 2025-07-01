
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauriChassisConfig {
    pub rendering_mode: RenderingMode,
    
    pub vsync: bool,
    
    pub target_fps: u32,
    
    pub debug_mode: bool,
    
    pub window: WindowConfig,
    
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RenderingMode {
    JavaScript,
    
    Native,
    
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    
    pub height: u32,
    
    pub title: String,
    
    pub resizable: bool,
    
    pub maximized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub gpu_acceleration: bool,
    
    pub max_memory_mb: u32,
    
    pub monitoring: bool,
}

impl Default for TauriChassisConfig {
    fn default() -> Self {
        Self {
            rendering_mode: RenderingMode::JavaScript,
            vsync: true,
            target_fps: 60,
            debug_mode: cfg!(debug_assertions),
            window: WindowConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Pax-Tauri Application".to_string(),
            resizable: true,
            maximized: false,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            gpu_acceleration: true,
            max_memory_mb: 512,
            monitoring: cfg!(debug_assertions),
        }
    }
}

impl TauriChassisConfig {
    pub fn builder() -> TauriChassisConfigBuilder {
        TauriChassisConfigBuilder::new()
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self.target_fps == 0 {
            return Err("Target FPS must be greater than 0".to_string());
        }
        
        if self.window.width == 0 || self.window.height == 0 {
            return Err("Window dimensions must be greater than 0".to_string());
        }
        
        if self.performance.max_memory_mb == 0 {
            return Err("Maximum memory must be greater than 0".to_string());
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TauriChassisConfigBuilder {
    config: TauriChassisConfig,
}

impl TauriChassisConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: TauriChassisConfig::default(),
        }
    }
    
    pub fn rendering_mode(mut self, mode: RenderingMode) -> Self {
        self.config.rendering_mode = mode;
        self
    }
    
    pub fn vsync(mut self, enabled: bool) -> Self {
        self.config.vsync = enabled;
        self
    }
    
    pub fn target_fps(mut self, fps: u32) -> Self {
        self.config.target_fps = fps;
        self
    }
    
    pub fn debug_mode(mut self, enabled: bool) -> Self {
        self.config.debug_mode = enabled;
        self
    }
    
    pub fn window(mut self, window: WindowConfig) -> Self {
        self.config.window = window;
        self
    }
    
    pub fn performance(mut self, performance: PerformanceConfig) -> Self {
        self.config.performance = performance;
        self
    }
    
    pub fn build(self) -> Result<TauriChassisConfig, String> {
        self.config.validate()?;
        Ok(self.config)
    }
}
