# Tauri-Pax Quick Start Guide

## 🚀 For Next Devin Session

### 1. Environment Setup (5 minutes)
```bash
# Install system dependencies (CRITICAL)
sudo apt update
sudo apt install libjavascriptcoregtk-4.1-dev libsoup-3.0-dev

# Clone and checkout
git clone https://github.com/waluenetwork/pax.git
cd pax
git checkout TAURI_PAX
```

### 2. Verify Current Status (2 minutes)
```bash
# Check you're on the right commit
git log --oneline -3
# Should show: 08dd4af9 Implement Phase 1, Unit 1.1...

# Test current build
cd pax-chassis-tauri
cargo build --features javascript-bridge
cargo test
```

### 3. Start Phase 1, Unit 1.2 (JavaScript Bridge Implementation)
```bash
# Open key files for implementation
code src/javascript.rs    # Main implementation file
code src/lib.rs          # Integration points
code examples/basic-app/  # Test application
```

### 4. Implementation Focus
**Primary Goal**: Complete JavaScript bridge renderer in `src/javascript.rs`

**Key Methods to Implement**:
- `execute_js()` - Execute JavaScript in WebView
- `init_canvas()` - Set up Canvas element
- `render_command_to_js()` - Convert render commands to JS
- Event handling integration

### 5. Testing
```bash
# Test implementation
cargo test --features javascript-bridge

# Test example app
cd examples/basic-app/src-tauri
cargo tauri dev
```

### 6. Commit Progress
```bash
git add pax-chassis-tauri/
git commit -m "Implement Phase 1, Unit 1.2: JavaScript Bridge Implementation"
git push origin TAURI_PAX
```

## 📋 Current Status
- ✅ Phase 1, Unit 1.1 COMPLETE
- 🚧 Phase 1, Unit 1.2 READY TO START
- 📝 PR #3 created and documented

## 🎯 Next Unit Goal
Complete functional JavaScript bridge renderer that can:
- Render basic shapes via Canvas API
- Handle mouse and keyboard events
- Integrate with Tauri WebView
- Pass all unit tests

**Estimated Time**: 2-4 hours for core implementation
