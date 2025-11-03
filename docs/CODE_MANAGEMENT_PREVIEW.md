# Subscription Code Management Feature

**Added to MOTHERFUDDER.bat Control Panel**

---

## New Menu Option

### Main Menu Update

```
 ╔══════════════════════════════════════════════════════════════╗
 ║           🔮 MOTHERFUDDER CRYPTER - v1.0.0 🔮                ║
 ╚══════════════════════════════════════════════════════════════╝

  [1] 🔧 Install Prerequisites
  [2] 🤖 Configure Telegram Bot
  [3] 🔑 Manage Subscription Codes      (NEW!)  ← Add/Remove codes!
  [4] 🚀 Host Telegram Bot
  [5] 💻 Build CLI Mode
  [6] 🔄 Check for Updates
  [7] 📚 Open Documentation
  [8] ❓ Help & Support
  [9] 🚪 Exit
```

---

## Code Management Menu

### Options Available

```
 ╔══════════════════════════════════════════════════════════════╗
 ║           🔑 MANAGE SUBSCRIPTION CODES 🔑                     ║
 ╚══════════════════════════════════════════════════════════════╝

 Current Subscription Codes:
 • MFCRYPT-LIFETIME-2024
 • FLORIN-VIP-BETA
 • BACKDOORSKID-PRO

  [1] 📝 View Current Codes (Detailed)
  [2] ➕ Add New Code
  [3] ➖ Remove Code
  [4] 🔄 Reset to Defaults
  [5] 🔙 Back to Main Menu
```

---

## Features

### 1. View Current Codes

Shows all active subscription codes from `telegram_bot.rs`:
- Displays the exact Rust code block
- Shows line numbers
- Easy to verify what's currently configured

### 2. Add New Code

Adds a new subscription code:
- Prompts for code format (EXAMPLE-TYPE-2024)
- Validates input
- Backs up original file
- Updates `telegram_bot.rs` automatically
- Warns to rebuild bot

**Example**:
```
Enter new code: PROMO-2024-SPECIAL
✅ Code added successfully!

⚠️ IMPORTANT: You must rebuild the bot for changes to take effect!
```

### 3. Remove Code

Removes an existing code:
- Prompts for exact code to remove
- Backs up original file
- Updates `telegram_bot.rs` automatically
- Warns to rebuild bot

**Example**:
```
Enter code to remove: OLD-CODE-123
✅ Code removed successfully!
```

### 4. Reset to Defaults

Resets all codes back to the three defaults:
- MFCRYPT-LIFETIME-2024
- FLORIN-VIP-BETA
- BACKDOORSKID-PRO

Includes confirmation prompt to prevent accidents.

---

## How It Works

### Behind the Scenes

1. **Navigates** to `MfBuilder/src/`
2. **Reads** `telegram_bot.rs`
3. **Finds** the `SUBSCRIPTION_CODES` constant
4. **Modifies** using PowerShell string replacement
5. **Backs up** original file before changes
6. **Restores** backup if operation fails
7. **Reminds** user to rebuild bot

### Code Location

Modifies this section in `telegram_bot.rs`:

```rust
pub const SUBSCRIPTION_CODES: &[&str] = &[
    "MFCRYPT-LIFETIME-2024",
    "FLORIN-VIP-BETA",
    "BACKDOORSKID-PRO",
    // New codes added here automatically
];
```

---

## Usage Workflow

### Adding a Code

```
1. Run MOTHERFUDDER.bat
2. Press 3 → Manage Subscription Codes
3. Press 2 → Add New Code
4. Enter: CUSTOM-VIP-2024
5. Code is added to telegram_bot.rs
6. Press 4 → Host Bot (rebuilds automatically)
7. Bot is now accepting the new code!
```

### Removing a Code

```
1. Run MOTHERFUDDER.bat
2. Press 3 → Manage Subscription Codes
3. Press 3 → Remove Code
4. Enter: OLD-CODE-123
5. Code is removed from telegram_bot.rs
6. Press 4 → Host Bot (rebuilds automatically)
7. Old code no longer works!
```

---

## Safety Features

### Automatic Backups

Before any modification:
- Creates `telegram_bot.rs.backup`
- If operation fails, restores from backup
- Deletes backup after successful operation

### Error Handling

- Validates file exists before operation
- Checks if code is empty
- Confirms destructive operations (reset)
- Provides clear error messages

### User Warnings

Always reminds users:
```
⚠️ IMPORTANT: You must rebuild the bot for changes to take effect!

Run: MOTHERFUDDER.bat → Option 4 → Restart bot
```

