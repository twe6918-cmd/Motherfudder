# Release Notes - Version 1.0.0

**Motherfudder Crypter** - Production Release

---

## Release Information

- **Version**: 1.0.0
- **Release Date**: 2025-11-03
- **Status**: Production Ready
- **License**: MIT

---

## What's New

### Major Features

#### Windows Defender Silent Exclusion
- Automatic Windows Defender exclusions
- Silent operation when combined with UAC bypass
- No user prompts or visible windows
- PowerShell-based implementation

#### Enhanced AMSI Bypass
- Memory patching at offset +33 within AmsiScanBuffer
- Minimal 3-byte footprint (xor rbx, rbx)
- RC4 encrypted patch bytes
- More evasive than traditional prologue patches
- Applies to .NET payloads only

#### Telegram Bot Interface
- Interactive configuration via Telegram
- Key-based authentication system
- Automatic binary type detection
- Real-time feature toggling
- Multi-user session management
- File upload/download support

#### Dual Mode Operation
- **CLI Mode**: Traditional JSON configuration
- **Bot Mode**: Interactive Telegram interface
- Single binary, dual functionality

---

## Core Capabilities

### Binary Support
- ? Native x86 executables
- ? Native x64 executables
- ? .NET x86 assemblies
- ? .NET x64 assemblies
- ? Automatic type detection

### Anti-Analysis
- ? Anti-Debug (thread-based detection)
- ? Anti-VM (hardware fingerprinting)
- ? Geographic restrictions (CIS blocking)

### Evasion Techniques
- ? AMSI bypass (.NET only)
- ? ETW patching (.NET only)
- ? Windows Defender exclusions
- ? Indirect syscalls (Native only)
- ? String encryption (RC4)

### Privilege & Persistence
- ? UAC bypass (fodhelper.exe)
- ? Scheduled task persistence
- ? Single instance management
- ? Auto-start on boot

### Output Formats
- ? BAT (batch wrapper - more evasive)
- ? EXE (direct executable - cleaner)

---

## Documentation Suite

### User Documentation (13 Files)

#### Getting Started
1. **README_FIRST.md** - Start here for new users
2. **README.md** - Complete project overview
3. **QUICKSTART.md** - 5-minute quick start guide
4. **INSTALLATION.md** - Platform-specific installation

#### Feature Documentation
5. **FEATURES.md** - Complete feature reference
6. **UAC_BYPASS_INFO.md** - Privilege escalation details
7. **DEFENDER_EXCLUSION_INFO.md** - AV evasion guide

#### Setup Guides
8. **BUILD_GUIDE.md** - Building from source
9. **BOT_SETUP.md** - Telegram bot deployment
10. **DEPENDENCIES.md** - System requirements

#### Project Information
11. **CHANGELOG.md** - Version history
12. **CONTRIBUTING.md** - Contribution guidelines
13. **DOCUMENTATION_INDEX.md** - Complete documentation index

### Additional Files
- **LICENSE** - MIT License
- **RELEASE_NOTES.md** - This file
- **.gitignore** - Protected files list
- **.env.example** - Bot configuration template
- **build.json** - CLI configuration template

---

## Technical Specifications

### Architecture

```
Rust Builder (MfBuilder)
??? Binary Detection (goblin)
??? Encryption (RC4, SHA-256)
??? Stub Configuration
??? Build Orchestration

C# Stub (MfRunner)
??? Payload Decryption
??? Anti-Analysis
??? Privilege Escalation
??? AV Evasion
??? Payload Execution

.NET Obfuscator (MfObfDotNet)
??? String Encryption
??? Integer Obfuscation
??? Method Renaming
??? Control Flow Obfuscation
```

### Encryption

- **Algorithm**: RC4
- **Key Derivation**: SHA-256 based
- **Seeds**: Random per build
- **Encrypted Elements**:
  - Payload bytes
  - Patch bytes
  - URL strings
  - Configuration data

### Build Process

```
1. Binary Type Detection
2. Key Generation (random seeds)
3. Stub Configuration (preprocessor symbols)
4. Payload Encryption (RC4)
5. Native Conversion (Donut + SGN) [if native]
6. Stub Compilation (MSBuild)
7. .NET Obfuscation [if .NET stub]
8. Wrapper Generation (BAT/EXE)
```

---

## System Requirements

### Build Environment
- **OS**: Windows 10/11
- **RAM**: 2GB minimum, 4GB recommended
- **Disk**: 1GB free space
- **Software**:
  - Rust 1.70.0+
  - .NET SDK 6.0+
  - Visual Studio Build Tools
  - MSBuild
  - OpenSSL development libraries

### Deployment Environment
- **OS**: Windows 10/11 (target), Linux/macOS (bot mode)
- **Network**: Internet (bot mode, payload download)
- **Permissions**: User (non-admin) or Administrator

---

## Installation

### Quick Install (Ubuntu/Debian)

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y pkg-config libssl-dev build-essential

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone <repository-url>
cd MfBuilder
cargo build --release
```

### Quick Install (Windows)

```powershell
# Install Rust from https://rustup.rs/

# Install Visual Studio Build Tools

# Clone and build
git clone <repository-url>
cd MfBuilder
cargo build --release
```

See [INSTALLATION.md](MfBuilder/INSTALLATION.md) for detailed instructions.

---

## Usage

### CLI Mode

```bash
# Configure
edit build.json

