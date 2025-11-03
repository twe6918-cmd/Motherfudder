# UAC Bypass Information

## How It Works

The UAC bypass implemented in this crypter uses the **fodhelper.exe** technique, which is a well-known UAC bypass method that has been effective on Windows 10/11.

### Technique Details

**Target**: `fodhelper.exe` (Microsoft Settings application)
**Method**: Registry hijacking via CurVer
**Privilege Required**: User must be in Administrators group (but not elevated)

### Steps:

1. Create a custom registry class under `HKCU\Software\Classes\<RandomName>`
2. Set command to execute via `Shell\Open\command` using `conhost.exe --headless`
3. Point `ms-settings\CurVer` to the custom class
4. Launch `fodhelper.exe` (auto-elevates and executes our command)
5. Clean up registry entries

### Code Flow:

```csharp
// In UacBypass.cs
PerformUacBypass(string powershellCommand)
??? Check if already elevated ? exit
??? Check if user is administrator ? exit if not
??? Generate random class name
??? Create registry entries:
?   ??? HKCU\Software\Classes\<RandomName>\Shell\Open\command
?   ??? HKCU\Software\Classes\ms-settings\CurVer
??? Execute fodhelper.exe
??? Wait and cleanup registry
??? Exit process (elevated instance continues)
```

## Integration with Defender Exclusion

When both **UAC Bypass** and **Defender Exclusion** are enabled:

1. **First execution** (non-elevated):
   - UAC bypass triggers
   - Elevates via fodhelper.exe
   - Launches elevated instance

2. **Second execution** (elevated):
   - Has admin rights
   - Adds Windows Defender exclusions **silently**
   - No UAC prompts
   - No user interaction needed

This creates a **silent defender exclusion** - the holy grail of AV evasion!

## Compatibility

? **Works on**:
- Windows 10 (all versions)
- Windows 11 21H2, 22H2, 23H2

?? **Requirements**:
- User must be in Administrators group
- UAC must be enabled (ironically!)
- User account must not be already elevated

? **Does NOT work**:
- Standard user (non-admin)
- Already elevated process
- Windows 7/8/8.1 (different technique needed)

## Detection Status

**As of 2024**:
- ? Still works on most systems
- ?? Some EDR solutions detect this
- ?? Microsoft Defender may flag the registry manipulation
- ? Most AV products don't detect it when properly obfuscated

**Evasion techniques used**:
- Random class names (prevents signature detection)
- Headless conhost execution (reduces visibility)
- Quick cleanup (minimizes forensic artifacts)
- Combined with other features (blends in)

## Why fodhelper.exe?

`fodhelper.exe` is a legitimate Windows binary that:
- **Auto-elevates**: Marked with `autoElevate="true"` in its manifest
- **Trusted**: Signed by Microsoft
- **Always present**: Part of Windows features
- **Registry-aware**: Checks HKCU for settings (our attack vector)

## Security Implications

This technique demonstrates why:
- User account control isn't a security boundary
- Admin users can escalate to high integrity without prompts
- Registry-based hijacking is still viable
- Defense in depth is necessary

## Alternative Techniques

If fodhelper fails, consider:
- **sdclt.exe** (similar registry hijacking)
- **eventvwr.exe** (MSC file execution)
- **computerdefaults.exe** (ProgID hijacking)
- **Disk Cleanup** (scheduled task elevation)

## References

- UAC bypass database: https://github.com/hfiref0x/UACME
- Windows internals on auto-elevation
- Microsoft's UAC documentation

---

**Created by Florin**
