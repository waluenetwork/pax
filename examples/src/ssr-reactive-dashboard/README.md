# SSR Reactive Dashboard Example

This example demonstrates Server-Side Rendering (SSR) with reactive capabilities using WebSocket communication between a FastAPI server and a Pax client application.

## Features

- **Server-Side Rendering**: Data is generated on the server and streamed to the client
- **Dynamic Template Streaming**: Real-time Pax DSL template updates via WebSocket
- **Reactive Updates**: Real-time data updates via WebSocket connection
- **Cross-platform**: Runs on Web, macOS, and iOS through Pax
- **Modern Architecture**: FastAPI backend with Pax frontend

## Architecture

```
FastAPI Server (Python) <--WebSocket--> Pax Client (Rust/WASM)
     |                                        |
     v                                        v
Random Data Generation              Reactive Property Updates
Dynamic Template Generation         Runtime Template Parsing
```

## Components

### Server (`/home/ubuntu/ssr-websocket-server/`)
- FastAPI application with WebSocket support
- Generates random dashboard data every 2 seconds
- Dynamic Pax template generation every 10 seconds
- CORS enabled for cross-origin requests
- Real-time data and template streaming to connected clients

### Client (`examples/src/ssr-reactive-dashboard/`)
- Pax application with reactive properties
- WebSocket client for real-time data consumption
- Runtime template parsing using pax_lang::parse_pax_str()
- Automatic reconnection capabilities
- Cross-platform rendering (Web/macOS/iOS)

## Data Model

The server generates dashboard items with:
- `id`: Unique identifier
- `title`: Display name (Revenue, Active Users, etc.)
- `value`: Random numeric value (10-1000)
- `trend`: Percentage change with direction indicators
- `color`: Random color selection
- `timestamp`: Current time

## Dynamic Template Streaming

This example demonstrates both data and template streaming:

- **Data Streaming**: Real-time dashboard data updates every 2 seconds
- **Template Streaming**: Dynamic Pax template updates every 10 seconds (every 5th message)

The server alternates between sending data updates and template updates, showcasing Pax's capability for runtime template parsing and dynamic UI generation.

### Message Format

```json
// Data message
{"Data": [{"id": 1, "title": "Revenue", "value": 123.45, ...}]}

// Template message  
{"Template": "<Group>...</Group>@settings{...}"}
```

### Template Parsing

The client uses `pax_lang::parse_pax_str()` for runtime template parsing:

```rust
if let Ok(ast) = parse_pax_str(Rule::pax_component_definition, template_content) {
    match pax_manifest::parsing::parse_template_from_component_definition_string(
        &mut tpc,
        template_content,
        ast,
    ) {
        Ok(_) => {
            web_sys::console::log_1(&format!("✅ Successfully parsed template: {}", template_content).into());
        }
        Err(e) => {
            web_sys::console::log_1(&format!("❌ Template parsing error: {:?}", e).into());
        }
    }
}
```

## Running the Example

### 1. Start the WebSocket Server
```bash
cd /home/ubuntu/ssr-websocket-server
poetry install
poetry run fastapi dev app/main.py --port 8090
```

### 2. Start the Pax Client
```bash
cd /home/ubuntu/repos/pax/examples/src/ssr-reactive-dashboard
/home/ubuntu/repos/pax/target/release/pax-cli run --target=web
```

### 3. Open Browser
Navigate to `http://localhost:8080` to see the reactive dashboard.

## WebSocket Communication

- **Connection**: `ws://localhost:8090/ws`
- **Data Format**: JSON with Data or Template messages
- **Update Frequency**: Every 2 seconds (data), every 10 seconds (templates)
- **Auto-reconnect**: Enabled by default

## Technical Details

### Reactive Properties
- `server_data`: Property<Vec<DataItem>> - Main dashboard data
- `connection_status`: Property<String> - WebSocket connection state
- `last_update`: Property<String> - Timestamp of last data update
- `auto_reconnect`: Property<bool> - Reconnection preference

### WebSocket Integration
- Uses `web-sys` WebSocket API for WASM compatibility
- Closure-based event handling for message processing
- JSON deserialization with serde
- Error handling and connection management

