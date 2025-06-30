# SSR Reactive Dashboard Example

This example demonstrates Server-Side Rendering (SSR) with reactive features using WebSocket communication between a Pax client and a FastAPI server.

## Features
- Real-time data streaming via WebSocket
- Reactive UI updates using Pax Property<T> system
- Connection status monitoring
- Auto-reconnection capability
- Dynamic data visualization

## Running the Example

1. Start the WebSocket server:
```bash
cd /path/to/ssr-websocket-server
poetry run fastapi dev app/main.py
```

2. Start the Pax client:
```bash
cd /path/to/pax/examples/src/ssr-reactive-dashboard
pax-cli run --target=web
```

3. Open http://localhost:8080 in your browser

## Architecture
- **Server**: FastAPI with WebSocket endpoint generating dynamic data
- **Client**: Pax WASM app with reactive Property<T> state management
- **Communication**: JSON over WebSocket for real-time updates
