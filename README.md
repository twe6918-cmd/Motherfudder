# Motherfudder

**Motherfudder** is a crypter for Windows executables designed for red teaming operations.<br>
This crypter includes advanced protection features including AMSI/ETW bypasses, anti-debugging, anti-VM, and more.<br>

## Features

- **Global AMSI Bypass** - Implementation by [Chainski](https://github.com/Chainski/GlobalAMSIBypass)
- **Anti-Debug Protection** - Detects and prevents debugger attachment
- **Anti-VM Detection** - Identifies virtualized environments
- **CIS Country Blacklisting** - Geo-based execution control
- **UAC Bypass** - Privilege escalation techniques
- **Persistence Mechanisms** - Startup and scheduled task persistence
- **ETW Patching** - Event Tracing for Windows bypass
- **Payload Encryption** - RC4 encryption with custom key derivation
- **Code Obfuscation** - String and integer obfuscation for .NET assemblies

## Quick Start (Automated Build)

The easiest way to build and use Motherfudder is with the automated build script:

1. **Run the automation script:**
   ```batch
   main.bat
   ```

2. **The script will automatically:**
   - Check for prerequisites (Visual Studio, Rust)
   - Build MfObfDotNet
   - Build MfRunner (x86 and x64)
   - Build MfBuilder
   - Create an `output` directory with all required files

3. **After the build completes:**
   - Place your `payload.exe` in the `output` directory
   - (Optional) Edit `build.json` to configure protection features
   - Run `MfBuilder.exe`
   - Upload encrypted shellcode when prompted
   - Paste the download link
   - Your crypted `stub.bat` will be generated

## Manual Build Instructions

See `Guide.txt` for detailed manual build instructions.

### Prerequisites

- Visual Studio with .NET Framework 4.8
- Rust (install from [rustup.rs](https://rustup.rs/))
- For native payloads:
  - [Donut](https://github.com/TheWover/donut)
  - [SGN](https://github.com/EgeBalci/sgn)

## Configuration

Edit `build.json` to customize protection features:

```json
{
    "file_extension": "BAT",          // Output type: BAT or EXE
    "anti_debug": true,                // Enable anti-debugging
    "anti_virtual_machine": true,      // Enable anti-VM detection
    "blacklist_cis_countries": true,   // Block execution in CIS countries
    "uac_bypass": true,                // Enable UAC bypass
    "single_instance": true,           // Allow only one instance
    "run_on_startup": false,           // Enable persistence
    "binder": false                    // Enable file binding
}
```

## Credits

- **GlobalAMSIBypass** - [Chainski](https://github.com/Chainski/GlobalAMSIBypass)

## Disclaimer

This tool is for authorized red team operations and security research only. Unauthorized use is illegal.

### Please leave a star if this project helps you
