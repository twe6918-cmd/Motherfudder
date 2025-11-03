# Telegram Bot Setup Guide

Comprehensive guide for deploying and configuring the Motherfudder Crypter Telegram bot.

---

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Initial Setup](#initial-setup)
4. [Configuration](#configuration)
5. [Deployment](#deployment)
6. [Usage](#usage)
7. [Advanced Configuration](#advanced-configuration)
8. [Security Considerations](#security-considerations)
9. [Troubleshooting](#troubleshooting)

---

## Overview

The Telegram bot interface provides an interactive method for crypting binaries with the following capabilities:

- **Authentication**: Key-based access control
- **Binary Detection**: Automatic identification of payload architecture
- **Interactive Configuration**: Real-time feature toggling
- **File Management**: Upload/download support
- **Multi-User**: Independent session management
- **Remote Access**: Operation from anywhere with internet

---

## Prerequisites

### System Requirements

- Linux server or workstation (recommended) or Windows/macOS
- Rust toolchain (1.70.0+)
- OpenSSL development libraries
- Network connectivity
- Telegram account

### Software Dependencies

```bash
# Ubuntu/Debian
sudo apt-get install pkg-config libssl-dev build-essential

# Fedora/RHEL
sudo dnf install pkg-config openssl-devel

# See INSTALLATION.md for other platforms
```

---

## Initial Setup

### Step 1: Create Telegram Bot

1. Open Telegram and search for [@BotFather](https://t.me/BotFather)

2. Send the `/newbot` command

3. Follow prompts:
   - **Bot Name**: Display name (e.g., "MfBuilder Crypter")
   - **Bot Username**: Unique identifier (e.g., "mfbuilder_bot")

4. Copy the bot token provided by BotFather

Example token format: `1234567890:ABCdefGHIjklMNOpqrsTUVwxyz`

### Step 2: Configure Environment

1. Navigate to project directory:

```bash
cd MfBuilder
```

2. Create environment file:

```bash
cp .env.example .env
```

3. Edit `.env` and add your bot token:

```env
TELOXIDE_TOKEN=1234567890:ABCdefGHIjklMNOpqrsTUVwxyz
```

**Important**: Never commit `.env` file to version control. It is already included in `.gitignore`.

### Step 3: Configure Authentication

Edit `src/telegram_bot.rs` (line 70):

```rust
const VALID_KEY: &str = "YourSecureKeyHere"; // Change this!
```

**Security Recommendations**:
- Use a strong, random key (minimum 16 characters)
- Include mixed case, numbers, and symbols
- Never share the key publicly
- Rotate keys periodically

Example strong key: `Xk9#mP2$vL8@nQ5!rT3`

---

## Configuration

### Bot Commands

The bot supports the following commands:

| Command | Description |
|---------|-------------|
| `/start` | Initialize bot and begin new session |
| `/help` | Display help information and usage instructions |
| `/reset` | Reset current session and clear state |

### Authentication Flow

```
User sends /start
    ?
Bot requests authentication key
    ?
User provides key
    ?
Bot validates against configured key
    ?
If valid: Grant access
If invalid: Deny access with error message
```

### Session Management

- Each user maintains independent session
- Sessions persist until explicitly reset
- State includes: authentication status, uploaded binary, configuration

---

## Deployment

### Local Deployment

For testing or single-user scenarios:

```bash
cd MfBuilder
cargo run -- --bot
```

Expected output:
```
[INFO] Starting MfBuilder Telegram Bot...
```

Keep terminal open while bot is running.

### Background Deployment

For production environments:

#### Using screen

```bash
screen -S mfbuilder-bot
cd MfBuilder
cargo run --release -- --bot
# Press Ctrl+A, then D to detach
```

Reattach: `screen -r mfbuilder-bot`

#### Using tmux

```bash
tmux new -s mfbuilder-bot
cd MfBuilder
cargo run --release -- --bot
# Press Ctrl+B, then D to detach
```

Reattach: `tmux attach -t mfbuilder-bot`

#### Using systemd (Linux)

Create service file: `/etc/systemd/system/mfbuilder-bot.service`

```ini
[Unit]
Description=MfBuilder Telegram Bot
After=network.target

[Service]
Type=simple
User=youruser
WorkingDirectory=/path/to/MfBuilder
Environment=TELOXIDE_TOKEN=your_token_here
ExecStart=/path/to/MfBuilder/target/release/MfBuilder --bot
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable mfbuilder-bot
sudo systemctl start mfbuilder-bot
```

Check status:

```bash
sudo systemctl status mfbuilder-bot
```

### Docker Deployment

Create `Dockerfile`:

```dockerfile
FROM rust:latest

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo build --release

ENV TELOXIDE_TOKEN=""

CMD ["./target/release/MfBuilder", "--bot"]
```

Build and run:

```bash
docker build -t mfbuilder-bot .
docker run -d \
    --name mfbuilder-bot \
    -e TELOXIDE_TOKEN=your_token_here \
    --restart unless-stopped \
    mfbuilder-bot
```

---

## Usage

### Complete Workflow

#### 1. Initiate Session

User sends `/start` command to bot.

#### 2. Authentication

Bot responds:
```
Welcome to MfBuilder Crypter Bot

Please enter your authentication key to continue:
```

User sends configured authentication key.

#### 3. Confirm Operation

Bot responds:
```
Authentication successful!

Do you want to crypt a binary?
Reply: yes or no
```

User responds: `yes`

#### 4. Upload Binary

Bot responds:
```
Please upload your binary (.exe file)

Only .exe files are supported!
```

User uploads `.exe` file via Telegram.

#### 5. Binary Detection

Bot analyzes uploaded file and responds:
```
Binary uploaded successfully!
Detected Type: .NET x64

Now let's configure the crypter options...
```

#### 6. Configure Options

Bot displays configuration menu:

```
Crypter Configuration

1. Anti Debug: OFF
2. Anti VM: OFF
3. Blacklist CIS Countries: OFF
4. UAC Bypass: OFF
5. Single Instance: OFF
6. Persistence: OFF
7. Windows Defender Exclusion: OFF
8. Output Format: BAT

To toggle an option, send its number (e.g., '1')
To change output format, send 'format' (BAT/EXE)
When done, send 'build' to create your crypted binary!
```

User toggles options by sending numbers:
- Send `1` to toggle Anti Debug
- Send `4` to toggle UAC Bypass
- Send `7` to toggle Defender Exclusion
- Send `format` to switch output format

Menu updates in real-time after each toggle.

#### 7. Build

When configuration is complete, user sends: `build`

Bot responds:
```
Starting build process...

This may take a few moments. Please wait...
```

#### 8. Receive Output

Upon completion:
```
Build completed successfully!

Sending your crypted binary...

[File sent]

Done! Your crypted binary is ready!

Send /start to crypt another binary.
```

---

## Advanced Configuration

### Multi-User Authentication

To implement per-user authentication, modify `src/telegram_bot.rs`:

```rust
use std::collections::HashMap;

fn is_valid_key(key: &str, user_id: UserId) -> bool {
    let valid_keys: HashMap<i64, &str> = HashMap::from([
        (123456789, "UserAKey"),
        (987654321, "UserBKey"),
    ]);
    
    valid_keys.get(&user_id.0).map_or(false, |&k| k == key)
}
```

### User ID Restrictions

Limit bot access to specific Telegram user IDs:

```rust
const ALLOWED_USERS: &[i64] = &[123456789, 987654321];

fn is_authorized(user_id: UserId) -> bool {
    ALLOWED_USERS.contains(&user_id.0)
}
```

### Rate Limiting

Implement basic rate limiting:

```rust
use std::time::{Duration, Instant};

struct RateLimit {
    last_request: Instant,
    min_interval: Duration,
}

impl RateLimit {
    fn check(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_request) >= self.min_interval {
            self.last_request = now;
            true
        } else {
            false
        }
    }
}
```

### Logging

Enable detailed logging:

```bash
RUST_LOG=debug cargo run -- --bot
```

Log levels: `error`, `warn`, `info`, `debug`, `trace`

---

## Security Considerations

### Best Practices

1. **Strong Authentication**:
   - Use complex authentication keys
   - Minimum 16 characters
   - Mix of uppercase, lowercase, numbers, symbols

2. **Token Protection**:
   - Never commit `.env` to repository
   - Use environment variables in production
   - Rotate tokens if compromised

3. **Access Control**:
   - Implement user ID restrictions for production
   - Monitor access logs
   - Revoke access immediately upon suspicion

4. **Network Security**:
   - Deploy behind firewall when possible
   - Use HTTPS for all communications (handled by Telegram)
   - Consider VPN for sensitive deployments

5. **File Management**:
   - Temporary files are auto-cleaned
   - Monitor disk space regularly
   - Implement file size limits if needed

### Monitoring

Monitor bot activity:

```bash
# View logs
journalctl -u mfbuilder-bot -f

# Check process
ps aux | grep MfBuilder

# Monitor resource usage
top -p $(pgrep -f MfBuilder)
```

---

## Troubleshooting

### Bot Not Responding

**Symptoms**: No response to `/start` or other commands

**Solutions**:
1. Verify bot token in `.env` is correct
2. Ensure bot process is running
3. Check network connectivity
4. Review logs for errors
5. Restart bot service

### Authentication Failures

**Symptoms**: "Invalid key" message despite correct key

**Solutions**:
1. Verify key in `src/telegram_bot.rs` matches exactly
2. Check for whitespace or special characters
3. Rebuild after changing authentication key
4. Clear bot session with `/reset`

### Build Failures

**Symptoms**: "Build failed" message

**Solutions**:
1. Check MSBuild is installed
2. Verify .NET SDK is available
3. Ensure uploaded file is valid PE executable
4. Review bot logs for specific errors
5. Check disk space availability

### Upload Issues

**Symptoms**: File upload rejected or fails

**Solutions**:
1. Ensure file is `.exe` format
2. Check file size (Telegram limit: 50MB for files, 2GB for documents)
3. Verify file is not corrupted
4. Try re-uploading

### Performance Issues

**Symptoms**: Slow responses or timeouts

**Solutions**:
1. Check server resources (CPU, RAM, disk)
2. Monitor concurrent users
3. Implement rate limiting
4. Consider dedicated server for production
5. Optimize build process

---

## Maintenance

### Updates

Update dependencies:

```bash
cd MfBuilder
cargo update
cargo build --release
```

Restart bot service after updates.

### Backup

Important files to backup:
- `src/telegram_bot.rs` (authentication configuration)
- `.env` (bot token)
- Service files (if using systemd)

### Monitoring

Regular checks:
- Bot responsiveness
- Disk space usage
- Error logs
- Authentication attempts
- Build success rate

---

## Support

For additional help:

- [QUICKSTART.md](QUICKSTART.md) - Quick start guide
- [INSTALLATION.md](INSTALLATION.md) - Installation instructions
- [README.md](../README.md) - Project overview
- [FEATURES.md](../FEATURES.md) - Feature documentation

---

## Notes

- Bot token grants full access to bot - keep secure
- Authentication key is stored in source code - change before deployment
- Sessions are memory-only - lost on restart
- File uploads are temporary - cleaned automatically
- Build artifacts are cleaned after download

---

**For production deployments, ensure proper security measures are implemented and authentication mechanisms are strengthened.**