### Dynamic Template Parsing
- Uses `pax_lang::parse_pax_str()` for runtime template parsing
- `TemplateNodeParseContext` for proper template processing
- Support for both data and template WebSocket messages
- Console logging for template parsing success/failure

## Troubleshooting

### White Screen Issue
If you see a white screen, check browser console for font-related errors. The Pax template requires proper `Font::Web()` syntax with all parameters specified:

```pax
font: Font::Web("Roboto", "", FontStyle::Normal, FontWeight::Light)
```

Common issues:
- Missing font weight specification
- Incorrect ID selectors in @settings block
- Malformed template syntax from server

### WebSocket Connection
Ensure the WebSocket server is running on port 8090 before starting the Pax client. Check browser console for connection status messages.

### Template Parsing
Check browser console for template parsing success/failure messages. Dynamic templates must follow proper Pax DSL syntax.

### Font Rendering Issues
The most common cause of white screen is font specification errors. Ensure all fonts use proper syntax:
- Use `Font::Web("FontName", "", FontStyle::Normal, FontWeight::Light)`
- All ID selectors must be properly formatted: `#title { ... }`
- Check browser console for "ExtraLight" or "Light" font weight errors

### Server Template Generation
The WebSocket server generates dynamic Pax templates with proper ID selectors. If templates are malformed, the client will fail to render them. Check server logs for template generation and client console for parsing errors.

### Current Implementation Status (Final)
- ✅ WebSocket server streaming both data and Pax DSL templates
- ✅ Client receiving and parsing WebSocket messages successfully
- ✅ Dynamic template parsing using pax_lang::parse_pax_str() - **WORKING**
- ✅ Reactive Property<T> system for automatic UI updates - **WORKING**
- ✅ Font specification fixes for proper rendering (Fixed FontWeight::ExtraLight → FontWeight::Light)
- ✅ White screen issue resolved with proper font weight specifications
- ✅ Fixed malformed ID selectors in server-generated templates
- ✅ Server generates proper Pax DSL with correct @settings syntax and # ID selectors
- ✅ Enhanced template parsing with proper error handling and logging
- ✅ Comprehensive WebSocket error handling and logging
- ✅ Safe template parsing without designtime dependency - **WORKING**
- ✅ Template parsing works in all configuration modes - **VERIFIED**
- ❌ **Runtime template application to UI - NOT IMPLEMENTED**
- ❌ **Dynamic UI updates from template changes - NOT WORKING**

### What Works
1. **WebSocket Communication**: Stable bidirectional communication between FastAPI server and Pax client
2. **Data Streaming**: Real-time dashboard data updates every 2 seconds with reactive UI updates
3. **Template Parsing**: Pax DSL templates are successfully parsed using pax_lang::parse_pax_str()
4. **Error Handling**: Comprehensive logging and error handling for WebSocket and parsing operations
5. **Cross-platform**: Verified working on web target

### What Doesn't Work
1. **Template Runtime Application**: Templates are parsed but not applied to the running UI
2. **Dynamic Template Updates**: UI doesn't change when new templates are received
3. **Designtime Integration**: Would require pax-designtime dependency for runtime template application

### Technical Limitations Discovered
- Runtime template application requires `ctx.designtime.replace_template()` method
- WebSocket closures cannot easily access mutable references to component state
- Template parsing works independently but runtime integration is complex
- Designtime dependency would be needed for full template streaming functionality

## Development Notes

This example showcases:
1. **SSR Pattern**: Server generates data, client renders reactively
2. **Dynamic DSL Streaming**: Server generates and streams Pax templates
3. **Runtime Template Parsing**: Client parses Pax DSL at runtime
4. **Real-time Updates**: WebSocket streaming for live data and templates
5. **Pax Reactive System**: Property<T> for automatic UI updates
6. **Cross-platform**: Single codebase for multiple targets
7. **Modern Web Stack**: FastAPI + Pax + WebSocket + WASM

The architecture demonstrates how Pax can be used for modern web applications with real-time data requirements and dynamic template generation while maintaining cross-platform compatibility.
