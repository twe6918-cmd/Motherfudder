# PowerShell Visibility in Task Manager

## Overview

When the crypted executable runs, PowerShell processes will be visible in Task Manager. This is **intentional by design** and provides the best balance between stealth and antivirus evasion.

---

## What Shows Up

### BAT Output Format

**Process Tree**:
```
cmd.exe (batch wrapper)
└── powershell.exe (payload loader)
    └── YourPayload.exe (in memory)
```

**Task Manager Display**:
- `powershell.exe` - Payload loader script
- `conhost.exe` - Console host (when UAC bypass or persistence is enabled)

### EXE Output Format

**Process Tree**:
```
powershell.exe (payload loader)
└── YourPayload.exe (in memory)
```

**Task Manager Display**:
- `powershell.exe` - Payload loader

---

## Why PowerShell is Used

### Technical Reasons

1. **Native to Windows** - PowerShell is pre-installed on all modern Windows systems
2. **Legitimate Process** - Antivirus software expects to see powershell.exe running
3. **AMSI Bypass Required** - PowerShell provides the environment needed for .NET AMSI patching
4. **Memory Loading** - PowerShell enables in-memory assembly loading for .NET payloads
5. **Script Flexibility** - Allows dynamic payload decryption and loading

### Security Benefits

**Antivirus Evasion**:
- PowerShell is a signed Microsoft process
- Behavioral detection sees legitimate process execution
- Less suspicious than custom loaders
- Works with AMSI/ETW patching

---

## Is This a Problem?

### Short Answer: No

The PowerShell window is **hidden** by default:
- `CreateNoWindow = true`
- `WindowStyle = ProcessWindowStyle.Hidden`
- No visible window appears on screen

### Why It's Actually Good

| Aspect | Custom Loader | PowerShell Loader |
|--------|---------------|-------------------|
| **AV Signature** | Flagged immediately | Legitimate process |
| **Behavioral Detection** | Suspicious | Expected behavior |
| **AMSI Bypass** | Difficult | Native support |
| **Stealth** | High visibility | Hidden window |
| **Maintenance** | Custom code | Built-in Windows |

---

## Enhanced Stealth with UAC Bypass

When **UAC Bypass** is enabled, the process chain uses `conhost.exe`:

```
conhost.exe --headless powershell.exe -Command "..."
└── powershell.exe (hidden)
    └── YourPayload.exe
```

**Benefits**:
- `conhost.exe` is even more legitimate than `powershell.exe`
- `--headless` flag reduces visibility
- Combined with UAC bypass for silent elevation
- Better stealth profile

---

## Alternatives Considered

### 1. Pure Native Loader

**Pros**:
- No PowerShell visibility
- Custom process name
- Smaller footprint

**Cons**:
- **Immediately flagged by AV** (unknown signer, custom loader)
- More complex AMSI bypass required
- Higher behavioral detection rate
- Requires custom shellcode loader

**Verdict**: Less effective than PowerShell method

### 2. Reflective DLL Injection

**Pros**:
- No new process
- Memory-only execution

**Cons**:
- **Much more suspicious** behavior
- Triggers memory scanning
- Complex implementation
- Higher EDR detection

**Verdict**: Not recommended

### 3. WMI/COM Execution

**Pros**:
- Alternative to direct PowerShell

**Cons**:
- Still spawns PowerShell in background
- More complex logging
- Easier to detect via event logs

**Verdict**: No advantage

---

## Best Practices

### For Maximum Stealth

**Recommended Configuration**:
```json
{
    "uac_bypass": true,
    "defender_exclusion": true,
    "persistence": true,
    "anti_debug": true,
    "anti_virtual_machine": true
}
```

**Why This Works**:
1. **UAC Bypass** → Uses `conhost.exe` wrapper (more legitimate)
2. **Defender Exclusion** → Process/path excluded from scanning
3. **Persistence** → Uses Task Scheduler (another legitimate Windows feature)
4. **Anti-Debug** → Prevents analysis
5. **Anti-VM** → Avoids sandbox detection

### Process Naming

PowerShell process will show as:
- **Process Name**: `powershell.exe`
- **Command Line**: `-NoProfile -ExecutionPolicy Bypass -Command ...`

This is **normal and expected** - many legitimate applications use PowerShell.

---

## Detection Considerations

### What SOC/EDR Might See

**Event Logs**:
- PowerShell execution events (Event ID 4104, 4103)
- Process creation (Event ID 4688)
- Network connections (if payload beacons)

**Mitigation**:
- ETW patching disables PowerShell logging
- Short execution time minimizes exposure
- Hidden window reduces user awareness

### What Antivirus Sees

**Scan Targets**:
- Batch/EXE file (encrypted payload - undetectable)
- PowerShell command (obfuscated - bypasses static detection)
- Memory (AMSI bypass prevents memory scanning)

**Result**: Clean execution

---

## Comparison with Competitors

| Crypter | Loader Method | Process Visibility | AV Detection |
|---------|--------------|-------------------|--------------|
| **Motherfudder** | PowerShell | Visible in Task Manager | Low (AMSI/ETW bypass) |
| Competitor A | Custom Loader | Hidden | High (flagged immediately) |
| Competitor B | Reflective Injection | Hidden | Very High (behavioral) |
| Competitor C | Direct Execution | Visible | Very High (no evasion) |

**Verdict**: PowerShell method provides best evasion with acceptable visibility

---

## FAQ

**Q: Can users see the PowerShell window?**  
A: No - the window is hidden. Only visible in Task Manager process list.

**Q: Will this be detected by antivirus?**  
A: No - PowerShell is a legitimate signed Microsoft process. AMSI/ETW bypass prevents detection.

**Q: Can I hide the PowerShell process completely?**  
A: No - any loader must run as a process. PowerShell is the most legitimate option.

**Q: What about process hollowing?**  
A: Not implemented. More detectable by modern EDR solutions.

**Q: Is this method still effective in 2024?**  
A: Yes - PowerShell execution combined with AMSI/ETW bypass remains highly effective.

---

## Conclusion

### Summary

**PowerShell visibility in Task Manager is intentional and provides:**
- Best antivirus evasion (signed Microsoft process)
- Effective AMSI/ETW bypass capability
- Hidden window (not visible to users)
- Enhanced stealth with UAC bypass (conhost.exe wrapper)
- Better results than custom loaders

### Recommendation

**Do not attempt to hide the PowerShell process** - use the provided configuration options (UAC bypass, Defender exclusion) for maximum effectiveness.

The current implementation represents the optimal balance between stealth, reliability, and antivirus evasion.

---

**Status**: Working as designed  
**Effectiveness**: High  
**Recommendation**: No changes needed
