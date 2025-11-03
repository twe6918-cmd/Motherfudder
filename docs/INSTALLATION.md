# Installation Guide

This guide provides detailed installation instructions for Motherfudder Crypter across different platforms and configurations.

---

## System Requirements

### Minimum Requirements

- **Operating System**: Windows 10/11 (for building), Linux/macOS (for bot-only deployment)
- **RAM**: 2GB minimum, 4GB recommended
- **Disk Space**: 1GB free space
- **Network**: Internet connection (for dependencies and bot mode)

### Software Requirements

#### Core Dependencies

1. **Rust Toolchain** (1.70.0 or later)
   - rustc
   - cargo
   - rustfmt (recommended)

2. **MSBuild** (for Windows builds)
   - Visual Studio Build Tools 2019 or later
   - .NET Framework 4.7.2 or later

3. **OpenSSL Development Libraries**
   - libssl-dev (Linux)
   - openssl-devel (Fedora/RHEL)
   - OpenSSL 1.1.1 or later

#### Optional Dependencies

- **Git** - For cloning repository
- **Telegram Account** - For bot mode operation

---

## Platform-Specific Installation

### Ubuntu / Debian

#### 1. Install System Dependencies

```bash
# Update package lists
sudo apt-get update

# Install build essentials
sudo apt-get install -y build-essential pkg-config libssl-dev git

# Install .NET SDK (for building MfRunner)
wget https://dot.net/v1/dotnet-install.sh
chmod +x dotnet-install.sh
./dotnet-install.sh --channel 6.0
```

#### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 3. Verify Installation

```bash
rustc --version
cargo --version
dotnet --version
openssl version
```

#### 4. Build Project

```bash
git clone <repository-url>
cd MfBuilder
cargo build --release
```

---

### Fedora / RHEL / CentOS

#### 1. Install System Dependencies

```bash
# Install build tools
sudo dnf groupinstall "Development Tools"

# Install dependencies
sudo dnf install -y pkg-config openssl-devel git

# Install .NET SDK
sudo dnf install dotnet-sdk-6.0
```

#### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 3. Build Project

```bash
git clone <repository-url>
cd MfBuilder
cargo build --release
```

---

### Arch Linux

#### 1. Install Dependencies

```bash
# Install required packages
sudo pacman -S base-devel pkg-config openssl git dotnet-sdk

# Install Rust
sudo pacman -S rust
```

#### 2. Build Project

```bash
git clone <repository-url>
cd MfBuilder
cargo build --release
```

---

### Windows

#### 1. Install Visual Studio Build Tools

Download and install from: https://visualstudio.microsoft.com/downloads/

Select the following workloads:
- .NET desktop development
- Desktop development with C++

#### 2. Install Rust

Download and run: https://rustup.rs/

```powershell
# Verify installation
rustc --version
cargo --version
```

#### 3. Install OpenSSL (Option A: vcpkg)

```powershell
# Install vcpkg
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat

# Install OpenSSL
.\vcpkg install openssl:x64-windows

# Set environment variable
$env:OPENSSL_DIR = "C:\path\to\vcpkg\installed\x64-windows"
```

#### 4. Install OpenSSL (Option B: Pre-built)

Download from: https://slproweb.com/products/Win32OpenSSL.html

Install to: `C:\Program Files\OpenSSL-Win64\`

Set environment variables:
```powershell
$env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"
```

#### 5. Build Project

```powershell
git clone <repository-url>
cd MfBuilder
cargo build --release
```

---

### macOS

#### 1. Install Homebrew

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

#### 2. Install Dependencies

```bash
# Install OpenSSL and pkg-config
brew install openssl pkg-config

