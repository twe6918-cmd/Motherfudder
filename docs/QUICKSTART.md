# Quick Start Guide

Get operational with Motherfudder Crypter in under 5 minutes.

---

## Prerequisites

Before starting, ensure you have:
- Rust toolchain installed
- OpenSSL development libraries
- .NET SDK (for building)

See [INSTALLATION.md•(INSTALLATION.md) for detailed setup instructions.

---

## Option 1: Telegram Bot Mode

### Step 1: Install Dependencies

```bash
# Ubuntu/Debian
sudo apt-get install -y pkg-config libssl-dev

# For other platforms, see INSTALLATION.md
```

### Step 2: Configure Bot

1. Create Telegram bot via [@BotFather•(https://t.me/BotFather):
   - Send `/newbot`
   - Follow prompts to set name and username
   - Copy the bot token

2. Set up environment:

```bash
cp .env.example .env
nano .env  # Add your TELOXIDE_TOKEN
```

3. Configure authentication key in `src/telegram_bot.rs` (line 70):

```rust
const VALID_KEY: &str = "YourSecureKeyHere"; // Change this!
```

### Step 3: Start Bot

```bash
cargo run -- --bot
```

### Step 4: Use Bot

1. Find your bot on Telegram
2. Send `/start` command
3. Enter your authentication key
4. Confirm crypting operation
5. Upload `.exe` file
6. Configure options (1-7) or change format
7. Send `build` to initiate build process
8. Receive crypted binary

---

## Option 2: CLI Mode

### Step 1: Install Dependencies

```bash
# Ubuntu/Debian
sudo apt-get install -y pkg-config libssl-dev
```

### Step 2: Configure Build

Edit `build.json`:

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
    "binder": false
}
```

### Step 3: Prepare Payload

```bash
cp /path/to/target.exe payload.exe
```

### Step 4: Build

```bash
cargo run
```

### Step 5: Retrieve Output

Output file: `out.bat` or `out.exe` (based on configuration)

---

## Common Use Cases

### Remote Operation

**Scenario**: Provide crypting service to authorized users

**Setup**: Deploy bot on server with secure authentication

**Workflow**:
1. Configure bot with secure key
2. Share bot username with authorized users
3. Users crypt binaries remotely
4. Receive crypted files via Telegram

### Batch Processing

**Scenario**: Process multiple binaries with same configuration

**Implementation**:
```bash
for binary in *.exe; do
    cp "$binary" payload.exe
    cargo run
    mv out.bat "${binary%.exe}_crypted.bat"
done
```

### Quick Testing

**Scenario**: Test different configurations rapidly

**Approach**: Use Telegram bot for interactive configuration testing

---

## Configuration Quick Reference

| Feature | Description | Recommended For |
|---------|-------------|-----------------|
| **Anti Debug** | Detects debuggers | Production |
| **Anti VM** | Detects virtual machines | Avoiding sandboxes |
| **Blacklist CIS** | Blocks CIS countries | Geographic restrictions |
| **UAC Bypass** | Silent privilege escalation | Admin operations |
| **Single Instance** | Prevents multiple instances | Resource management |
| **Persistence** | Auto-start on boot | Long-term deployment |
| **Defender Exclusion** | Windows Defender bypass | Maximum stealth |
| **Output: BAT** | Batch file wrapper | Higher evasion |
| **Output: EXE** | Direct executable | Simpler deployment |

---

## Security Checklist

Before production deployment:

- [ • Change default authentication key
- [ • Secure bot token in `.env`
- [ • Never commit `.env` to version control
- [ • Test in isolated environment
- [ • Verify all features function correctly
- [ • Review applicable laws and regulations
- [ • Obtain proper authorization

---

## Troubleshooting

### "Could not find openssl"

**Solution**:
```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# See INSTALLATION.md for other platforms
```

### "Invalid key" in Bot

**Solution**:
- Verify authentication key matches configuration in `src/telegram_bot.rs`
- Ensure no extra whitespace in key

### "Build failed"

**Solution**:
- Verify MSBuild is installed
- Check that `payload.exe` is valid PE executable
- Review error logs for specific issues

### Bot Not Responding

**Solution**:
- Verify `TELOXIDE_TOKEN` in `.env` is correct
- Ensure bot is running with `--bot` flag
- Check network connectivity
- Verify bot username is correct

---

## Example Bot Session

```
User: /start
Bot: Welcome to MfBuilder Crypter Bot
     Please enter your authentication key to continue:

User: SecureKey123
Bot: Authentication successful!
     Do you want to crypt a binary•
     Reply: yes or no

User: yes
Bot: Please upload your binary (.exe file)
     Only .exe files are supported!

User: [uploads application.exe•
Bot: Binary uploaded successfully!
     Detected Type: .NET x64
     
     Now let's configure the crypter options...
     
     Configuration menu appears...

User: 7
Bot: [Updated configuration with Defender Exclusion enabled•

User: 4
Bot: [Updated configuration with UAC Bypass enabled•
     [Shows: Windows Defender Exclusion: ON (Silent with UAC!)•

User: build
Bot: Starting build process...
     Building crypted binary...
     Build completed successfully!
     [Sends crypted file•
     Done! Your crypted binary is ready!
```

---

## Next Steps

After initial setup:

1. **Read Full Documentation**:
   - [README.md•(../README.md) - Project overview
   - [FEATURES.md•(../FEATURES.md) - Detailed feature documentation
   - [BOT_SETUP.md•(BOT_SETUP.md) - Advanced bot configuration

2. **Review Technical Details**:
   - [UAC_BYPASS_INFO.md•(UAC_BYPASS_INFO.md) - UAC bypass technique
   - [DEFENDER_EXCLUSION_INFO.md•(DEFENDER_EXCLUSION_INFO.md) - Defender evasion

3. **Stay Updated**:
   - [CHANGELOG.md•(../CHANGELOG.md) - Version history

---

## Best Practices

### Configuration Management

- Start with default settings and customize incrementally
- Test configurations in safe environment before production
- Document custom configurations for reproducibility

### Deployment

- Use bot mode for remote/multi-user scenarios
- Use CLI mode for automation and batch processing
- Maintain separate configurations for testing and production

### Security

- Rotate authentication keys periodically
- Monitor bot access logs
- Keep `.env` file secure
- Review permissions on output files

---

## Support

For additional assistance:

- Review documentation in `docs/` directory
- Check [INSTALLATION.md•(INSTALLATION.md) for setup issues
- Consult [FEATURES.md•(../FEATURES.md) for feature details
- Verify system requirements and dependencies

---

**For detailed information on all features and advanced configuration, consult the complete documentation suite.**
