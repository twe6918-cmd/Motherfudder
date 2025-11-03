# Build Dependencies

This document lists the required system dependencies for building MfBuilder.

## Linux (Ubuntu/Debian)

Install OpenSSL development libraries:
```bash
sudo apt-get update
sudo apt-get install -y pkg-config libssl-dev
```

## Linux (Fedora/RHEL/CentOS)

```bash
sudo dnf install -y pkg-config openssl-devel
```

## Linux (Arch)

```bash
sudo pacman -S pkg-config openssl
```

## Windows

On Windows, you may need to:
1. Install Visual Studio Build Tools
2. Install OpenSSL from https://slproweb.com/products/Win32OpenSSL.html
3. Or use `vcpkg` to install OpenSSL:
   ```powershell
   vcpkg install openssl:x64-windows
   ```

## macOS

```bash
brew install openssl pkg-config
```

## After Installing Dependencies

Try building again:
```bash
cd MfBuilder
cargo build --release
```

## Troubleshooting

If you still encounter OpenSSL errors, you may need to set environment variables:

```bash
export OPENSSL_DIR=/usr/local/ssl
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
```

Adjust paths according to your OpenSSL installation location.
