# Pax Local Development Guide / Pax Yerel Geliştirme Kılavuzu

## English

### Building and Running Pax from Source Code

This guide explains how to build and run Pax applications using the local source code instead of the published version.

#### Prerequisites

**System Dependencies:**
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y pkg-config libglib2.0-dev cmake \
    libfreetype6-dev libfontconfig1-dev \
    libpango1.0-dev libcairo2-dev libatk1.0-dev \
    libgdk-pixbuf2.0-dev libgtk-3-dev

# macOS
brew install pkg-config cmake freetype fontconfig pango cairo atk gdk-pixbuf gtk+3
```

**Rust Toolchain:**
```bash
# Install Rust 1.88.0+ (required for edition2024 support)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
rustup default stable

# Install wasm-pack for web target
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

**Node.js Dependencies:**
```bash
# Install Node.js and esbuild
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install node
npm install -g esbuild
```

#### Build Process

**Important:** The build order is critical for embedded template extraction to work correctly.

1. **Clone the repository:**
```bash
git clone https://github.com/waluenetwork/pax.git
cd pax
```

2. **Build web interface files first:**
```bash
cd pax-compiler/files/interfaces/web
./build-interface.sh
```

3. **Build pax-cli:**
```bash
cd ../../../  # Back to workspace root
cargo build --release -p pax-cli
```

4. **Verify the build:**
```bash
ls -la target/release/pax-cli
# Should show the executable file
```

#### Running Examples

Use the locally built pax-cli to run examples:

```bash
# Increment example
cd examples/src/increment
/path/to/pax/target/release/pax-cli run --target=web

# Calculator example  
cd ../calculator
/path/to/pax/target/release/pax-cli run --target=web

# Fireworks example
cd ../fireworks
/path/to/pax/target/release/pax-cli run --target=web
```

#### Creating New Projects

```bash
# Create a new project
/path/to/pax/target/release/pax-cli create my-project
cd my-project
/path/to/pax/target/release/pax-cli run --target=web
```

#### Development Workflow

1. Make changes to Pax source code
2. If you modified web interface files, run `./build-interface.sh`
3. Rebuild pax-cli: `cargo build --release -p pax-cli`
4. Test with examples or your projects

#### Troubleshooting

**Problem:** `pax-interface-web.js` 404 errors
**Solution:** Run `./build-interface.sh` before building pax-cli

**Problem:** JavaScript syntax errors in browser
**Solution:** Ensure the index.html template loads `pax-interface-web.js` correctly

**Problem:** Compilation errors about edition2024
**Solution:** Update Rust to version 1.88.0 or later

---

## Türkçe

### Pax'ı Kaynak Koddan Derleme ve Çalıştırma

Bu kılavuz, yayınlanan sürüm yerine yerel kaynak kodunu kullanarak Pax uygulamalarının nasıl derleneceğini ve çalıştırılacağını açıklar.

#### Ön Gereksinimler

**Sistem Bağımlılıkları:**
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y pkg-config libglib2.0-dev cmake \
    libfreetype6-dev libfontconfig1-dev \
    libpango1.0-dev libcairo2-dev libatk1.0-dev \
    libgdk-pixbuf2.0-dev libgtk-3-dev

# macOS
brew install pkg-config cmake freetype fontconfig pango cairo atk gdk-pixbuf gtk+3
```

**Rust Araç Zinciri:**
```bash
# Rust 1.88.0+ kurulumu (edition2024 desteği için gerekli)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
rustup default stable

# Web hedefi için wasm-pack kurulumu
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

**Node.js Bağımlılıkları:**
```bash
# Node.js ve esbuild kurulumu
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install node
npm install -g esbuild
```

#### Derleme Süreci

**Önemli:** Gömülü şablon çıkarımının doğru çalışması için derleme sırası kritiktir.

1. **Repository'yi klonlayın:**
```bash
git clone https://github.com/waluenetwork/pax.git
cd pax
```

2. **Önce web arayüz dosyalarını derleyin:**
```bash
cd pax-compiler/files/interfaces/web
./build-interface.sh
```

3. **pax-cli'yi derleyin:**
```bash
cd ../../../  # Workspace root'a geri dön
cargo build --release -p pax-cli
```

4. **Derlemeyi doğrulayın:**
```bash
ls -la target/release/pax-cli
# Çalıştırılabilir dosyayı göstermeli
```

#### Örnekleri Çalıştırma

Yerel olarak derlenmiş pax-cli'yi kullanarak örnekleri çalıştırın:

```bash
# Increment örneği
cd examples/src/increment
/path/to/pax/target/release/pax-cli run --target=web

# Calculator örneği
cd ../calculator
/path/to/pax/target/release/pax-cli run --target=web

# Fireworks örneği
cd ../fireworks
/path/to/pax/target/release/pax-cli run --target=web
```

#### Yeni Proje Oluşturma

```bash
# Yeni proje oluştur
/path/to/pax/target/release/pax-cli create my-project
cd my-project
/path/to/pax/target/release/pax-cli run --target=web
```

#### Geliştirme İş Akışı

1. Pax kaynak kodunda değişiklik yapın
2. Web arayüz dosyalarını değiştirdiyseniz, `./build-interface.sh` çalıştırın
3. pax-cli'yi yeniden derleyin: `cargo build --release -p pax-cli`
4. Örnekler veya projelerinizle test edin

#### Sorun Giderme

**Sorun:** `pax-interface-web.js` 404 hataları
**Çözüm:** pax-cli derlemeden önce `./build-interface.sh` çalıştırın

**Sorun:** Tarayıcıda JavaScript sözdizimi hataları
**Çözüm:** index.html şablonunun `pax-interface-web.js` dosyasını doğru yüklediğinden emin olun

**Sorun:** edition2024 hakkında derleme hataları
**Çözüm:** Rust'ı 1.88.0 veya daha yeni sürüme güncelleyin
