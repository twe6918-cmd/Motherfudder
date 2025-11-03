# Telegram Bot Technical Documentation

Comprehensive technical guide to the Telegram bot implementation, deployment, and operation.

---

## Table of Contents

1. [Architecture Overview•(#architecture-overview)
2. [How the Bot Works•(#how-the-bot-works)
3. [VPS Deployment•(#vps-deployment)
4. [Update Process•(#update-process)
5. [File Access & Permissions•(#file-access--permissions)
6. [Session Management•(#session-management)
7. [Security Model•(#security-model)
8. [Performance Considerations•(#performance-considerations)

---

## Architecture Overview

### System Components

```
Telegram Servers (Cloud)
        •
    Bot API
        •
VPS/Server (Your Infrastructure)
        •
MfBuilder Bot Process
        •
•
•  Bot Runtime Components           •
•
•  • teloxide Framework             •
•  • Session Manager (HashMap)      •
•  • File Handler (Upload/Download) •
•  • Build Orchestrator             •
•  • MfBuilder Core                 •
•  • MfRunner Compiler              •
•  • MfObfDotNet                    •
•
        •
Temporary File System
```

### Data Flow

```
User Message
    •
Telegram API • Bot Process
    •
teloxide Framework
    •
Message Handler
    •
Session Lookup (HashMap)
    •
State Machine Processing
    •
Action Execution
    •
Response Generation
    •
Telegram API • User
```

---

## How the Bot Works

### 1. Bot Initialization

When you run `./MfBuilder --bot`:

```rust
// In main.rs
#[tokio::main•
async fn main() {
    if args[1• == "--bot" {
        telegram_bot::run_telegram_bot().await;
    }
}
```

**Process**:
1. Loads environment variables from `.env`
2. Reads `TELOXIDE_TOKEN`
3. Initializes teloxide framework
4. Creates session storage (in-memory HashMap)
5. Registers message handlers
6. Starts event loop
7. Connects to Telegram API servers

### 2. File Access Model

**How Bot Accesses Files**:

The bot runs as a standard process and accesses files from:

```
Bot Working Directory:
/path/to/MfBuilder/
    • MfBuilder (executable)
    • MfRunner.exe
    • MfObfDotNet.exe
    • build.json
    • .env (bot token)
    • [temporary files•
```

**File Access Pattern**:
```rust
// Bot reads these files directly from filesystem
fs::read("MfRunner.exe")           // C# stub template
fs::read("MfObfDotNet.exe")        // .NET obfuscator
fs::read("build.json")             // Default config (optional)

// Bot writes temporary files
fs::write(&temp_path, binary_data)  // Uploaded binary
fs::write("payload.exe", data)      // Build input
fs::read("out.bat" or "out.exe")    // Build output
```

**File Permissions Required**:
- Read: `MfRunner.exe`, `MfObfDotNet.exe`
- Read/Write: Working directory (for temp files)
- Execute: `MfBuilder` binary itself

### 3. Connection to Telegram

**Network Communication**:

```
Bot Process
    •
HTTPS (443)
    •
api.telegram.org
    •
Telegram Bot API
    •
Telegram Servers
    •
User's Telegram Client
```

**What Gets Transmitted**:
- Text messages (configuration, status)
- File uploads (user's binary • bot)
- File downloads (crypted binary • user)
- Command updates (long polling or webhooks)

**Bot Token Usage**:
- Token authenticates bot to Telegram API
- Loaded from `.env` file
- Used in HTTPS headers
- Never transmitted to end users

### 4. Session Management

**In-Memory Session Storage**:

```rust
// Type definition
type Sessions = Arc<Mutex<HashMap<ChatId, UserSession>>>;

// Structure
Sessions {
    123456789: UserSession {
        authenticated: true,
        awaiting_binary: false,
        config: BuildConfig { ... },
        binary_path: Some("temp_123456789_app.exe")
    },
    987654321: UserSession {
        authenticated: false,
        ...
    }
}
```

**Session Lifecycle**:
1. User sends `/start` • Session created in HashMap
2. Authentication • `authenticated = true`
3. File upload • `binary_path = Some(path)`
4. Configuration • `config` updated
5. Build • Process executes, file sent
6. `/reset` or bot restart • Session destroyed

**Important**: Sessions are **memory-only** and lost on bot restart.

### 5. Build Process Flow

When user sends `build`:

```
1. Lock session HashMap
2. Retrieve user's configuration
3. Read uploaded binary from temp file
4. Create MfBuilder config struct
5. Write binary to payload.exe
6. Spawn blocking task:
   • build_with_config()
       • Detect binary architecture
       • Configure stub
       • Encrypt payload
       • Compile stub (MSBuild)
       • Generate output (out.bat/out.exe)
7. Read output file
8. Send as Telegram document
9. Clean up temporary files
10. Unlock session
```

**Execution Context**:
```rust
// Bot uses tokio for async operations
tokio::task::spawn_blocking(move || {
    // Build process runs in separate thread
    crate::build_with_config(build_config);
})
```

### 6. File Upload/Download

**User Uploads Binary**:
```
1. User sends .exe file in Telegram
2. Bot receives file metadata
3. Bot calls Telegram API to get file
4. Downloads file bytes via HTTPS
5. Validates file (check .exe extension)
6. Writes to temporary file: temp_{chat_id}_{filename}
7. Detects binary type (Native/NET, x86/x64)
8. Stores file path in session
```

**Bot Sends Crypted Binary**:
```
1. Build completes successfully
2. Bot reads output file (out.bat or out.exe)
3. Creates InputFile from path
4. Calls bot.send_document()
5. Telegram API uploads file
6. User receives file in chat
7. Bot deletes temporary files
```

---

## VPS Deployment

### Why Use a VPS•

**Benefits**:
- • **24/7 Uptime**: Bot runs continuously
- • **Remote Access**: Crypt binaries from anywhere
- • **No Local Resources**: Your PC doesn't need to run
- • **Reliability**: Professional infrastructure
- • **Multiple Users**: Serve many users simultaneously

**Recommended Setup**:
- VPS with Windows Server 2019/2022 (for MSBuild)
- Or Linux VPS (requires Mono for MSBuild)
- Minimum 2GB RAM, 20GB disk
- Dedicated IP address
- Firewall configured

### VPS Providers

**Recommended Providers**:
1. **DigitalOcean** - $10-20/month, easy setup
2. **Vultr** - $10-15/month, global locations
3. **Linode** - $10/month, reliable
4. **Hetzner** - •5-10/month, Europe-based
5. **AWS EC2** - Pay-as-you-go, scalable

### VPS Setup Process

#### Option A: Windows VPS

```powershell
# 1. Connect via RDP
# 2. Install prerequisites
# Install Visual Studio Build Tools
# Install Rust: https://rustup.rs/
# Install .NET SDK

# 3. Clone/Upload project
git clone <repository-url>
cd MfBuilder

# 4. Build
cargo build --release

# 5. Configure bot
copy .env.example .env
notepad .env  # Add TELOXIDE_TOKEN

# 6. Run in background
# Create scheduled task or use nssm.exe
nssm install MfBuilderBot "C:\path\to\MfBuilder.exe" "--bot"
nssm start MfBuilderBot
```

#### Option B: Linux VPS (Ubuntu)

```bash
# 1. Connect via SSH
ssh user@your-vps-ip

# 2. Install prerequisites
sudo apt-get update
sudo apt-get install -y pkg-config libssl-dev build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Mono (for MSBuild)
sudo apt install -y mono-complete

# 3. Upload/clone project
git clone <repository-url>
cd MfBuilder

# 4. Build
cargo build --release

# 5. Configure
cp .env.example .env
nano .env  # Add TELOXIDE_TOKEN

# 6. Run with systemd (see below)
```

### Running 24/7 with systemd (Linux)

Create service file: `/etc/systemd/system/mfbuilder-bot.service`

```ini
[Unit•
Description=MfBuilder Telegram Bot
After=network.target

[Service•
Type=simple
User=mfbuilder
WorkingDirectory=/home/mfbuilder/MfBuilder
Environment="TELOXIDE_TOKEN=your_token_here"
ExecStart=/home/mfbuilder/MfBuilder/target/release/MfBuilder --bot
Restart=always
RestartSec=10

[Install•
WantedBy=multi-user.target
```

**Enable and start**:
```bash
sudo systemctl daemon-reload
sudo systemctl enable mfbuilder-bot
sudo systemctl start mfbuilder-bot

# Check status
sudo systemctl status mfbuilder-bot

# View logs
sudo journalctl -u mfbuilder-bot -f
```

### Running 24/7 with screen (Quick Method)

```bash
# Start screen session
screen -S mfbuilder

# Run bot
cd MfBuilder
./target/release/MfBuilder --bot

# Detach: Ctrl+A, then D
# Reattach: screen -r mfbuilder
# List sessions: screen -ls
```

### VPS Security

**Essential Security Measures**:

```bash
# 1. Firewall - Only allow SSH and HTTPS
sudo ufw allow 22/tcp
sudo ufw allow 443/tcp
sudo ufw enable

# 2. SSH Key Authentication (disable password)
sudo nano /etc/ssh/sshd_config
# Set: PasswordAuthentication no
sudo systemctl restart sshd

# 3. Create dedicated user
sudo useradd -m -s /bin/bash mfbuilder
sudo su - mfbuilder

# 4. Keep updated
sudo apt-get update && sudo apt-get upgrade -y
```

**Bot Token Security**:
- Store in `.env` file (never commit)
- Set file permissions: `chmod 600 .env`
- Consider using environment variables
- Rotate token if compromised

---

## Update Process

### How to Update Bot While Running 24/7

**Scenario**: You've updated code and want to deploy changes without disrupting service.

#### Method 1: Quick Restart (Minimal Downtime)

```bash
# 1. SSH into VPS
ssh user@your-vps-ip

# 2. Navigate to project
cd MfBuilder

# 3. Pull latest changes
git pull origin main

# 4. Rebuild
cargo build --release

# 5. Restart bot service
sudo systemctl restart mfbuilder-bot

# Downtime: ~5-30 seconds (build time)
```

#### Method 2: Blue-Green Deployment (Zero Downtime)

```bash
# 1. Build in separate directory
cd /tmp
git clone <repository-url> MfBuilder-new
cd MfBuilder-new
cargo build --release

# 2. Stop old service
sudo systemctl stop mfbuilder-bot

# 3. Swap binaries
sudo mv /home/mfbuilder/MfBuilder/target/release/MfBuilder \
        /home/mfbuilder/MfBuilder/target/release/MfBuilder.old
sudo cp /tmp/MfBuilder-new/target/release/MfBuilder \
        /home/mfbuilder/MfBuilder/target/release/MfBuilder

# 4. Start new service
sudo systemctl start mfbuilder-bot

# 5. Verify
sudo systemctl status mfbuilder-bot

# 6. Cleanup
rm -rf /tmp/MfBuilder-new
```

#### Method 3: Git Workflow

```bash
# Update workflow
cd MfBuilder

# Stash any local changes
git stash

# Pull updates
git pull origin main

# Apply local config
git stash pop  # If you had changes

# Rebuild
cargo build --release

# Restart
sudo systemctl restart mfbuilder-bot
```

### File Updates Only (No Code Changes)

If you only updated `MfRunner.exe` or `MfObfDotNet.exe`:

```bash
# 1. Copy new files
scp MfRunner.exe user@vps:/home/mfbuilder/MfBuilder/
scp MfObfDotNet.exe user@vps:/home/mfbuilder/MfBuilder/

# 2. Restart bot
ssh user@vps "sudo systemctl restart mfbuilder-bot"

# No recompilation needed!
```

### Configuration Updates

If you changed authentication key or added features:

```bash
# 1. Edit source
nano src/telegram_bot.rs  # Change VALID_KEY

# 2. Rebuild
cargo build --release

# 3. Restart
sudo systemctl restart mfbuilder-bot

# Sessions are lost (users need to re-authenticate)
```

### Update Checklist

Before updating:
- [ • Backup current binary
- [ • Test changes locally
- [ • Review dependencies
- [ • Plan for session loss (if applicable)
- [ • Notify users of downtime (if any)

After updating:
- [ • Verify bot responds to `/start`
- [ • Test full workflow (upload • configure • build)
- [ • Check logs for errors
- [ • Monitor for issues

---

## File Access & Permissions

### Required Files in Working Directory

```
MfBuilder/
• MfBuilder               # Bot executable (must be executable)
• MfRunner.exe           # C# stub template (read access)
• MfObfDotNet.exe        # .NET obfuscator (read access)
• .env                   # Bot token (read access, 600 permissions)
• build.json             # Optional default config
• [temp files•           # Created/deleted during operation
```

### File Permission Requirements

```bash
# Set ownership
sudo chown -R mfbuilder:mfbuilder /home/mfbuilder/MfBuilder

# Set permissions
chmod 755 /home/mfbuilder/MfBuilder/target/release/MfBuilder  # Executable
chmod 644 /home/mfbuilder/MfBuilder/MfRunner.exe              # Read
chmod 644 /home/mfbuilder/MfBuilder/MfObfDotNet.exe          # Read
chmod 600 /home/mfbuilder/MfBuilder/.env                      # Secure token
chmod 755 /home/mfbuilder/MfBuilder                           # Directory
```

### Temporary File Handling

**File Lifecycle**:
```
User uploads • temp_{chatid}_{filename}
    •
Copied to • payload.exe
    •
Build process creates • out.bat or out.exe
    •
Sent to user • [file downloaded•
    •
Cleanup • All temp files deleted
```

**Automatic Cleanup**:
```rust
// In telegram_bot.rs
fs::remove_file(&binary_path).ok();  // Uploaded file
fs::remove_file("payload.exe").ok();  // Build input
fs::remove_file(output_file).ok();    // Build output
```

**Manual Cleanup** (if bot crashes):
```bash
# Remove orphaned temp files
cd MfBuilder
rm -f temp_*
rm -f payload.exe
rm -f out.bat out.exe
```

---

## Session Management

### Session Storage Implementation

```rust
// In-memory HashMap
pub type Sessions = Arc<Mutex<HashMap<ChatId, UserSession>>>;

// Thread-safe access
let mut sessions_lock = sessions.lock().unwrap();
let session = sessions_lock.entry(chat_id).or_insert_with(UserSession::default);
```

### Session State Machine

```
State: New User
    • /start
State: Awaiting Auth
    • send key
State: Authenticated
    • "yes"
State: Awaiting Binary
    • upload file
State: Configuring
    • toggle options
State: Ready to Build
    • "build"
State: Building
    • complete
State: Done
```

### Session Persistence Limitations

**Current Implementation**:
- • Multiple users simultaneously
- • Independent configurations
- • Fast in-memory access
- • Lost on bot restart
- • Not saved to disk
- • No session recovery

**Workaround for Restarts**:
Users simply need to:
1. Send `/start` again
2. Re-authenticate
3. Re-upload binary
4. Re-configure (starts from default)

### Concurrent User Handling

**Thread Safety**:
```rust
// Mutex ensures only one thread modifies sessions at a time
Arc<Mutex<HashMap>>
    •      •
    |       Lock for exclusive access
     Shared across threads
```

**Capacity**: Limited only by:
- Available memory
- VPS resources
- Telegram API rate limits

---

## Security Model

### Authentication Flow

```
User sends message
    •
Check if authenticated in session
    •
If NO • Request key
    •
User sends key
    •
Compare with VALID_KEY constant
    •
If match • Grant access
If no match • Deny + error message
```

**Security Considerations**:
-  Key stored in source code (plaintext)
-  Single key for all users
-  No rate limiting on auth attempts
- • Per-session authentication
- • No key in network traffic (only once)

### Recommended Security Enhancements

**1. Environment Variable Keys**:
```rust
// Instead of hardcoded key
const VALID_KEY: &str = env!("AUTH_KEY");

// Set in systemd service
Environment="AUTH_KEY=YourSecureKey"
```

**2. Per-User Authentication**:
```rust
// In telegram_bot.rs
fn is_valid_key(key: &str, user_id: UserId) -> bool {
    let users = HashMap::from([
        (123456789, "UserAKey"),
        (987654321, "UserBKey"),
    •);
    users.get(&user_id.0).map_or(false, |&k| k == key)
}
```

**3. User ID Whitelist**:
```rust
const ALLOWED_USERS: &[i64• = &[123456789, 987654321•;

fn check_access(user_id: UserId) -> bool {
    ALLOWED_USERS.contains(&user_id.0)
}
```

---

## Performance Considerations

### Resource Usage

**Typical Operation**:
- RAM: 50-100MB idle, 200-500MB during build
- CPU: <5% idle, 50-90% during build
- Disk: ~500MB for files, variable for temps
- Network: Minimal (only file uploads/downloads)

**Per-Build Resources**:
- Temporary disk: File size • 3 (input, encrypted, output)
- Build time: 10-60 seconds depending on payload size
- Network: Upload + download size

### Concurrent Builds

**Current Limitation**: Sequential builds

```rust
// Builds block the main thread
tokio::task::spawn_blocking(move || {
    build_with_config(config);  // Blocks until complete
})
```

**Impact**:
- User A starts build • 30 seconds
- User B starts build • Waits until User A completes
- User B's build starts • 30 seconds
- Total: 60 seconds for both

**Potential Enhancement**:
Build queue system for true parallelism.

### Rate Limiting

**Telegram API Limits**:
- 30 messages/second per bot
- 20 MB max file size (bots can use 50 MB)
- 2 GB for documents sent via URL

**Recommended Limits**:
- 1 build per user per minute
- 10 MB max upload file size
- 5 concurrent builds maximum

---

## Troubleshooting

### Bot Not Starting

```bash
# Check logs
sudo journalctl -u mfbuilder-bot -n 50

# Common issues:
# - Invalid TELOXIDE_TOKEN
# - Missing .env file
# - Port 443 blocked
# - Missing dependencies
```

### Build Failures on VPS

```bash
# Check MSBuild availability
which msbuild  # or mono /path/to/MSBuild.dll

# Check file permissions
ls -la MfRunner.exe MfObfDotNet.exe

# Check disk space
df -h

# Check memory
free -h
```

### Session Issues

```bash
# Users not getting responses:
# 1. Restart bot (clears corrupted sessions)
sudo systemctl restart mfbuilder-bot

# 2. Tell user to /reset

# 3. Check logs for errors
sudo journalctl -u mfbuilder-bot -f
```

---

## Summary

### How It All Works Together

1. **VPS runs bot 24/7** with systemd or screen
2. **Bot connects** to Telegram API using token
3. **Users interact** via Telegram messages
4. **Sessions stored** in memory (HashMap)
5. **Files accessed** from local filesystem
6. **Builds execute** in blocking tasks
7. **Results sent** back via Telegram
8. **Updates deployed** by rebuilding and restarting

### Quick Commands Reference

```bash
# Start bot (screen)
screen -S mfbuilder
./target/release/MfBuilder --bot

# Start bot (systemd)
sudo systemctl start mfbuilder-bot

# Update bot
git pull && cargo build --release && sudo systemctl restart mfbuilder-bot

# Check status
sudo systemctl status mfbuilder-bot

# View logs
sudo journalctl -u mfbuilder-bot -f

# Cleanup temp files
rm -f temp_* payload.exe out.*
```

---

**For deployment assistance, see [BOT_SETUP.md•(BOT_SETUP.md)**  
**For VPS security hardening, consult standard Linux hardening guides**
