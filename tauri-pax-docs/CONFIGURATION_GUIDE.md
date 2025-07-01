# Pax-Tauri Configuration Guide

## 🎯 Configuration Overview

This guide provides comprehensive documentation for configuring Pax-Tauri applications, covering all rendering modes, performance settings, and platform-specific optimizations.

## 🔧 Basic Configuration

### **Default Configuration**

```rust
// src/main.rs
use pax_chassis_tauri::*;

#[pax]
#[main]
#[chassis(TauriPaxConfig::default())]
pub struct MyApp {
    // Your application properties
}

// Default configuration provides:
// - JavaScript bridge rendering (maximum compatibility)
// - Balanced performance profile
// - Debug features enabled in development
// - Platform-specific optimizations enabled
```

### **Configuration Builder Pattern**

```rust
use pax_chassis_tauri::*;

let config = TauriPaxConfig::builder()
    .rendering_mode(RenderingMode::Hybrid)
    .performance_profile(PerformanceProfile::HighPerformance)
    .enable_debug_overlay(cfg!(debug_assertions))
    .platform_optimizations(true)
    .vsync(true)
    .target_fps(60)
    .build()?;

#[pax]
#[main]
#[chassis(config)]
pub struct MyApp {
    // Your application
}
```

## 🎨 Rendering Configuration

### **Rendering Mode Selection**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RenderingMode {
    /// JavaScript bridge rendering (maximum compatibility)
    JavaScript,
    
    /// Native graphics rendering (maximum performance)
    Native,
    
    /// Intelligent hybrid mode (automatic switching)
    Hybrid {
        /// Fallback mode if hybrid fails
        fallback: Box<RenderingMode>,
        
        /// Switching strategy configuration
        strategy: HybridStrategy,
    },
}

// Example configurations
let javascript_config = TauriPaxConfig::builder()
    .rendering_mode(RenderingMode::JavaScript)
    .build()?;

let native_config = TauriPaxConfig::builder()
    .rendering_mode(RenderingMode::Native)
    .build()?;

let hybrid_config = TauriPaxConfig::builder()
    .rendering_mode(RenderingMode::Hybrid {
        fallback: Box::new(RenderingMode::JavaScript),
        strategy: HybridStrategy::PerformanceBased,
    })
    .build()?;
```

### **Rendering Quality Settings**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingConfig {
    /// Rendering mode selection
    pub mode: RenderingMode,
    
    /// Anti-aliasing configuration
    pub antialiasing: AntialiasingConfig,
    
    /// Text rendering quality
    pub text_quality: TextQuality,
    
    /// Image scaling quality
    pub image_scaling: ImageScaling,
    
    /// VSync configuration
    pub vsync: bool,
    
    /// Target frame rate
    pub target_fps: u32,
    
    /// GPU acceleration preference
    pub gpu_acceleration: GpuAcceleration,
}

// Example high-quality configuration
let high_quality = RenderingConfig {
    mode: RenderingMode::Native,
    antialiasing: AntialiasingConfig::MSAA4x,
    text_quality: TextQuality::High,
    image_scaling: ImageScaling::Lanczos,
    vsync: true,
    target_fps: 60,
    gpu_acceleration: GpuAcceleration::Preferred,
};

// Example performance-optimized configuration
let performance = RenderingConfig {
    mode: RenderingMode::Hybrid {
        fallback: Box::new(RenderingMode::JavaScript),
        strategy: HybridStrategy::PerformanceBased,
    },
    antialiasing: AntialiasingConfig::None,
    text_quality: TextQuality::Medium,
    image_scaling: ImageScaling::Linear,
    vsync: false,
    target_fps: 120,
    gpu_acceleration: GpuAcceleration::Required,
};
```

## ⚡ Performance Configuration

### **Performance Profiles**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceProfile {
    /// Maximum compatibility, moderate performance
    Compatibility,
    
    /// Balanced performance and quality
    Balanced,
    
    /// Maximum performance, may sacrifice quality
    HighPerformance,
    
    /// Battery-optimized for mobile devices
    PowerSaver,
    
    /// Custom performance configuration
    Custom(CustomPerformanceConfig),
}