# Install .NET SDK
brew install --cask dotnet-sdk
```

#### 3. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 4. Set Environment Variables

```bash
echo 'export PKG_CONFIG_PATH="/usr/local/opt/openssl/lib/pkgconfig"' >> ~/.zshrc
source ~/.zshrc
```

#### 5. Build Project

```bash
git clone <repository-url>
cd MfBuilder
cargo build --release
```

---

## Post-Installation Setup

### Telegram Bot Configuration (Optional)

If you plan to use bot mode:

#### 1. Create Telegram Bot

1. Open Telegram and search for [@BotFather•(https://t.me/BotFather)
2. Send `/newbot` command
3. Follow prompts to set bot name and username
4. Copy the provided bot token

#### 2. Configure Environment

```bash
cd MfBuilder
cp .env.example .env
```

Edit `.env`:
```env
TELOXIDE_TOKEN=YOUR_BOT_TOKEN_HERE
```

#### 3. Configure Authentication

Edit `src/telegram_bot.rs` (line 70):
```rust
const VALID_KEY: &str = "YourSecureKeyHere"; // Change this!
```

#### 4. Test Bot

```bash
./target/release/MfBuilder --bot
```

---

## Verification

### Test CLI Mode

```bash
cd MfBuilder

# Create test configuration
cat > build.json << EOF
{
    "file_extension": "EXE",
    "anti_debug": false,
    "anti_virtual_machine": false,
    "blacklist_cis_countries": false,
    "uac_bypass": false,
    "single_instance": false,
    "run_on_startup": false,
    "defender_exclusion": false,
    "binder": false
}
EOF

# Place test payload (any .exe)
cp /path/to/test.exe payload.exe

# Run build
./target/release/MfBuilder
```

Expected output:
- Build process completes successfully
- `out.exe` or `out.bat` created

### Test Bot Mode

```bash
./target/release/MfBuilder --bot
```

Expected output:
```
[INFO• Starting MfBuilder Telegram Bot...
```

Test in Telegram:
1. Find your bot
2. Send `/start`
3. Bot should respond with welcome message

---

## Troubleshooting

### Common Issues

#### "Could not find openssl"

**Linux:**
```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# Fedora/RHEL
sudo dnf install openssl-devel pkg-config
```

**macOS:**
```bash
brew install openssl pkg-config
export PKG_CONFIG_PATH="/usr/local/opt/openssl/lib/pkgconfig"
```

**Windows:**
- Ensure OpenSSL is installed
- Set `OPENSSL_DIR` environment variable
- Restart terminal after setting variables

#### "MSBuild not found"

**Windows:**
- Install Visual Studio Build Tools
- Add MSBuild to PATH: `C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\MSBuild\Current\Bin`

**Linux:**
- Install Mono: `sudo apt-get install mono-complete`
- Or use .NET SDK's MSBuild: ensure dotnet is in PATH

#### Rust Compilation Errors

```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

#### Permission Denied

```bash
# Linux/macOS
chmod +x target/release/MfBuilder

# Or run with sudo if needed (not recommended)
```

---

## Updating

### Update Rust Dependencies

```bash
cd MfBuilder
cargo update
cargo build --release
```

### Update System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get upgrade libssl-dev pkg-config
```

**Fedora/RHEL:**
```bash
sudo dnf update openssl-devel pkg-config
```

---

## Uninstallation

### Remove Binary

```bash
cd MfBuilder
cargo clean
rm -rf target/
```

### Remove Rust (Optional)

```bash
rustup self uninstall
```

### Remove System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get remove libssl-dev pkg-config
sudo apt-get autoremove
```

**Fedora/RHEL:**
```bash
sudo dnf remove openssl-devel pkg-config
```

---

## Docker Deployment (Advanced)

For isolated bot deployment:

```dockerfile
FROM rust:latest

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy project
WORKDIR /app
COPY . .

# Build
RUN cargo build --release

# Set environment
ENV TELOXIDE_TOKEN=""

# Run bot
CMD ["./target/release/MfBuilder", "--bot"•
```

Build and run:
```bash
docker build -t mfbuilder-bot .
docker run -e TELOXIDE_TOKEN=your_token mfbuilder-bot
```

---

## Next Steps

After successful installation:

1. Read [QUICKSTART.md•(QUICKSTART.md) for usage examples
2. Review [BOT_SETUP.md•(BOT_SETUP.md) for bot configuration
3. Consult [../README.md•(../README.md) for feature documentation
4. Check [CHANGELOG.md•(../CHANGELOG.md) for version information

---

## Support

For installation issues:
- Check troubleshooting section above
- Review system requirements
- Ensure all dependencies are installed
- Verify environment variables are set correctly

For bugs or feature requests, please open an issue in the repository.
