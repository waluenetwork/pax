# Pax-Chassis-Tauri

Tauri chassis for the Pax UI framework, providing dual-mode rendering capabilities for cross-platform desktop applications.

## Features

- **Dual-Mode Rendering**: JavaScript Bridge and Native Graphics modes
- **Intelligent Hybrid Mode**: Automatic switching between rendering backends
- **Cross-Platform**: Support for macOS, Windows, and Linux
- **Performance Monitoring**: Built-in performance metrics and optimization
- **Modular Architecture**: Pluggable rendering backends through trait abstraction

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
pax-chassis-tauri = "0.38.3"
tauri = { version = "2.0", features = ["api-all"] }
```

Basic usage:

```rust
use pax_chassis_tauri::{TauriChassis, TauriChassisConfig};

#[tokio::main]
async fn main() {
    let config = TauriChassisConfig::default();
    
    tauri::Builder::default()
        .setup(|app| {
            let mut chassis = TauriChassis::new(config)?;
            chassis.initialize(app)?;
            app.manage(chassis);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Configuration

The chassis supports extensive configuration through the `TauriChassisConfig` struct:

```rust
let config = TauriChassisConfig::builder()
    .rendering_mode(RenderingMode::Hybrid)
    .target_fps(60)
    .vsync(true)
    .debug_mode(true)
    .build()?;
```

## Rendering Modes

### JavaScript Bridge Mode
- Maximum compatibility across platforms
- WebView-based rendering using Canvas API
- Ideal for UI-heavy applications

### Native Graphics Mode
- Maximum performance using platform-specific APIs
- Direct GPU acceleration
- Ideal for graphics-intensive applications

### Hybrid Mode
- Intelligent switching between JavaScript and Native modes
- Automatic performance optimization
- Best of both worlds

## Examples

See the `examples/` directory for complete working examples:

- `basic-app/`: Minimal Tauri-Pax application setup
- More examples will be added in future phases

## Development Status

This is Phase 1 (Foundation) of the Pax-Tauri integration project. Current status:

- ✅ Project structure and workspace integration
- ✅ Core trait abstractions and configuration system
- ✅ JavaScript renderer stub implementation
- 🚧 JavaScript bridge implementation (Unit 1.4)
- 🚧 Pax engine integration (Unit 1.5)
- 🚧 Native graphics renderer (Phase 2)
- 🚧 Hybrid mode implementation (Phase 3)

## Architecture

The chassis follows a modular architecture with the following components:

- **TauriChassis**: Main integration point with Tauri
- **TauriRenderer**: Trait for pluggable rendering backends
- **Event System**: Translation between Tauri and Pax events
- **Configuration**: Comprehensive configuration and validation
- **Error Handling**: Structured error types and handling

## Contributing

This project is part of the larger Pax ecosystem. See the main Pax repository for contribution guidelines.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