// Pre-configured profiles
let config = TauriPaxConfig::builder()
    .performance_profile(PerformanceProfile::HighPerformance)
    .build()?;

// Custom performance configuration
let custom_perf = CustomPerformanceConfig {
    cpu_usage_limit: 0.8,           // 80% CPU usage limit
    memory_limit: 512 * 1024 * 1024, // 512MB memory limit
    gpu_usage_target: 0.6,          // 60% GPU utilization target
    thermal_throttling: true,       // Enable thermal management
    power_management: PowerManagement::Adaptive,
    quality_scaling: true,          // Enable dynamic quality scaling
};

let config = TauriPaxConfig::builder()
    .performance_profile(PerformanceProfile::Custom(custom_perf))
    .build()?;
```

### **Memory Management Configuration**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Maximum texture cache size in MB
    pub texture_cache_size: usize,
    
    /// Maximum font cache size in MB
    pub font_cache_size: usize,
    
    /// Command buffer pool size
    pub command_buffer_pool_size: usize,
    
    /// Enable automatic garbage collection
    pub auto_gc: bool,
    
    /// Memory pressure threshold (0.0-1.0)
    pub memory_pressure_threshold: f32,
    
    /// Memory allocation strategy
    pub allocation_strategy: AllocationStrategy,
}

let memory_config = MemoryConfig {
    texture_cache_size: 128,        // 128MB texture cache
    font_cache_size: 16,            // 16MB font cache
    command_buffer_pool_size: 1000, // 1000 command buffers
    auto_gc: true,
    memory_pressure_threshold: 0.8, // GC at 80% memory usage
    allocation_strategy: AllocationStrategy::PoolBased,
};
```

## 🔄 Hybrid Mode Configuration

### **Hybrid Strategy Configuration**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HybridStrategy {
    /// Switch based on performance metrics
    PerformanceBased,
    
    /// Switch based on scene complexity
    ComplexityBased,
    
    /// Switch based on system resources
    ResourceBased,
    
    /// Custom switching logic
    Custom(CustomHybridConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomHybridConfig {
    /// Complexity threshold for native mode
    pub complexity_threshold: f32,
    
    /// Performance threshold for mode switching
    pub performance_threshold: Duration,
    
    /// Minimum time between mode switches
    pub switch_cooldown: Duration,
    
    /// Enable predictive switching
    pub predictive_switching: bool,
    
    /// Switching decision weights
    pub decision_weights: DecisionWeights,
}

let hybrid_config = CustomHybridConfig {
    complexity_threshold: 100.0,
    performance_threshold: Duration::from_millis(16), // 60 FPS target
    switch_cooldown: Duration::from_millis(500),      // 500ms cooldown
    predictive_switching: true,
    decision_weights: DecisionWeights {
        performance: 0.4,
        complexity: 0.3,
        resources: 0.2,
        user_preference: 0.1,
    },
};
```

### **Mode Switching Configuration**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeSwitchingConfig {
    /// Enable seamless mode switching
    pub seamless_switching: bool,
    
    /// State preservation strategy
    pub state_preservation: StatePreservationStrategy,
    
    /// Resource migration strategy
    pub resource_migration: ResourceMigrationStrategy,
    
    /// Maximum switching latency
    pub max_switch_latency: Duration,
    
    /// Enable switching animations
    pub switching_animations: bool,
}

let switching_config = ModeSwitchingConfig {
    seamless_switching: true,
    state_preservation: StatePreservationStrategy::Full,
    resource_migration: ResourceMigrationStrategy::Lazy,
    max_switch_latency: Duration::from_millis(16),
    switching_animations: false, // Disable for performance
};
```

## 🖥️ Platform-Specific Configuration

### **macOS Configuration**

```rust
#[cfg(target_os = "macos")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacOSConfig {
    /// Use Metal for GPU acceleration
    pub use_metal: bool,
    
    /// Enable Core Animation integration
    pub core_animation: bool,
    
    /// Retina display optimization
    pub retina_optimization: bool,
    
    /// macOS appearance integration
    pub appearance_integration: bool,
    
    /// Touch Bar support
    pub touch_bar_support: bool,
}

#[cfg(target_os = "macos")]
let macos_config = MacOSConfig {
    use_metal: true,
    core_animation: true,
    retina_optimization: true,
    appearance_integration: true,
    touch_bar_support: false,
};
```

### **Windows Configuration**

```rust
#[cfg(target_os = "windows")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsConfig {
    /// Use Direct2D for rendering
    pub use_direct2d: bool,
    
    /// DirectWrite text rendering
    pub directwrite: bool,
    
    /// Windows composition integration
    pub composition_integration: bool,
    
    /// High DPI awareness
    pub high_dpi_awareness: bool,
    
    /// Windows theme integration
    pub theme_integration: bool,
}

#[cfg(target_os = "windows")]
let windows_config = WindowsConfig {
    use_direct2d: true,
    directwrite: true,
    composition_integration: true,
    high_dpi_awareness: true,
    theme_integration: true,
};
```

### **Linux Configuration**

```rust
#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinuxConfig {
    /// Graphics backend preference
    pub graphics_backend: LinuxGraphicsBackend,
    
    /// Wayland support
    pub wayland_support: bool,
    
    /// X11 support
    pub x11_support: bool,
    
    /// Desktop environment integration
    pub desktop_integration: bool,
    
    /// Hardware acceleration detection
    pub hw_accel_detection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinuxGraphicsBackend {
    Vulkan,
    OpenGL,
    Software,
    Auto,
}

#[cfg(target_os = "linux")]
let linux_config = LinuxConfig {
    graphics_backend: LinuxGraphicsBackend::Auto,
    wayland_support: true,
    x11_support: true,
    desktop_integration: true,
    hw_accel_detection: true,
};
```

## 🐛 Debug and Development Configuration

### **Debug Configuration**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    /// Enable debug overlay
    pub debug_overlay: bool,
    
    /// Show performance metrics
    pub performance_metrics: bool,
    
    /// Render debug information
    pub render_debug: bool,
    
    /// Event system debugging
    pub event_debug: bool,
    
    /// Memory usage tracking
    pub memory_tracking: bool,
    
    /// Log level configuration
    pub log_level: LogLevel,
    
    /// Enable profiling hooks
    pub profiling: bool,
}

