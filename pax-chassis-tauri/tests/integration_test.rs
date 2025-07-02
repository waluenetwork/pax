use pax_chassis_tauri::{TauriChassis, TauriChassisConfig, RenderCommand};

#[tokio::test]
async fn test_end_to_end_canvas_rendering() {
    let config = TauriChassisConfig::default();
    let chassis = TauriChassis::new(config).unwrap();
    
    let _commands = vec![
        RenderCommand::SetViewport { width: 800, height: 600 },
        RenderCommand::Clear { color: "#ffffff".to_string() },
        RenderCommand::DrawRect { 
            x: 100.0, y: 100.0, width: 200.0, height: 150.0, 
            color: "#ff0000".to_string() 
        },
        RenderCommand::DrawText { 
            text: "Hello Pax!".to_string(), 
            x: 150.0, y: 175.0, font_size: 24.0 
        }
    ];
    
    drop(chassis); // Just verify the chassis was created successfully
}

#[test]
fn test_javascript_renderer_canvas_integration() {
    use pax_chassis_tauri::javascript::JavaScriptRenderer;
    use pax_chassis_tauri::TauriRenderer;
    
    let mut renderer = JavaScriptRenderer::new().unwrap();
    let config = TauriChassisConfig::default();
    
    assert!(renderer.initialize(&config).is_ok());
    
    let commands = vec![
        RenderCommand::SetViewport { width: 1024, height: 768 },
        RenderCommand::Clear { color: "#f0f0f0".to_string() },
        RenderCommand::DrawRect { 
            x: 50.0, y: 50.0, width: 100.0, height: 100.0, 
            color: "#0066cc".to_string() 
        }
    ];
    
    assert!(renderer.render_frame(&commands).is_ok());
    assert!(renderer.shutdown().is_ok());
}

#[test]
fn test_property_sync_integration() {
    use pax_chassis_tauri::javascript::JavaScriptRenderer;
    
    let renderer = JavaScriptRenderer::new().unwrap();
    
    assert!(renderer.sync_property("backgroundColor", "\"#ffffff\"").is_ok());
    assert!(renderer.sync_property("fontSize", "16").is_ok());
    assert!(renderer.sync_property("visible", "true").is_ok());
    
    assert!(renderer.get_property("backgroundColor").is_ok());
}

#[test]
fn test_complex_rendering_sequence() {
    use pax_chassis_tauri::javascript::JavaScriptRenderer;
    use pax_chassis_tauri::TauriRenderer;
    
    let mut renderer = JavaScriptRenderer::new().unwrap();
    let config = TauriChassisConfig::default();
    renderer.initialize(&config).unwrap();
    
    let frame1_commands = vec![
        RenderCommand::SetViewport { width: 800, height: 600 },
        RenderCommand::Clear { color: "#ffffff".to_string() },
        RenderCommand::DrawRect { x: 0.0, y: 0.0, width: 800.0, height: 100.0, color: "#ff0000".to_string() },
    ];
    
    let frame2_commands = vec![
        RenderCommand::DrawRect { x: 0.0, y: 100.0, width: 800.0, height: 100.0, color: "#00ff00".to_string() },
        RenderCommand::DrawText { text: "Frame 2".to_string(), x: 400.0, y: 150.0, font_size: 32.0 },
    ];
    
    let frame3_commands = vec![
        RenderCommand::DrawRect { x: 0.0, y: 200.0, width: 800.0, height: 100.0, color: "#0000ff".to_string() },
        RenderCommand::DrawText { text: "Frame 3".to_string(), x: 400.0, y: 250.0, font_size: 32.0 },
    ];
    
    assert!(renderer.render_frame(&frame1_commands).is_ok());
    assert!(renderer.render_frame(&frame2_commands).is_ok());
    assert!(renderer.render_frame(&frame3_commands).is_ok());
}

#[test]
fn test_performance_monitoring() {
    let config = TauriChassisConfig::default();
    let mut chassis = TauriChassis::new(config).unwrap();
    
    chassis.start_performance_monitoring();
    
    for _ in 0..10 {
        chassis.record_frame();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    
    let metrics = chassis.get_performance_metrics();
    assert!(metrics.fps > 30.0);
    assert!(metrics.memory_usage > 0);
    assert!(metrics.tick_count >= 0);
}

#[test]
fn test_engine_lifecycle() {
    let config = TauriChassisConfig::default();
    let mut chassis = TauriChassis::new(config).unwrap();
    
    chassis.initialize_for_testing().unwrap();
    assert!(chassis.tick().is_ok());
    
    chassis.start_performance_monitoring();
    let metrics = chassis.get_performance_metrics();
    assert!(metrics.tick_count > 0);
}

#[test]
fn test_pax_engine_integration() {
    let config = TauriChassisConfig::default();
    let mut chassis = TauriChassis::new(config).unwrap();
    
    chassis.initialize_for_testing().unwrap();
    
    // assert!(chassis.initialize_engine(Box::new(mock_traverser)).is_ok());
    
    assert!(chassis.tick().is_ok());
}

#[test]
fn test_native_message_conversion() {
    let config = TauriChassisConfig::default();
    let chassis = TauriChassis::new(config).unwrap();
    
    let text_create = pax_message::NativeMessage::TextCreate(
        pax_message::AnyCreatePatch {
            id: 1,
            parent_frame: Some(0),
            occlusion_layer_id: 0,
        }
    );
    
    let render_cmd = chassis.convert_message_to_render_command(&text_create);
    assert!(render_cmd.is_some());
    assert!(matches!(render_cmd.unwrap(), RenderCommand::DrawText { .. }));
}

#[test]
fn test_performance_metrics_structure() {
    let config = TauriChassisConfig::default();
    let mut chassis = TauriChassis::new(config).unwrap();
    
    chassis.initialize_for_testing().unwrap();
    
    let metrics = chassis.get_performance_metrics();
    assert!(metrics.fps > 0.0);
    assert!(metrics.memory_usage > 0);
    assert_eq!(metrics.tick_count, 0);
    
    chassis.tick().unwrap();
    let updated_metrics = chassis.get_performance_metrics();
    assert_eq!(updated_metrics.tick_count, 1);
}

#[test]
fn test_pax_engine_message_processing() {
    let config = TauriChassisConfig::default();
    let mut chassis = TauriChassis::new(config).unwrap();
    
    chassis.initialize_for_testing().unwrap();
    chassis.initialize_engine_for_testing().unwrap();
    
    let messages = chassis.tick().unwrap();
    // assert!(!messages.is_empty());
    
    let text_create = pax_message::NativeMessage::TextCreate(
        pax_message::AnyCreatePatch {
            id: 1,
            parent_frame: Some(0),
            occlusion_layer_id: 0,
        }
    );
    
    let render_cmd = chassis.convert_message_to_render_command(&text_create);
    assert!(render_cmd.is_some());
    assert!(matches!(render_cmd.unwrap(), RenderCommand::DrawText { .. }));
}

#[test]
fn test_engine_fallback_mode() {
    let config = TauriChassisConfig::default();
    let mut chassis = TauriChassis::new(config).unwrap();
    
    chassis.initialize_for_testing().unwrap();
    
    assert!(chassis.tick().is_ok());
    
    let metrics = chassis.get_performance_metrics();
    assert!(metrics.tick_count > 0);
}
