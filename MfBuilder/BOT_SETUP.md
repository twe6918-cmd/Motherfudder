# MfBuilder Telegram Bot Setup Guide

## Overview

The MfBuilder Telegram Bot provides an interactive interface for crypting binaries with customizable options. Users can authenticate, upload binaries, configure settings, and receive crypted executables directly through Telegram.

## Features

? **Key-based Authentication** - Secure access with authentication key  
? **Binary Type Detection** - Automatically detects Native x86/x64 and .NET x86/x64 binaries  
? **Interactive Configuration** - Toggle features on/off with simple commands  
? **Multiple Output Formats** - Generate BAT or EXE files  
? **Session Management** - Independent sessions for each user  

### Configurable Options

1. **Anti Debug** - Detect and prevent debugging
2. **Anti VM** - Detect and exit on virtual machines
3. **Blacklist CIS Countries** - Block execution in CIS countries
4. **UAC Bypass** - Attempt UAC elevation
5. **Single Instance** - Prevent multiple instances
6. **Persistence** - Add startup persistence
7. **Output Format** - Choose between BAT or EXE

## Setup Instructions

### 1. Create a Telegram Bot

1. Open Telegram and search for `@BotFather`
2. Send `/newbot` command
3. Follow the instructions to set a name and username
4. Copy the **bot token** provided by BotFather

### 2. Configure Environment

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` and add your bot token:
   ```env
   TELOXIDE_TOKEN=1234567890:ABCdefGHIjklMNOpqrsTUVwxyz
   ```

### 3. Set Authentication Key

Edit `src/telegram_bot.rs` and change the authentication key:

```rust
const VALID_KEY: &str = "YourSecureKeyHere"; // Change this!
```

?? **Important**: Use a strong, unique key for production!

### 4. Build and Run

Build the project:
```bash
cargo build --release
```

Run in bot mode:
```bash
./target/release/MfBuilder --bot
```

Or for development:
```bash
cargo run -- --bot
```

## Usage Flow

### 1. Start Conversation
User sends `/start` to the bot

### 2. Authentication
Bot asks for authentication key ? User sends the key

### 3. Confirm Crypting
Bot asks "Do you want to crypt a binary?" ? User replies `yes` or `no`

### 4. Upload Binary
User uploads `.exe` file (must be a valid PE executable)

### 5. Configure Options
Bot shows configuration menu with current settings:

```
?? Crypter Configuration

1. Anti Debug: ? OFF
2. Anti VM: ? OFF
3. Blacklist CIS Countries: ? OFF
4. UAC Bypass: ? OFF
5. Single Instance: ? OFF
6. Persistence: ? OFF
7. Output Format: BAT

?? To toggle an option, send its number (e.g., '1')
?? To change output format, send 'format'
? When done, send 'build' to create your crypted binary!
```

User can:
- Send `1` to `6` to toggle features
- Send `format` to switch between BAT/EXE
- Send `build` when ready

### 6. Build Process
Bot builds the crypted binary and sends it back to the user

### 7. Download Result
User receives the crypted binary as a file attachment

## Bot Commands

- `/start` - Start the bot and begin a new session
- `/help` - Show help information
- `/reset` - Reset current session

## Example Session

```
User: /start
Bot: ?? Welcome to MfBuilder Crypter Bot
     Please enter your authentication key to continue:

User: MfCrypter2024
Bot: ? Authentication successful!
     Do you want to crypt a binary?
     Reply: yes or no

User: yes
Bot: ?? Please upload your binary (.exe file)
     ?? Only .exe files are supported!

User: [uploads calc.exe]
Bot: ? Binary uploaded successfully!
     ?? Detected Type: .NET x64
     Now let's configure the crypter options...
     
     [Configuration menu appears]

User: 1
Bot: [Updated menu with Anti Debug: ? ON]

User: 2
Bot: [Updated menu with Anti VM: ? ON]

User: build
Bot: ?? Starting build process...
     This may take a few moments. Please wait...
     
     ?? Building crypted binary...
     
     ? Build completed successfully!
     Sending your crypted binary...
     
     [Sends crypted file]
     
     ?? Done! Your crypted binary is ready!
     Send /start to crypt another binary.
```

## CLI Mode (Original)

To use the traditional CLI mode with `build.json`:

```bash
./target/release/MfBuilder
```

This will use the configuration from `build.json` and `payload.exe` as before.

## Security Notes

- **Keep your bot token secret** - Never commit `.env` to version control
- **Use a strong authentication key** - Change the default key before deployment
- **Limit bot access** - Consider restricting the bot to specific Telegram user IDs
- **Monitor usage** - Keep logs of who uses the bot and when
- **Clean up files** - Temporary files are automatically cleaned, but monitor disk space

## Troubleshooting

### Bot doesn't respond
- Check that the bot token is correct in `.env`
- Ensure the bot is running with `--bot` flag
- Check firewall/network settings

### Build fails
- Ensure all dependencies are installed (MSBuild, .NET SDK, etc.)
- Check that temporary files have write permissions
- Review logs for specific error messages

### Authentication issues
- Verify the authentication key matches exactly
- Use `/reset` to clear session and try again

## Advanced Configuration

### Custom Authentication Keys per User

Modify `telegram_bot.rs` to support multiple keys:

```rust
fn is_valid_key(key: &str, user_id: UserId) -> bool {
    let valid_keys = HashMap::from([
        (123456789, "UserAKey"),
        (987654321, "UserBKey"),
    ]);
    
    valid_keys.get(&user_id.0).map_or(false, |&k| k == key)
}
```

### Rate Limiting

Consider adding rate limiting to prevent abuse:

```rust
use std::time::{Duration, Instant};

struct RateLimit {
    last_request: Instant,
    min_interval: Duration,
}
```

### Logging

Enable detailed logging:

```bash
RUST_LOG=debug cargo run -- --bot
```

## Support

For issues or questions:
- Check the main README.md
- Review the source code in `src/telegram_bot.rs`
- Ensure all prerequisites are installed