// Development configuration
let debug_config = DebugConfig {
    debug_overlay: cfg!(debug_assertions),
    performance_metrics: true,
    render_debug: cfg!(debug_assertions),
    event_debug: false,
    memory_tracking: true,
    log_level: if cfg!(debug_assertions) { LogLevel::Debug } else { LogLevel::Info },
    profiling: cfg!(debug_assertions),
};
```

### **Hot Reload Configuration**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotReloadConfig {
    /// Enable hot reload for .pax files
    pub pax_files: bool,
    
    /// Enable hot reload for assets
    pub assets: bool,
    
    /// Hot reload polling interval
    pub poll_interval: Duration,
    
    /// Preserve application state during reload
    pub preserve_state: bool,
    
    /// Show reload notifications
    pub show_notifications: bool,
}

let hot_reload_config = HotReloadConfig {
    pax_files: cfg!(debug_assertions),
    assets: cfg!(debug_assertions),
    poll_interval: Duration::from_millis(500),
    preserve_state: true,
    show_notifications: cfg!(debug_assertions),
};
```

## 📁 Configuration File Formats

### **TOML Configuration**

```toml
# tauri-pax.toml
[rendering]
mode = "Hybrid"
vsync = true
target_fps = 60
antialiasing = "MSAA4x"

[rendering.hybrid]
fallback = "JavaScript"
strategy = "PerformanceBased"

[performance]
profile = "Balanced"
cpu_usage_limit = 0.8
memory_limit = 536870912  # 512MB

[memory]
texture_cache_size = 128
font_cache_size = 16
auto_gc = true

[debug]
debug_overlay = false
performance_metrics = true
log_level = "Info"

[platform.macos]
use_metal = true
retina_optimization = true

[platform.windows]
use_direct2d = true
high_dpi_awareness = true

[platform.linux]
graphics_backend = "Auto"
wayland_support = true
```

### **JSON Configuration**

