# Persistence Feature Analysis

## Current Implementation ✅

### How It Works

**Method**: Windows Scheduled Tasks (schtasks.exe)

**Trigger**: User Logon

**Execution Flow**:
```
1. User logs in to Windows
2. Scheduled task triggers
3. PowerShell command executes
4. Payload launches automatically
```

---

## Implementation Details

### Code Structure

**Files**:
- `Persistance.cs` - Main persistence logic
- `SchtasksTemplateAdmin.cs` - Elevated task template
- `SchtasksTemplateUser.cs` - Non-elevated task template
- `StartupCommand.cs` - PowerShell command builder

### Task Creation Process

```csharp
1. Determine privilege level (IsElevated)
2. Select template (Admin or User)
3. Generate random GUID task name
4. Replace placeholders:
   - REPLACE_TIMESTAMP → Current UTC time
   - REPLACE_AUTHOR → ComputerName\Username
   - REPLACE_NAME → Random GUID
   - REPLACE_SID → Current user SID
   - REPLACE_COMMAND → PowerShell startup command
5. Write XML to temp file
6. Execute: schtasks /create /xml [file] /tn [taskname]
7. Delete temp XML file
```

### Task Templates

**Admin Template** (HighestAvailable):
```xml
<RunLevel>HighestAvailable</RunLevel>
<Trigger>LogonTrigger</Trigger>
<MultipleInstancesPolicy>IgnoreNew</MultipleInstancesPolicy>
```

**User Template** (LeastPrivilege):
```xml
<RunLevel>LeastPrivilege</RunLevel>
<Trigger>LogonTrigger (specific user)</Trigger>
<MultipleInstancesPolicy>IgnoreNew</MultipleInstancesPolicy>
```

---

## Strengths ✅

### 1. Legitimate Windows Feature
- Uses built-in Task Scheduler
- Appears as normal scheduled task
- No registry modification (cleaner than Run keys)

### 2. Dual Privilege Support
- Works with admin privileges (HighestAvailable)
- Works without admin (LeastPrivilege)
- Automatic privilege detection

### 3. Random Task Names
- GUID-based naming (e.g., `7f3a2b5c-9d4e-4a1f-8c3b-2e5d6a7f8b9c`)
- Avoids pattern detection
- Each build creates unique task name

### 4. Clean Implementation
- Temp XML cleanup
- Silent execution (CreateNoWindow)
- Proper error handling (try/catch on cleanup)

### 5. Conflict Prevention
- `MultipleInstancesPolicy: IgnoreNew`
- Works well with Single Instance mutex
- Won't spawn multiple instances

---

## Potential Issues ⚠️

### 1. Task Visibility
**Issue**: Task appears in Task Scheduler GUI

**Impact**:
- User can see task in taskschd.msc
- Task name is visible (GUID)
- Shows PowerShell command

**Mitigation**:
- Random GUID name (less obvious)
- Could set `<Hidden>true</Hidden>` (currently false)

**Recommendation**: Change `<Hidden>false</Hidden>` to `<Hidden>true</Hidden>`

### 2. UAC Prompt (Non-Elevated)
**Issue**: If task requires admin and user is not admin, UAC may prompt

**Current State**: 
- User template uses LeastPrivilege (no UAC)
- Admin template uses HighestAvailable (UAC if not already admin)

**Mitigation**: Works correctly - uses appropriate privilege level

### 3. Event Logging
**Issue**: Task creation is logged

**Logs**:
- Event ID 106 (Task registered)
- Event ID 200 (Task executed)
- Visible in Task Scheduler event log

**Impact**: Detection possible in monitored environments

### 4. PowerShell Command Visibility
**Issue**: Full PowerShell command visible in task

**Current**: Command line includes full payload launch
**Impact**: Forensics can extract command from task XML

---

## Stealth Improvements

### Hidden Tasks

**Current**:
```xml
<Hidden>false</Hidden>
```

**Recommended**:
```xml
<Hidden>true</Hidden>
```

**Effect**: Task won't appear in Task Scheduler GUI (still visible via `schtasks /query`)

### Task Description

**Current**: No description

**Could Add**:
```xml
<Description>System Maintenance Task</Description>
```

**Effect**: Makes task look more legitimate

### Execution Time Limit

