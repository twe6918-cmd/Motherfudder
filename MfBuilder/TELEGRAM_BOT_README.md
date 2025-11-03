# MfBuilder Telegram Bot

A Telegram bot service for MfBuilder that allows users to crypt binaries via Telegram.

## Features

- ?? Key-based authentication
- ?? File upload handling (.exe only)
- ?? Feature toggles:
  - Anti Debug
  - Anti VM
  - Blacklist CIS Countries
  - UAC Bypass
  - Single Instance
  - Persistence
- ?? Automated build process
- ?? Build result delivery

## Setup

1. Create a Telegram bot via [@BotFather](https://t.me/botfather)
2. Get your bot token
3. Set up environment variables:

```bash
TELEGRAM_BOT_TOKEN=your_bot_token_here
AUTH_KEY=your_auth_key_here
```

4. Build the bot:

```bash
cd MfBuilder
cargo build --release --bin telegram_bot
```

5. Run the bot:

```bash
./target/release/telegram_bot
```

## Usage

1. Start a chat with your bot
2. Send your authentication key
3. Send `/crypt` to start
4. Upload an .exe file
5. Configure settings with `/settings` or use inline buttons
6. Send `/build` or click "Build" button
7. If the build requires an encrypted payload URL:
   - The bot will generate the encrypted payload
   - Upload it to a file hosting service (e.g., anonfile.la)
   - Send the direct download URL back to the bot
8. Receive your crypted binary!

## Commands

- `/start` - Start the bot
- `/crypt` - Start crypting a binary
- `/settings` - Configure build settings
- `/build` - Start the build process

## Notes

- The bot detects binary architecture automatically (x64/x86/NET64/NET86)
- For native builds, you'll need to provide the upload URL for the encrypted shellcode
- The bot generates encrypted payloads and waits for you to upload them manually
- Build process may take several minutes depending on the binary size
