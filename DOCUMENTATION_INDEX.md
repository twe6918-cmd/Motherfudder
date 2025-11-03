# Documentation Index

Complete guide to Motherfudder Crypter documentation.

---

## Quick Navigation

### Getting Started
1. [README.md](README.md) - **Start here** - Project overview and introduction
2. [INSTALLATION.md](MfBuilder/INSTALLATION.md) - Platform-specific installation instructions
3. [QUICKSTART.md](MfBuilder/QUICKSTART.md) - Get operational in 5 minutes

### Core Documentation
4. [FEATURES.md](FEATURES.md) - Comprehensive feature documentation
5. [CHANGELOG.md](CHANGELOG.md) - Version history and updates

### Interface Guides
6. [BOT_SETUP.md](MfBuilder/BOT_SETUP.md) - Telegram bot deployment and configuration
7. [BOT_TECHNICAL.md](MfBuilder/BOT_TECHNICAL.md) - Bot technical details and VPS deployment
8. [build.json](MfBuilder/build.json) - CLI mode configuration reference

### Technical Documentation
9. [UAC_BYPASS_INFO.md](MfBuilder/UAC_BYPASS_INFO.md) - UAC bypass technique details
10. [DEFENDER_EXCLUSION_INFO.md](MfBuilder/DEFENDER_EXCLUSION_INFO.md) - Windows Defender evasion guide
11. [DEPENDENCIES.md](MfBuilder/DEPENDENCIES.md) - System dependency installation

