
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
        self.validate_rendering_mode()?;
        self.validate_performance_settings()?;
        self.validate_window_settings()?;
        Ok(())
    }
    
    fn validate_rendering_mode(&self) -> Result<(), String> {
        match self.rendering_mode {
            #[cfg(not(feature = "javascript-bridge"))]
            RenderingMode::JavaScript => {
                Err("JavaScript rendering mode requires 'javascript-bridge' feature".to_string())
            }
            #[cfg(not(feature = "native-graphics"))]
            RenderingMode::Native => {
                Err("Native rendering mode requires 'native-graphics' feature".to_string())
            }
            #[cfg(not(feature = "hybrid-mode"))]
            RenderingMode::Hybrid => {
                Err("Hybrid rendering mode requires 'hybrid-mode' feature".to_string())
            }
            _ => Ok(())
        }
    }
    
    fn validate_performance_settings(&self) -> Result<(), String> {
        if self.target_fps == 0 {
            return Err("Target FPS must be greater than 0".to_string());
        }
        
        if self.target_fps > 240 {
            return Err("Target FPS should not exceed 240 for practical reasons".to_string());
        }
        
        if self.performance.max_memory_mb == 0 {
            return Err("Maximum memory must be greater than 0".to_string());
        }
        
        if self.performance.max_memory_mb > 8192 {
            return Err("Maximum memory should not exceed 8GB for practical reasons".to_string());
        }
        
        Ok(())
    }
    
    fn validate_window_settings(&self) -> Result<(), String> {
        if self.window.width == 0 || self.window.height == 0 {
            return Err("Window dimensions must be greater than 0".to_string());
        }
        
        if self.window.width > 7680 || self.window.height > 4320 {
            return Err("Window dimensions exceed reasonable limits (8K resolution)".to_string());
        }
        
        if self.window.width < 100 || self.window.height < 100 {
            return Err("Window dimensions too small (minimum 100x100)".to_string());
        }
        
        Ok(())
    }
    
    pub fn get_available_rendering_modes() -> Vec<RenderingMode> {
        let mut modes = Vec::new();
        
        #[cfg(feature = "javascript-bridge")]
        modes.push(RenderingMode::JavaScript);
        
        #[cfg(feature = "native-graphics")]
        modes.push(RenderingMode::Native);
        
        #[cfg(feature = "hybrid-mode")]
        modes.push(RenderingMode::Hybrid);
        
        modes
    }
    
    pub fn is_rendering_mode_available(&self) -> bool {
        Self::get_available_rendering_modes().contains(&self.rendering_mode)
    }
    
    pub fn with_rendering_mode(mut self, mode: RenderingMode) -> Result<Self, String> {
        if !Self::get_available_rendering_modes().contains(&mode) {
            return Err(format!("Rendering mode {:?} not available with current features", mode));
        }
        self.rendering_mode = mode;
        Ok(self)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_validation_fps_limits() {
        let mut config = TauriChassisConfig::default();
        
        config.target_fps = 300;
        assert!(config.validate().is_err());
        
        config.target_fps = 120;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_enhanced_validation_memory_limits() {
        let mut config = TauriChassisConfig::default();
        
        config.performance.max_memory_mb = 10000;
        assert!(config.validate().is_err());
        
        config.performance.max_memory_mb = 1024;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_enhanced_validation_window_limits() {
        let mut config = TauriChassisConfig::default();
        
        config.window.width = 50;
        config.window.height = 50;
        assert!(config.validate().is_err());
        
        config.window.width = 10000;
        config.window.height = 10000;
        assert!(config.validate().is_err());
        
        config.window.width = 1024;
        config.window.height = 768;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_available_rendering_modes() {
        let modes = TauriChassisConfig::get_available_rendering_modes();
        assert!(!modes.is_empty());
        
        #[cfg(feature = "javascript-bridge")]
        assert!(modes.contains(&RenderingMode::JavaScript));
    }

    #[test]
    fn test_rendering_mode_availability_check() {
        let config = TauriChassisConfig::default();
        assert!(config.is_rendering_mode_available());
    }

    #[test]
    fn test_with_rendering_mode_builder() {
        let config = TauriChassisConfig::default();
        
        let available_modes = TauriChassisConfig::get_available_rendering_modes();
        if let Some(mode) = available_modes.first() {
            let result = config.with_rendering_mode(*mode);
            assert!(result.is_ok());
        }
    }
}