```json
{
  "rendering": {
    "mode": "Native",
    "vsync": true,
    "target_fps": 120,
    "antialiasing": "None",
    "text_quality": "High"
  },
  "performance": {
    "profile": "HighPerformance",
    "cpu_usage_limit": 0.9,
    "memory_limit": 268435456
  },
  "debug": {
    "debug_overlay": false,
    "performance_metrics": false,
    "log_level": "Warn"
  }
}
```

### **Environment Variable Configuration**

```bash
# Environment variables for configuration override
export TAURI_PAX_RENDERING_MODE=Native
export TAURI_PAX_PERFORMANCE_PROFILE=HighPerformance
export TAURI_PAX_DEBUG_OVERLAY=true
export TAURI_PAX_VSYNC=false
export TAURI_PAX_TARGET_FPS=144
export TAURI_PAX_LOG_LEVEL=Debug
```

## 🔄 Runtime Configuration

### **Dynamic Configuration Updates**

```rust
// Runtime configuration updates
impl TauriPaxApp {
    pub fn update_rendering_mode(&mut self, mode: RenderingMode) -> Result<(), ConfigError> {
        self.config.rendering.mode = mode;
        self.chassis.apply_config_changes(&self.config)?;
        Ok(())
    }
    
    pub fn update_performance_profile(&mut self, profile: PerformanceProfile) -> Result<(), ConfigError> {
        self.config.performance.profile = profile;
        self.chassis.apply_performance_config(&self.config.performance)?;
        Ok(())
    }
    
    pub fn toggle_debug_overlay(&mut self) -> Result<(), ConfigError> {
        self.config.debug.debug_overlay = !self.config.debug.debug_overlay;
        self.chassis.toggle_debug_overlay(self.config.debug.debug_overlay)?;
        Ok(())
    }
}
```

### **Configuration Validation**

```rust
impl TauriPaxConfig {
    pub fn validate(&self) -> Result<(), Vec<ConfigError>> {
        let mut errors = Vec::new();
        
        // Validate rendering configuration
        if let Err(e) = self.rendering.validate() {
            errors.push(e);
        }
        
        // Validate performance configuration
        if let Err(e) = self.performance.validate() {
            errors.push(e);
        }
        
        // Validate platform-specific configuration
        if let Err(e) = self.platform.validate() {
            errors.push(e);
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

## 📚 Configuration Examples

### **Gaming Application Configuration**

```rust
let gaming_config = TauriPaxConfig::builder()
    .rendering_mode(RenderingMode::Native)
    .performance_profile(PerformanceProfile::HighPerformance)
    .target_fps(144)
    .vsync(false)
    .gpu_acceleration(GpuAcceleration::Required)
    .antialiasing(AntialiasingConfig::None)
    .memory_config(MemoryConfig {
        texture_cache_size: 512,
        auto_gc: false,
        allocation_strategy: AllocationStrategy::PreAllocated,
        ..Default::default()
    })
    .build()?;
```

### **Business Application Configuration**

```rust
let business_config = TauriPaxConfig::builder()
    .rendering_mode(RenderingMode::Hybrid {
        fallback: Box::new(RenderingMode::JavaScript),
        strategy: HybridStrategy::ComplexityBased,
    })
    .performance_profile(PerformanceProfile::Balanced)
    .target_fps(60)
    .vsync(true)
    .text_quality(TextQuality::High)
    .debug_config(DebugConfig {
        debug_overlay: false,
        performance_metrics: false,
        log_level: LogLevel::Warn,
        ..Default::default()
    })
    .build()?;
```

### **Development Configuration**

```rust
let dev_config = TauriPaxConfig::builder()
    .rendering_mode(RenderingMode::JavaScript) // For easier debugging
    .performance_profile(PerformanceProfile::Balanced)
    .debug_config(DebugConfig {
        debug_overlay: true,
        performance_metrics: true,
        render_debug: true,
        event_debug: true,
        memory_tracking: true,
        log_level: LogLevel::Debug,
        profiling: true,
    })
    .hot_reload_config(HotReloadConfig {
        pax_files: true,
        assets: true,
        poll_interval: Duration::from_millis(250),
        preserve_state: true,
        show_notifications: true,
    })
    .build()?;
```

This configuration guide provides comprehensive documentation for all aspects of Pax-Tauri configuration, enabling developers to optimize their applications for specific use cases and deployment environments.