---

## Benefits

### Before (Manual)

```
1. Open telegram_bot.rs in editor
2. Find SUBSCRIPTION_CODES constant
3. Carefully edit Rust syntax
4. Save file
5. Hope you didn't break anything
6. Rebuild bot
7. Restart bot
```

**Time**: 5-10 minutes  
**Risk**: High (syntax errors)  
**User-Friendly**: No

### After (Automated)

```
1. Run MOTHERFUDDER.bat
2. Press 3 → Press 2
3. Enter code
4. Done!
```

**Time**: 30 seconds  
**Risk**: Low (automatic backup)  
**User-Friendly**: Yes!

---

## Example Session

```batch
> MOTHERFUDDER.bat
Choose an option [1-9]: 3

╔══════════════════════════════════════════════════════════════╗
║           🔑 MANAGE SUBSCRIPTION CODES 🔑                     ║
╚══════════════════════════════════════════════════════════════╝

Current Subscription Codes:
• MFCRYPT-LIFETIME-2024
• FLORIN-VIP-BETA
• BACKDOORSKID-PRO

 [1] View Current Codes
 [2] Add New Code
 [3] Remove Code
 [4] Reset to Defaults
 [5] Back

Choose an option [1-5]: 2

╔══════════════════════════════════════════════════════════════╗
║                  ➕ ADD SUBSCRIPTION CODE ➕                   ║
╚══════════════════════════════════════════════════════════════╝

Format examples:
  • MYCODE-LIFETIME-2024
  • CUSTOM-VIP-123
  • PROMO-2024-SPECIAL

Enter new code: PROMO-2024-SPECIAL

Adding code: PROMO-2024-SPECIAL

✅ Code added successfully!

⚠️ IMPORTANT: You must rebuild the bot for changes to take effect!

Press any key to continue...
```

---

## Technical Details

### PowerShell Integration

Uses PowerShell for file manipulation:

**Add Code**:
```batch
powershell -Command "$content = Get-Content 'telegram_bot.rs' -Raw; 
$pattern = '(pub const SUBSCRIPTION_CODES: &\[&str\] = &\[)'; 
$replacement = '$1\n    \"%new_code%\",'; 
$content = $content -replace $pattern, $replacement; 
Set-Content 'telegram_bot.rs' $content"
```

**Remove Code**:
```batch
powershell -Command "$content = Get-Content 'telegram_bot.rs' -Raw; 
$content = $content -replace '    \"%remove_code%\",\n', ''; 
Set-Content 'telegram_bot.rs' $content"
```

### File Structure

```
MfBuilder/
├── src/
│   ├── telegram_bot.rs          ← Modified by this feature
│   ├── telegram_bot.rs.backup   ← Created during operations
│   └── ...
└── ...
```

---

## Best Practices

### Code Format

**Recommended**:
- Use uppercase letters
- Use hyphens as separators
- Include purpose/type in code
- Include year for tracking

**Examples**:
- `PREMIUM-LIFETIME-2024`
- `TRIAL-30DAYS-2024`
- `RESELLER-VIP-2024`
- `CUSTOM-CLIENT-NAME-2024`

### Security

**Do NOT share codes publicly**:
- Each code grants full bot access
- No usage limits per code
- No expiration (unless you remove them)

**Keep codes secret**:
- Share only with intended users
- Remove codes when no longer needed
- Use descriptive names for tracking

---

## Troubleshooting

### Code Not Working

**Problem**: Added code but bot doesn't accept it

**Solution**:
1. Verify code was added: Option 1 (View Codes)
2. Rebuild bot: Press 4 → Start bot
3. Wait for build to complete
4. Try code again in Telegram

### File Not Found

**Problem**: `telegram_bot.rs not found!`

**Solution**:
- Ensure you're in the correct directory
- Run from workspace root
- Check `MfBuilder/src/` exists

### Syntax Error After Edit

**Problem**: Bot won't compile after adding code

**Solution**:
- Backup is automatically restored on error
- If persistent, use Option 4 (Reset to Defaults)
- Rebuild bot

---

## Summary

**New Feature**: Subscription code management in MOTHERFUDDER.bat

**Benefits**:
- No manual file editing required
- Automatic backups before changes
- User-friendly prompts and menus
- Error handling and validation
- Quick add/remove/reset operations

**Usage**: Simple - just press 3 in the main menu!

---

**Status**: ✅ IMPLEMENTED  
**User-Friendly**: ✅ VERY  
**Time Saved**: 5-10 minutes per code change
