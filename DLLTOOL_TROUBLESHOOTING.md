# Troubleshooting dlltool.exe Error

## What is dlltool.exe?

`dlltool.exe` is a MinGW/MSYS2 tool used for creating import libraries from DLLs. It's typically used when building native C/C++ code with MinGW.

## Why am I seeing this error?

If you're seeing a `dlltool.exe` error when building your C# project, it's likely because:

1. **MSBuild is trying to use MinGW tools** - This shouldn't happen for pure C# projects (.NET Framework 4.8), but can occur if:
   - MinGW/MSYS2 is in your PATH and MSBuild is picking it up
   - Visual Studio/MSBuild is misconfigured to use MinGW tools
   - There's a build target or task that's incorrectly trying to use dlltool

2. **The tool isn't found** - `dlltool.exe` might not be in your PATH, or MSBuild is looking for it in the wrong location

## Solutions

### Solution 1: Check MSBuild Output (Recommended)

The code has been updated to capture and display MSBuild output. When you run the build, you should now see:
- **Stdout** - Normal build output
- **Stderr** - Error messages including the dlltool.exe error details
- **Exit code** - MSBuild exit status

This will help identify exactly where and why dlltool.exe is being called.

### Solution 2: Ensure MinGW/MSYS2 Isn't Interfering

1. **Check your PATH environment variable**:
   - Open System Properties → Environment Variables
   - Look for MinGW or MSYS2 paths in your PATH
   - If found and not needed, temporarily remove them

2. **Verify MSBuild is using the correct toolchain**:
   - MSBuild should use the Microsoft C# compiler (csc.exe), not MinGW tools
   - The project file (`MfRunner.csproj`) doesn't reference any native libraries

### Solution 3: If dlltool.exe is Required (Unlikely)

If for some reason dlltool.exe is actually needed:

1. **Install MinGW/MSYS2**:
   - Download from: https://www.mingw-w64.org/
   - Or install via MSYS2: https://www.msys2.org/
   - Add the `bin` directory to your PATH

2. **Verify it's accessible**:
   ```cmd
   dlltool.exe --version
   ```

### Solution 4: Check for Custom Build Targets

If the error persists, check if there are any:
- Custom `.targets` files in the project
- NuGet packages that might be adding native build steps
- MSBuild extensions that modify the build process

## What Changed in the Code?

The `ms_build()` function in `mf_runner.rs` has been updated to:
- ✅ Capture and display MSBuild stdout/stderr
- ✅ Check MSBuild exit status
- ✅ Provide better error messages
- ✅ Show exit codes when builds fail

This will help you see the actual error message from MSBuild, which will tell you exactly why dlltool.exe is being called.

## Next Steps

1. **Run the build again** - You should now see detailed error output
2. **Look for the dlltool.exe error** in the stderr output
3. **Check the error message** - It will tell you:
   - Where dlltool.exe is being called from
   - What command is trying to use it
   - Why it's failing (not found, wrong arguments, etc.)

## Common Error Messages

- `'dlltool.exe' is not recognized` → dlltool.exe not in PATH
- `Cannot find dlltool.exe` → MSBuild looking for it in wrong location
- `dlltool.exe: error: ...` → dlltool.exe found but arguments are wrong

The updated code will show you the exact error message so you can diagnose the issue more effectively.
