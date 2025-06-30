# Pax Release Process / Pax Yayın Süreci

## English

### Creating GitHub Releases

This document outlines the process for creating GitHub releases of Pax with pre-built binaries.

#### Prerequisites

**Development Environment:**
- Complete local development setup (see [LOCAL_DEVELOPMENT.md](LOCAL_DEVELOPMENT.md))
- GitHub CLI (`gh`) installed and authenticated
- Cross-compilation toolchains for target platforms

**Required Toolchains:**
```bash
# Add target platforms for cross-compilation
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu
```

#### Release Workflow

1. **Prepare Release Branch**
```bash
# Create release branch
git checkout dev
git pull origin dev
git checkout -b release/v0.38.4  # Update version number

# Update version numbers in Cargo.toml files
# Update CHANGELOG.md with release notes
```

2. **Build Cross-Platform Binaries**
```bash
# Ensure web interface is built
cd pax-compiler/files/interfaces/web
./build-interface.sh

# Build for different platforms
cd ../../../

# Linux (x86_64)
cargo build --release --target x86_64-unknown-linux-gnu -p pax-cli
cp target/x86_64-unknown-linux-gnu/release/pax-cli releases/pax-cli-linux-x86_64

# macOS (Intel)
cargo build --release --target x86_64-apple-darwin -p pax-cli
cp target/x86_64-apple-darwin/release/pax-cli releases/pax-cli-macos-x86_64

# macOS (Apple Silicon)
cargo build --release --target aarch64-apple-darwin -p pax-cli
cp target/aarch64-apple-darwin/release/pax-cli releases/pax-cli-macos-aarch64

# Windows
cargo build --release --target x86_64-pc-windows-gnu -p pax-cli
cp target/x86_64-pc-windows-gnu/release/pax-cli.exe releases/pax-cli-windows-x86_64.exe
```

3. **Create Release Package**
```bash
# Create release directory structure
mkdir -p releases/v0.38.4
cd releases/v0.38.4

# Copy binaries
cp ../pax-cli-* .

# Create installation scripts
cat > install-linux.sh << 'EOF'
#!/bin/bash
chmod +x pax-cli-linux-x86_64
sudo mv pax-cli-linux-x86_64 /usr/local/bin/pax-cli
echo "Pax CLI installed successfully!"
EOF

cat > install-macos.sh << 'EOF'
#!/bin/bash
# Detect architecture
if [[ $(uname -m) == "arm64" ]]; then
    chmod +x pax-cli-macos-aarch64
    sudo mv pax-cli-macos-aarch64 /usr/local/bin/pax-cli
else
    chmod +x pax-cli-macos-x86_64
    sudo mv pax-cli-macos-x86_64 /usr/local/bin/pax-cli
fi
echo "Pax CLI installed successfully!"
EOF

# Create README for release
cat > README.md << 'EOF'
# Pax CLI Release v0.38.4

## Installation

### Linux
```bash
wget https://github.com/waluenetwork/pax/releases/download/v0.38.4/pax-cli-linux-x86_64
chmod +x pax-cli-linux-x86_64
sudo mv pax-cli-linux-x86_64 /usr/local/bin/pax-cli
```

### macOS
```bash
# Intel Macs
wget https://github.com/waluenetwork/pax/releases/download/v0.38.4/pax-cli-macos-x86_64
chmod +x pax-cli-macos-x86_64
sudo mv pax-cli-macos-x86_64 /usr/local/bin/pax-cli

# Apple Silicon Macs
wget https://github.com/waluenetwork/pax/releases/download/v0.38.4/pax-cli-macos-aarch64
chmod +x pax-cli-macos-aarch64
sudo mv pax-cli-macos-aarch64 /usr/local/bin/pax-cli
```

### Windows
Download `pax-cli-windows-x86_64.exe` and add to your PATH.

## Usage

```bash
# Create new project
pax-cli create my-project
cd my-project

# Run with Pax Designer
pax-cli run --target=web
```

## What's New in v0.38.4

- Fixed local source code build process
- Improved embedded template extraction
- Enhanced documentation for developers
- Better cross-platform compatibility

For complete documentation, visit: https://docs.pax.dev/
EOF
```

4. **Create GitHub Release**
```bash
# Commit release changes
git add .
git commit -m "Prepare release v0.38.4"
git push origin release/v0.38.4

# Create GitHub release
gh release create v0.38.4 \
    --title "Pax v0.38.4 - Enhanced Local Development" \
    --notes-file releases/v0.38.4/README.md \
    --target release/v0.38.4 \
    releases/v0.38.4/pax-cli-*

# Merge release branch
git checkout dev
git merge release/v0.38.4
git push origin dev
```

#### Release Checklist

- [ ] Version numbers updated in all Cargo.toml files
- [ ] CHANGELOG.md updated with release notes
- [ ] All examples tested with new build
- [ ] Cross-platform binaries built and tested
- [ ] Installation scripts created and tested
- [ ] GitHub release created with proper assets
- [ ] Documentation updated with release information
- [ ] Community notified (Discord, social media)

#### Automated Release Script

```bash
#!/bin/bash
# scripts/create-release.sh

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 v0.38.4"
    exit 1
fi

echo "Creating release $VERSION..."

# Build web interface
cd pax-compiler/files/interfaces/web
./build-interface.sh
cd ../../../

# Create release directory
mkdir -p releases/$VERSION
cd releases/$VERSION

# Build cross-platform binaries
echo "Building Linux binary..."
cargo build --release --target x86_64-unknown-linux-gnu -p pax-cli
cp ../../target/x86_64-unknown-linux-gnu/release/pax-cli pax-cli-linux-x86_64

echo "Building macOS binaries..."
cargo build --release --target x86_64-apple-darwin -p pax-cli
cp ../../target/x86_64-apple-darwin/release/pax-cli pax-cli-macos-x86_64

cargo build --release --target aarch64-apple-darwin -p pax-cli
cp ../../target/aarch64-apple-darwin/release/pax-cli pax-cli-macos-aarch64

echo "Building Windows binary..."
cargo build --release --target x86_64-pc-windows-gnu -p pax-cli
cp ../../target/x86_64-pc-windows-gnu/release/pax-cli.exe pax-cli-windows-x86_64.exe

echo "Release $VERSION prepared in releases/$VERSION/"
echo "Run 'gh release create $VERSION' to publish"
```

---

## Türkçe

### GitHub Yayınları Oluşturma

Bu belge, önceden derlenmiş ikili dosyalarla Pax'ın GitHub yayınlarını oluşturma sürecini açıklar.

#### Ön Gereksinimler

**Geliştirme Ortamı:**
- Tam yerel geliştirme kurulumu ([LOCAL_DEVELOPMENT.md](LOCAL_DEVELOPMENT.md) bakın)
- GitHub CLI (`gh`) kurulu ve kimlik doğrulaması yapılmış
- Hedef platformlar için çapraz derleme araç zincirleri

#### Yayın İş Akışı

1. **Yayın Dalı Hazırlama**
2. **Çapraz Platform İkili Dosyaları Derleme**
3. **Yayın Paketi Oluşturma**
4. **GitHub Yayını Oluşturma**

Detaylı adımlar için yukarıdaki İngilizce bölümü takip edin.

#### Otomatik Yayın Betiği

Yayın sürecini otomatikleştirmek için `scripts/create-release.sh` betiğini kullanın.

```bash
chmod +x scripts/create-release.sh
./scripts/create-release.sh v0.38.4
```
