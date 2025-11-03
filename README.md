# Motherfudder

**Motherfudder** is a crypter for Windows executables that supports both native and .NET binaries.<br>

## ? New Features

### ?? Telegram Bot Interface
The crypter now includes a fully-featured Telegram bot that allows you to:
- Authenticate with a secure key
- Upload binaries (.exe files)
- Automatically detect binary type (Native x86/x64 or .NET x86/x64)
- Interactively configure all crypter options
- Receive your crypted binary directly in Telegram

**Quick Start:**
```bash
# Setup bot (see BOT_SETUP.md for details)
cp MfBuilder/.env.example MfBuilder/.env
# Edit .env and add your Telegram bot token

# Run in bot mode
cd MfBuilder
cargo run -- --bot
```

See [BOT_SETUP.md](MfBuilder/BOT_SETUP.md) for complete setup instructions.

### ??? Enhanced AMSI Bypass
- Updated AMSI bypass using memory patching at offset +33
- More evasive technique using `xor rbx, rbx` instruction
- Proper memory protection restoration for stealth

## Features

- ? **Dual Binary Support**: Native (x86/x64) and .NET executables
- ? **Anti-Debug Protection**: Detect and prevent debugging attempts
- ? **Anti-VM Detection**: Detect virtual machine environments
- ? **CIS Country Blocking**: Prevent execution in CIS countries
- ? **UAC Bypass**: Attempt privilege escalation
- ? **Single Instance**: Prevent multiple simultaneous instances
- ? **Persistence**: Auto-start on system boot
- ? **AMSI Bypass**: Evade Windows Antimalware Scan Interface
- ? **ETW Patching**: Disable Event Tracing for Windows
- ? **String Obfuscation**: Encrypted strings with RC4
- ? **Multiple Output Formats**: Generate BAT or EXE files

## Usage Modes

### ?? Telegram Bot Mode (Recommended)

Interactive interface via Telegram:
```bash
cd MfBuilder
cargo run -- --bot
```

1. Start a chat with your bot
2. Send your authentication key
3. Upload your binary
4. Configure options interactively
5. Receive crypted binary

### ??? CLI Mode (Traditional)

Configure via `build.json`:
```bash
cd MfBuilder
# Edit build.json with your settings
# Place your binary as payload.exe
cargo run
```

## Configuration Options

All options available in both bot and CLI modes:

| Option | Description |
|--------|-------------|
| Anti Debug | Detect and prevent debugging |
| Anti VM | Exit if running in a virtual machine |
| Blacklist CIS | Block execution in CIS countries |
| UAC Bypass | Attempt to elevate privileges (fodhelper.exe technique) |
| Single Instance | Prevent multiple instances |
| Persistence | Add startup persistence |
| Defender Exclusion | Add Windows Defender exclusions (?? silent when combined with UAC bypass!) |
| Output Format | BAT or EXE file |

**Note**: AMSI/ETW patches are automatically applied for .NET payloads only. Native payloads use syscalls and don't need these patches.

## Building

### Prerequisites
- Rust toolchain (latest stable)
- MSBuild (for .NET compilation)
- .NET SDK (for .NET binary support)
- Windows OS (for building)

### Compile
```bash
cd MfBuilder
cargo build --release
```

## Project Structure

```
MfBuilder/
??? src/
?   ??? main.rs              # Entry point and CLI mode
?   ??? telegram_bot.rs      # Telegram bot implementation
?   ??? binary_arch.rs       # Binary type detection
?   ??? build_native.rs      # Native binary builder
?   ??? build_dotnet.rs      # .NET binary builder
?   ??? ...
??? MfRunner/                # C# stub for .NET execution
?   ??? Patches/
?   ?   ??? PatchAMSI.cs    # Enhanced AMSI bypass
?   ?   ??? PatchETW.cs     # ETW patching
?   ??? ...
??? build.json              # CLI mode configuration
??? .env.example           # Bot token template
??? BOT_SETUP.md          # Bot setup guide
```

## Security Notes

?? **This tool is for educational and authorized testing purposes only**

- Keep your Telegram bot token secure
- Use strong authentication keys
- Never commit `.env` to version control
- Monitor bot usage and logs
- Be aware of the legal implications in your jurisdiction

## Known Issues

This is an early release with some known limitations. Contributions welcome!

## Author

**Developed by Florin**

Special techniques and features:
- Enhanced AMSI bypass (offset +33 patching)
- Windows Defender silent exclusions
- UAC bypass integration
- Telegram bot interface

## Contributing

Feel free to submit issues and pull requests.

### Please leave a star if this project helps you ?