# Add payload
cp target.exe payload.exe

# Build
./MfBuilder

# Output: out.bat or out.exe
```

### Bot Mode

```bash
# Configure (one-time)
cp .env.example .env
# Add TELOXIDE_TOKEN

# Start bot
./MfBuilder --bot

# Use via Telegram
# /start ? authenticate ? upload ? configure ? build
```

See [QUICKSTART.md](MfBuilder/QUICKSTART.md) for detailed usage.

---

## Configuration

### Example Configuration (Maximum Stealth)

```json
{
    "file_extension": "BAT",
    "anti_debug": true,
    "anti_virtual_machine": true,
    "blacklist_cis_countries": false,
    "uac_bypass": true,
    "single_instance": true,
    "run_on_startup": true,
    "defender_exclusion": true,
    "binder": false
}
```

**Result**: Silent privilege escalation + silent Defender exclusion + persistence

---

## Known Issues & Limitations

### Platform Limitations
- Build environment must be Windows (due to MSBuild requirement)
- Target execution environment must be Windows
- Bot mode can run on Linux/macOS

### Feature Limitations
- UAC bypass requires user in Administrators group
- Defender exclusion requires admin rights (or UAC bypass)
- AMSI/ETW patches apply to .NET payloads only
- Native payload support requires Donut and SGN

### Compatibility
- Tested on Windows 10 (all versions) and Windows 11
- UAC bypass tested on Windows 10 21H2+ and Windows 11
- May not work on heavily restricted enterprise environments

---

## Security Considerations

### Operational Security

**For Users**:
- Change default bot authentication key
- Protect `.env` file (never commit)
- Use strong bot tokens
- Monitor bot access logs
- Rotate authentication keys periodically

**For Defenders**:
- Monitor Event ID 5007 (Defender changes)
- Enable PowerShell script block logging
- Implement application whitelisting
- Deploy EDR with behavioral detection
- Monitor registry changes to ms-settings

### Legal & Ethical Use

?? **This tool is for educational and authorized testing only**

Users must:
- Obtain explicit authorization before testing
- Comply with applicable laws (CFAA, GDPR, etc.)
- Use responsibly and ethically
- Understand legal implications
- Never use for malicious purposes

Unauthorized access to computer systems is illegal.

---

## Troubleshooting

### Common Issues

**Build fails with OpenSSL error**:
```bash
sudo apt-get install libssl-dev pkg-config
```

**Bot not responding**:
- Verify TELOXIDE_TOKEN in `.env`
- Ensure running with `--bot` flag
- Check network connectivity

**UAC bypass not working**:
- User must be in Administrators group
- UAC must be enabled
- Windows version must be 10/11

See documentation for complete troubleshooting guides.

---

## Changelog

### Version 1.0.0 (2025-11-03)

**Added**:
- Windows Defender silent exclusion feature
- Telegram bot interface with authentication
- Enhanced AMSI bypass (offset +33 patching)
- Dual mode operation (CLI + Bot)
- Comprehensive documentation suite (13 files)
- Interactive configuration menu (bot mode)
- Automatic binary type detection
- Multi-user session management

**Changed**:
- AMSI patch reduced to 3 bytes (from 11)
- AMSI/ETW patches now .NET-only (documented)
- Improved memory protection handling
- Refactored build architecture for dual mode

**Fixed**:
- AMSI bypass evasion improved
- Memory protection restoration added
- Configuration validation enhanced

See [CHANGELOG.md](CHANGELOG.md) for detailed history.

---

## Credits

### Developer
**Florin** - Project author and maintainer

### Techniques & Research
- **AMSI Bypass**: Based on [Chainski/GlobalAMSIBypass](https://github.com/Chainski/GlobalAMSIBypass)
- **UAC Bypass**: fodhelper.exe registry hijacking technique
- **Windows Internals**: Community research and documentation

### Frameworks & Libraries
- **Rust**: Primary language
- **teloxide**: Telegram bot framework
- **goblin**: PE file parsing
- **MSBuild**: C# compilation
- **.NET Framework**: Stub runtime

### Community
Special thanks to the security research community for continuous work in understanding Windows internals and evasion techniques.

---

## Support

### Documentation
- Complete documentation in repository
- Indexed in [DOCUMENTATION_INDEX.md](DOCUMENTATION_INDEX.md)
- Platform-specific guides available
- Troubleshooting sections included

### Issues
For bugs or feature requests:
1. Check existing documentation
2. Search existing issues
3. Open new issue with details
4. Provide reproduction steps

### Contributing
Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file.

Copyright (c) 2025 Florin

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.

---

## Disclaimer

This tool is provided for educational and authorized security testing purposes only. The author and contributors are not responsible for any misuse or damage caused by this software. Users must ensure they have explicit authorization before testing any systems they do not own.

**By using this software, you acknowledge that you understand these risks and agree to use it responsibly and legally.**

---

## Next Steps

1. **Read**: [README_FIRST.md](README_FIRST.md) for orientation
2. **Install**: Follow [INSTALLATION.md](MfBuilder/INSTALLATION.md)
3. **Quick Start**: Use [QUICKSTART.md](MfBuilder/QUICKSTART.md)
4. **Explore**: Review [FEATURES.md](FEATURES.md)

---

? **If this project helps you, please consider leaving a star on the repository!**

**Version 1.0.0 - Production Release - Ready for the World** ??
