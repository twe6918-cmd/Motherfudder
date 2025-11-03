# Motherfudder Crypter

**Motherfudder** is an advanced crypter for Windows executables supporting both native (x86/x64) and .NET binaries. It features multiple evasion techniques, interactive Telegram bot interface, and silent Windows Defender exclusions.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)]()
[![Language](https://img.shields.io/badge/language-Rust%20%7C%20C%23-orange.svg)]()

---

## ?? Disclaimer

This tool is designed for **authorized security testing, penetration testing, and educational purposes only**. Users are responsible for complying with applicable laws and regulations. Unauthorized use against systems you do not own or have explicit permission to test is illegal and unethical.

---

## Features

### Core Capabilities

- **Dual Binary Support**: Crypts both native (x86/x64) and .NET executables
- **Anti-Analysis**: Detects and prevents debugging and VM execution
- **Geographic Restrictions**: Optional blocking of execution in CIS countries
- **Privilege Escalation**: UAC bypass using fodhelper.exe technique
- **Instance Management**: Prevents multiple simultaneous executions
- **Persistence**: Auto-start on system boot
- **AV Evasion**: AMSI/ETW patching for .NET payloads
- **Silent Defender Exclusions**: Automatic Windows Defender exclusion with UAC bypass
- **Multiple Output Formats**: BAT or EXE file generation

### What This Crypter Does NOT Include

For transparency, the following advanced techniques are **not implemented**:

- ? **Process Hollowing** - Does not use process hollowing/RunPE techniques
- ? **Process Injection** - No remote process injection
- ? **Reflective DLL Injection** - Not implemented
- ? **Thread Hijacking** - Does not hijack existing threads
- ? **APC Injection** - No asynchronous procedure call injection

**What it uses instead**:
- ? Direct payload execution (for .NET assemblies)
- ? Indirect syscalls (for native payloads)
- ? Memory-based AMSI/ETW patching
- ? Windows Defender exclusions
- ? UAC bypass for privilege escalation

This approach provides effective evasion while maintaining simplicity and reliability.

### Advanced Features

#### AMSI Bypass (.NET Payloads)
- Memory patching at offset +33 within AmsiScanBuffer
- Minimal 3-byte patch using `xor rbx, rbx` instruction
- Proper memory protection restoration for stealth
- Encrypted patch bytes (RC4)

#### ETW Patching (.NET Payloads)
- Disables Event Tracing for Windows
- Prevents telemetry and behavioral analysis

#### Windows Defender Exclusion
- Adds path and process exclusions
- **Silent operation** when combined with UAC bypass
- PowerShell-based implementation
- Graceful failure handling

#### UAC Bypass
- fodhelper.exe registry hijacking technique
- Supports Windows 10/11
- Enables silent privilege escalation
- Automatic cleanup of registry artifacts

### Interface Modes

#### Telegram Bot Mode
- Key-based authentication
- Interactive configuration menu
- Automatic binary type detection
- Real-time feature toggling
- File upload/download support
- Multi-user session management

#### CLI Mode
- JSON-based configuration
- Batch processing support
- Scriptable builds
- Traditional workflow

---

## Quick Start

### Prerequisites

- Rust toolchain (latest stable)
- MSBuild and .NET SDK
- OpenSSL development libraries
- Windows operating system (for building)

### Installation

1. **Install dependencies**:

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y pkg-config libssl-dev build-essential

# Fedora/RHEL
sudo dnf install -y pkg-config openssl-devel

# See docs/DEPENDENCIES.md for other platforms
```

2. **Clone and build**:

```bash
git clone <repository-url>
cd MfBuilder
cargo build --release
```

### Usage

#### Telegram Bot Mode

1. Obtain bot token from [@BotFather](https://t.me/BotFather)
2. Configure environment:

```bash
cp .env.example .env
# Edit .env and add: TELOXIDE_TOKEN=your_token_here
```

3. Customize authentication key in `src/telegram_bot.rs` (line 70)

4. Start bot:

```bash
./target/release/MfBuilder --bot
```

5. Interact via Telegram:
   - Send `/start`
   - Authenticate with configured key
   - Upload `.exe` file
   - Configure options interactively
   - Receive crypted binary

#### CLI Mode

1. Configure build settings:

```json
// build.json
{
    "file_extension": "BAT",
    "anti_debug": true,
    "anti_virtual_machine": true,
    "blacklist_cis_countries": false,
    "uac_bypass": true,
    "single_instance": true,
    "run_on_startup": false,
    "defender_exclusion": true,
    "binder": false
}
```

2. Place target binary:

```bash
cp /path/to/target.exe payload.exe
```

3. Execute build:

```bash
./target/release/MfBuilder
```

4. Retrieve output: `out.bat` or `out.exe`

---

## Configuration Options

| Option | Description | Applies To |
|--------|-------------|------------|
| **Anti Debug** | Detects and terminates if debugger present | All payloads |
| **Anti VM** | Detects virtual machine environments and exits | All payloads |
| **Blacklist CIS** | Prevents execution in CIS countries (timezone-based) | All payloads |
| **UAC Bypass** | Elevates privileges using fodhelper.exe technique | All payloads |
| **Single Instance** | Prevents multiple simultaneous instances (mutex-based) | All payloads |
| **Persistence** | Adds scheduled task for auto-start on boot | All payloads |
| **Defender Exclusion** | Adds Windows Defender exclusions (silent with UAC bypass) | All payloads |
| **Output Format** | BAT (more evasive) or EXE (cleaner) | Build option |

**Note**: AMSI and ETW patches are automatically applied to .NET payloads only. Native payloads use indirect syscalls and do not require these patches.

---

## Architecture

### Project Structure

```
.
??? MfBuilder/                  # Main builder (Rust)
?   ??? src/
?   ?   ??? main.rs            # Entry point and CLI mode
?   ?   ??? telegram_bot.rs    # Telegram bot implementation
?   ?   ??? binary_arch.rs     # Binary type detection
?   ?   ??? build_native.rs    # Native payload builder
?   ?   ??? build_dotnet.rs    # .NET payload builder
?   ?   ??? ...
?   ??? MfRunner/              # C# stub loader
?   ?   ??? Patches/
?   ?   ?   ??? PatchAMSI.cs  # AMSI bypass
?   ?   ?   ??? PatchETW.cs   # ETW patching
?   ?   ?   ??? ...
?   ?   ??? Utilities/
?   ?   ?   ??? DefenderExclusion.cs
?   ?   ?   ??? UacBypass.cs
?   ?   ?   ??? ...
?   ?   ??? ...
?   ??? build.json             # CLI configuration
??? docs/                      # Documentation
```

### Build Process

1. **Binary Detection**: Determines if target is native or .NET, x86 or x64
2. **Key Generation**: Creates random encryption keys and seeds
3. **Stub Configuration**: Applies preprocessor symbols based on config
4. **Payload Encryption**: Encrypts payload with RC4
5. **Compilation**: Builds configured stub with MSBuild
6. **Obfuscation**: Applies .NET obfuscation (for .NET builds)
7. **Wrapper Generation**: Creates BAT or EXE wrapper

---

## Security Features

### Encryption

- **RC4 Cipher**: Payload and string encryption
- **Key Derivation**: SHA-256 based key derivation with random seeds
- **Unique Keys**: Each build generates unique encryption keys

### Evasion Techniques

#### .NET Payloads
- AMSI bypass (offset +33 patching)
- ETW patching
- CLR string obfuscation
- Method obfuscation
- Integer encoding

#### Native Payloads
- Indirect syscalls
- API hashing
- No Import Address Table entries
- Manual function loading

#### All Payloads
- String encryption
- API obfuscation
- Debugger detection
- VM detection
- Sandbox evasion

---

## Documentation

Comprehensive documentation is available in the `docs/` directory:

- **[Installation Guide](MfBuilder/DEPENDENCIES.md)** - System requirements and setup
- **[Quick Start Guide](MfBuilder/QUICKSTART.md)** - Get started in 5 minutes
- **[Bot Setup Guide](MfBuilder/BOT_SETUP.md)** - Telegram bot configuration
- **[UAC Bypass Documentation](MfBuilder/UAC_BYPASS_INFO.md)** - Technical details
- **[Defender Exclusion Guide](MfBuilder/DEFENDER_EXCLUSION_INFO.md)** - WD evasion details
- **[Changelog](CHANGELOG.md)** - Version history and updates

---

## Best Practices

### Operational Security

1. **Change default authentication key** before deploying bot
2. **Use strong bot tokens** and never commit `.env` file
3. **Monitor bot usage** and maintain access logs
4. **Test in isolated environments** before production use
5. **Understand legal implications** in your jurisdiction

### Configuration Recommendations

**For Maximum Stealth** (Recommended):
```json
{
    "file_extension": "BAT",
    "anti_debug": true,
    "anti_virtual_machine": true,
    "uac_bypass": true,
    "defender_exclusion": true,
    "defender_exclude_drive": false,
    "single_instance": true,
    "run_on_startup": true
}
```

**For Maximum Evasion** (?? Testing Only):
```json
{
    "file_extension": "BAT",
    "anti_debug": true,
    "anti_virtual_machine": true,
    "uac_bypass": true,
    "defender_exclusion": true,
    "defender_exclude_drive": true,  // Nuclear option!
    "single_instance": true,
    "run_on_startup": true
}
```

**For Testing**:
```json
{
    "file_extension": "EXE",
    "anti_debug": false,
    "anti_virtual_machine": false,
    "uac_bypass": false,
    "defender_exclusion": false
}
```

---

## Troubleshooting

### Build Errors

**OpenSSL not found**:
```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# See docs/DEPENDENCIES.md for other platforms
```

**MSBuild errors**:
- Ensure .NET SDK is installed
- Verify MSBuild is in PATH
- Check C# project configuration

### Runtime Issues

**Bot not responding**:
- Verify `TELOXIDE_TOKEN` in `.env`
- Ensure bot is started with `--bot` flag
- Check network connectivity

**UAC bypass failing**:
- Verify user is in Administrators group
- Ensure UAC is enabled on system
- Check Windows version compatibility

**Defender exclusion not applied**:
- Confirm UAC bypass succeeded
- Verify PowerShell execution policy
- Check Windows Defender status

---

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Submit a pull request

### Development Setup

```bash
git clone <repository-url>
cd MfBuilder
cargo build
cargo test
```

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Future Enhancements

Potential improvements being researched for future versions:

- **Enhanced Syscalls**: Hell's Gate / Halo's Gate techniques ([Ebyte-Syscalls](https://github.com/EvilBytecode/Ebyte-Syscalls))
- **Lifetime Patching**: Persistent AMSI/ETW monitoring ([Lifetime-Amsi-EtwPatch](https://github.com/EvilBytecode/Lifetime-Amsi-EtwPatch))
- **Interactive Buttons**: Inline keyboard interface for bot
- **Build Queue**: Parallel build processing
- **Performance**: Stub caching and optimizations

See [FUTURE_ENHANCEMENTS.md](FUTURE_ENHANCEMENTS.md) for detailed analysis and roadmap.

**Current Recommendation**: Existing techniques (Chainski AMSI bypass, indirect syscalls) are proven effective. Focus remains on stability and user experience.

---

## Credits

### Techniques and Frameworks

- **AMSI Bypass**: Based on research from [Chainski/GlobalAMSIBypass](https://github.com/Chainski/GlobalAMSIBypass)
- **Telegram Bot**: Powered by [teloxide](https://github.com/teloxide/teloxide)
- **UAC Bypass**: fodhelper.exe technique
- **Binary Parsing**: [goblin](https://github.com/m4b/goblin)

### Research & Inspiration

- **Syscall Techniques**: [EvilBytecode/Ebyte-Syscalls](https://github.com/EvilBytecode/Ebyte-Syscalls)
- **Persistent Patching**: [EvilBytecode/Lifetime-Amsi-EtwPatch](https://github.com/EvilBytecode/Lifetime-Amsi-EtwPatch)

### Author

Developed and maintained by **Florin**

---

## Acknowledgments

Special thanks to the security research community for their continuous work in understanding and documenting Windows internals and evasion techniques.

---

## Disclaimer (Reiterated)

**This tool is provided for educational and authorized testing purposes only.**

The author and contributors are not responsible for any misuse or damage caused by this software. Users must ensure they have explicit authorization before testing any systems they do not own. Unauthorized access to computer systems is illegal under laws including but not limited to the Computer Fraud and Abuse Act (CFAA) and similar legislation worldwide.

By using this software, you acknowledge that you understand these risks and agree to use it responsibly and legally.

---

### ? If this project helps you, please consider leaving a star!
