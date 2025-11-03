# Welcome to Motherfudder Crypter

**Professional Windows executable crypter with dual binary support and advanced evasion techniques.**

---

## Quick Links

- **New Users**: Start with [README.md•(README.md)
- **Installation**: See [INSTALLATION.md•(MfBuilder/INSTALLATION.md)
- **Quick Start**: Follow [QUICKSTART.md•(MfBuilder/QUICKSTART.md)
- **All Documentation**: Browse [DOCUMENTATION_INDEX.md•(DOCUMENTATION_INDEX.md)

---

## What is Motherfudder•

Motherfudder is an advanced crypter for Windows executables that supports:

• **Native & .NET binaries** (x86/x64)  
• **Anti-analysis features** (debug, VM, sandbox detection)  
• **Privilege escalation** (UAC bypass)  
• **AV evasion** (AMSI bypass, Defender exclusion)  
• **Persistence** (auto-start, single instance)  
• **Multiple interfaces** (CLI & Telegram bot)  

---

## Getting Started

### 1. Read the README
Start with [README.md•(README.md) for:
- Feature overview
- Use cases
- Architecture details
- Quick examples

### 2. Install
Follow platform-specific instructions in [INSTALLATION.md•(MfBuilder/INSTALLATION.md):
- Ubuntu/Debian
- Fedora/RHEL
- Windows
- macOS
- Docker

### 3. Choose Your Mode

**Telegram Bot** (Recommended for beginners):
- Interactive configuration
- File upload/download
- No command-line needed
- Guide: [BOT_SETUP.md•(MfBuilder/BOT_SETUP.md)

**CLI Mode** (For automation):
- JSON configuration
- Batch processing
- Scriptable builds
- Guide: [QUICKSTART.md•(MfBuilder/QUICKSTART.md)

---

## Documentation Structure

### Core Documentation
- [README.md•(README.md) - Project overview and introduction
- [FEATURES.md•(FEATURES.md) - Complete feature reference
- [CHANGELOG.md•(CHANGELOG.md) - Version history

### Setup Guides
- [INSTALLATION.md•(MfBuilder/INSTALLATION.md) - Installation for all platforms
- [BUILD_GUIDE.md•(BUILD_GUIDE.md) - Building from source
- [DEPENDENCIES.md•(MfBuilder/DEPENDENCIES.md) - System requirements

### Usage Guides
- [QUICKSTART.md•(MfBuilder/QUICKSTART.md) - 5-minute quick start
- [BOT_SETUP.md•(MfBuilder/BOT_SETUP.md) - Telegram bot configuration

### Technical Documentation
- [UAC_BYPASS_INFO.md•(MfBuilder/UAC_BYPASS_INFO.md) - Privilege escalation details
- [DEFENDER_EXCLUSION_INFO.md•(MfBuilder/DEFENDER_EXCLUSION_INFO.md) - AV evasion guide

### Reference
- [DOCUMENTATION_INDEX.md•(DOCUMENTATION_INDEX.md) - Complete documentation index
- [CONTRIBUTING.md•(CONTRIBUTING.md) - Contribution guidelines
- [LICENSE•(LICENSE) - MIT License

---

## Key Features

### Silent Windows Defender Exclusion
Combine **UAC Bypass** + **Defender Exclusion** for completely silent operation:
- No user prompts
- No visible windows
- Automatic privilege escalation
- Silent Defender bypass

### Enhanced AMSI Bypass
For .NET payloads:
- Memory patching at offset +33
- Minimal 3-byte footprint
- Encrypted patch bytes
- Harder to detect than traditional methods

### Telegram Bot Interface
Interactive crypting service:
- Key-based authentication
- Automatic binary detection
- Real-time configuration
- Multi-user support
- Remote operation

### Dual Binary Support
Works with:
- Native x86/x64 executables
- .NET x86/x64 assemblies
- Automatic detection
- Optimized patches per type

---

## Quick Examples

### CLI Mode
```bash
cd MfBuilder
cargo run

# Configure build.json, add payload.exe
# Output: out.bat or out.exe
```

### Bot Mode
```bash
cd MfBuilder
cargo run -- --bot

# Use via Telegram:
# /start • authenticate • upload • configure • build
```

---

## Support & Resources

### Documentation
- Full documentation in `docs/` directory
- Indexed in [DOCUMENTATION_INDEX.md•(DOCUMENTATION_INDEX.md)
- Platform-specific guides available

### Troubleshooting
- Check relevant guide's troubleshooting section
- Review [INSTALLATION.md•(MfBuilder/INSTALLATION.md) for setup issues
- Consult [FEATURES.md•(FEATURES.md) for feature details

### Contributing
- Read [CONTRIBUTING.md•(CONTRIBUTING.md)
- Follow coding standards
- Submit via pull request

---

## Legal Notice

 **For Educational and Authorized Testing Only**

This tool is provided for legitimate security testing, penetration testing, and educational purposes. Users must:

- Obtain proper authorization before testing any systems
- Comply with applicable laws and regulations
- Use responsibly and ethically
- Understand legal implications in their jurisdiction

Unauthorized use is illegal and unethical. See [LICENSE•(LICENSE) for full terms.

---

## Author & Credits

**Developed by Florin**

### Techniques & Frameworks
- AMSI Bypass: [Chainski/GlobalAMSIBypass•(https://github.com/Chainski/GlobalAMSIBypass)
- Telegram Bot: [teloxide•(https://github.com/teloxide/teloxide)
- Binary Parsing: [goblin•(https://github.com/m4b/goblin)
- UAC Bypass: fodhelper.exe technique

---

## Next Steps

1. **Read** [README.md•(README.md) for comprehensive overview
2. **Install** following [INSTALLATION.md•(MfBuilder/INSTALLATION.md)
3. **Start** with [QUICKSTART.md•(MfBuilder/QUICKSTART.md)
4. **Explore** advanced features in [FEATURES.md•(FEATURES.md)

---

## Project Status

- **Version**: 1.0.0
- **Status**: Production Ready
- **License**: MIT
- **Platform**: Windows (build), Linux/macOS (bot mode)
- **Language**: Rust + C#

---

## Quick Command Reference

```bash
# Build from source
cd MfBuilder && cargo build --release

# CLI mode
./MfBuilder

# Bot mode
./MfBuilder --bot

# Help
./MfBuilder --help
```

---

**For complete documentation, start with [README.md•(README.md) or browse [DOCUMENTATION_INDEX.md•(DOCUMENTATION_INDEX.md)**

• **If this project helps you, please consider leaving a star!**