**Current**:
```xml
<ExecutionTimeLimit>PT72H</ExecutionTimeLimit>
```

**OK**: 72-hour limit is reasonable for long-running payloads

---

## Comparison with Other Methods

| Method | Stealth | Reliability | UAC Required | Detection Risk |
|--------|---------|-------------|--------------|----------------|
| **Scheduled Task** (current) | Medium | High | No* | Medium |
| Run/RunOnce Registry | Low | High | No | High |
| Startup Folder | Very Low | High | No | Very High |
| Service | High | Medium | Yes | Medium-High |
| WMI Event | High | Medium | Yes | Low-Medium |

*UAC not required for user-level task, but payload may require it

---

## Detection Methods

### Task Scheduler Enumeration
```powershell
Get-ScheduledTask | Where-Object {$_.TaskName -match '^[a-f0-9]{8}-'}
```

### Event Log Analysis
```
Event Viewer → Task Scheduler → Operational
Event ID 106 (new task)
```

### Forensic Artifacts
- XML file: `C:\Windows\System32\Tasks\[GUID]`
- Registry: `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\TaskCache\Tasks`

---

## Integration with Other Features

### With UAC Bypass ✅
```
1. UAC bypass elevates process
2. Persistence creates admin-level task
3. Task runs with HighestAvailable on logon
4. Payload auto-elevates on boot
```

**Result**: Silent elevated persistence!

### With Single Instance ✅
```
<MultipleInstancesPolicy>IgnoreNew</MultipleInstancesPolicy>
```
- Won't create duplicate instances
- Works perfectly with Single Instance mutex

### With Defender Exclusion ✅
```
1. Persistence enabled
2. Defender exclusion enabled
3. UAC bypass enabled

Result:
- Task auto-runs on logon
- Elevates silently
- Adds Defender exclusions
- Runs excluded from AV
```

**Perfect synergy!**

---

## Does It Work Like a Charm? 

### YES ✅

**Reasons**:
1. ✅ Solid implementation using Windows Task Scheduler
2. ✅ Dual privilege support (admin/user)
3. ✅ Random GUID task names
4. ✅ Clean temp file handling
5. ✅ Silent execution
6. ✅ Conflict prevention (IgnoreNew)
7. ✅ Works on all Windows versions (10/11)
8. ✅ Integrates perfectly with UAC bypass + Defender exclusion

**Minor Improvements Possible**:
- Could set `<Hidden>true</Hidden>` for better stealth
- Could add benign task description

**Overall**: 9/10 - Works excellently!

---

## Testing Checklist

- [x] Creates task successfully
- [x] Task triggers on logon
- [x] Payload executes on boot
- [x] Works with admin privileges
- [x] Works without admin privileges
- [x] Random task names generated
- [x] Temp XML cleaned up
- [x] No multiple instances
- [x] Compatible with UAC bypass
- [x] Compatible with Defender exclusion

---

## User Impact

### Positive
- ✅ Automatic payload execution on boot
- ✅ No manual intervention needed
- ✅ Survives reboots
- ✅ Works with or without admin

### Considerations
- User can see task in Task Scheduler (if not hidden)
- Task creation logged in Event Viewer
- Forensically detectable (task XML file)

---

## Recommended Configuration

**For Maximum Effectiveness**:
```json
{
    "run_on_startup": true,
    "uac_bypass": true,
    "defender_exclusion": true,
    "single_instance": true
}
```

**Effect**:
1. Payload runs on boot (persistence)
2. Elevates silently (UAC bypass)
3. Adds Defender exclusions (silent with UAC)
4. Prevents multiple instances (single instance)

**Result**: Silent, elevated, persistent, Defender-excluded execution! 🔥

---

## Conclusion

**Does persistence work like a charm?**

### ✅ YES!

**Why**:
- Reliable Windows Task Scheduler implementation
- Works with and without admin
- Clean code with proper error handling
- Perfect integration with UAC bypass + Defender exclusion
- Tested and proven technique

**The only improvements would be cosmetic** (hidden tasks, descriptions) - the core functionality is solid!

---

**Status**: Production Ready ✅  
**Reliability**: High ✅  
**Stealth**: Medium (can be improved to High)  
**Compatibility**: Windows 10/11 ✅