### Project Information
12. [LICENSE](LICENSE) - Project license
13. [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
14. [RELEASE_NOTES.md](RELEASE_NOTES.md) - Version 1.0.0 release information
15. [FUTURE_ENHANCEMENTS.md](FUTURE_ENHANCEMENTS.md) - Potential improvements and research

---

## Documentation by Use Case

### New Users

**First Time Setup**:
1. Read [README.md](README.md) for overview
2. Follow [INSTALLATION.md](MfBuilder/INSTALLATION.md) for platform
3. Use [QUICKSTART.md](MfBuilder/QUICKSTART.md) to get running

### Telegram Bot Users

**Bot Deployment**:
1. Complete installation per [INSTALLATION.md](MfBuilder/INSTALLATION.md)
2. Follow [BOT_SETUP.md](MfBuilder/BOT_SETUP.md) for configuration
3. Read [BOT_TECHNICAL.md](MfBuilder/BOT_TECHNICAL.md) for VPS deployment
4. Reference [QUICKSTART.md](MfBuilder/QUICKSTART.md) for usage

### CLI Users

**Command-Line Operation**:
1. Install per [INSTALLATION.md](MfBuilder/INSTALLATION.md)
2. Configure `build.json`
3. Reference [QUICKSTART.md](MfBuilder/QUICKSTART.md) for workflow

### Advanced Users

**Technical Details**:
1. [FEATURES.md](FEATURES.md) - All features explained (includes what's NOT included)
2. [BOT_TECHNICAL.md](MfBuilder/BOT_TECHNICAL.md) - Bot internals and VPS deployment
3. [UAC_BYPASS_INFO.md](MfBuilder/UAC_BYPASS_INFO.md) - Privilege escalation
4. [DEFENDER_EXCLUSION_INFO.md](MfBuilder/DEFENDER_EXCLUSION_INFO.md) - AV evasion
5. Source code in `MfBuilder/src/` and `MfBuilder/MfRunner/`

### Contributors

**Development**:
1. [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
2. [INSTALLATION.md](MfBuilder/INSTALLATION.md) - Development setup
3. [CHANGELOG.md](CHANGELOG.md) - Recent changes
4. Source repositories in `MfBuilder/`

---

## Documentation by Topic

### Installation & Setup
- [INSTALLATION.md](MfBuilder/INSTALLATION.md) - Detailed installation
- [DEPENDENCIES.md](MfBuilder/DEPENDENCIES.md) - System requirements
- [QUICKSTART.md](MfBuilder/QUICKSTART.md) - Quick setup

### Features & Capabilities
- [FEATURES.md](FEATURES.md) - Complete feature reference
- [README.md](README.md) - Feature overview

### Configuration
- [BOT_SETUP.md](MfBuilder/BOT_SETUP.md) - Bot configuration
- `build.json` - CLI configuration
- [QUICKSTART.md](MfBuilder/QUICKSTART.md) - Configuration examples

### Technical Details
- [UAC_BYPASS_INFO.md](MfBuilder/UAC_BYPASS_INFO.md) - UAC technique
- [DEFENDER_EXCLUSION_INFO.md](MfBuilder/DEFENDER_EXCLUSION_INFO.md) - Defender evasion
- [FEATURES.md](FEATURES.md) - Implementation details

### Troubleshooting
- [INSTALLATION.md](MfBuilder/INSTALLATION.md#troubleshooting) - Installation issues
- [QUICKSTART.md](MfBuilder/QUICKSTART.md#troubleshooting) - Common problems
- [BOT_SETUP.md](MfBuilder/BOT_SETUP.md#troubleshooting) - Bot-specific issues

### Project Information
- [README.md](README.md) - Overview and credits
- [LICENSE](LICENSE) - Legal terms
- [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute
- [CHANGELOG.md](CHANGELOG.md) - Version history

---

## File Organization

```
Motherfudder/
??? README.md                          # Main project documentation
??? LICENSE                            # MIT License
??? CHANGELOG.md                       # Version history
??? CONTRIBUTING.md                    # Contribution guidelines
??? FEATURES.md                        # Feature documentation
??? DOCUMENTATION_INDEX.md            # This file
?
??? MfBuilder/                         # Main application
?   ??? .env.example                  # Bot token template
?   ??? build.json                    # CLI configuration
?   ?
?   ??? INSTALLATION.md               # Installation guide
?   ??? QUICKSTART.md                 # Quick start guide
?   ??? BOT_SETUP.md                  # Bot setup guide
?   ??? DEPENDENCIES.md               # System dependencies
?   ??? UAC_BYPASS_INFO.md           # UAC bypass details
?   ??? DEFENDER_EXCLUSION_INFO.md   # Defender exclusion guide
?   ?
?   ??? src/                          # Rust source code
?   ?   ??? main.rs                  # Entry point
?   ?   ??? telegram_bot.rs          # Bot implementation
?   ?   ??? binary_arch.rs           # Binary detection
?   ?   ??? build_native.rs          # Native builder
?   ?   ??? build_dotnet.rs          # .NET builder
?   ?   ??? ...
?   ?
?   ??? MfRunner/                     # C# stub loader
?       ??? Program.cs               # Main entry point
?       ??? Patches/                 # AV evasion
?       ?   ??? PatchAMSI.cs
?       ?   ??? PatchETW.cs
?       ?   ??? ...
?       ??? Utilities/               # Features
?       ?   ??? DefenderExclusion.cs
?       ?   ??? UacBypass.cs
?       ?   ??? ...
?       ??? ...
?
??? MfObfDotNet/                      # .NET obfuscator
    ??? ...
```

---

## Documentation Standards

### Format

All documentation uses Markdown format with:
- Clear headings and structure
- Code blocks with syntax highlighting
- Tables for structured information
- Links for cross-references

### Maintenance

Documentation is maintained alongside code:
- Updated with feature changes
- Reviewed for accuracy
- Kept consistent in style
- Version controlled with code

### Contributing

To improve documentation:
1. Read [CONTRIBUTING.md](CONTRIBUTING.md)
2. Make changes in Markdown
3. Submit pull request
4. Include rationale for changes

---

## Version Information

This documentation set corresponds to:
- **Version**: 1.0.0
- **Last Updated**: 2025-11-03
- **Maintainer**: Motherfudder Project

For latest version, always refer to repository main branch.

---

## External Resources

### Technologies Used
- [Rust](https://www.rust-lang.org/) - Primary language
- [Teloxide](https://github.com/teloxide/teloxide) - Telegram bot framework
- [.NET Framework](https://dotnet.microsoft.com/) - Stub runtime
- [Goblin](https://github.com/m4b/goblin) - Binary parsing

### Related Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Telegram Bot API](https://core.telegram.org/bots/api)
- [Windows API](https://docs.microsoft.com/en-us/windows/win32/api/)

---

## Support

For questions or issues:

1. **Check Documentation**: Search this index for relevant topics
2. **Review Examples**: Check QUICKSTART.md for common scenarios
3. **Read Troubleshooting**: Consult troubleshooting sections
4. **Open Issue**: If problem persists, open issue in repository

---

## Feedback

Documentation feedback is welcome:
- Unclear instructions
- Missing information
- Outdated content
- Suggested improvements

Submit via pull request or issue in repository.

---

**This documentation index is maintained to provide easy navigation and reference for all project documentation.**
