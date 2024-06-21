# Clear the screen
Clear-Host

# Set the title to WebX Windows Installation
$host.ui.RawUI.WindowTitle = "WebX Windows Compilation"

# Create our helper function for error handling
function Write-Error([string]$message, [bool]$exit = $false, [bool]$newLineMsg = $true)
{
    # Write the [!] in dark red, and the message in red
    Write-Host "[!] " -ForegroundColor DarkRed -NoNewline
    if ($newLineMsg) { Write-Host $message -ForegroundColor Red }
    else { Write-Host $message -ForegroundColor Red -NoNewline }

    # Exit the script if specified
    if ($exit) { exit }
}

# Setup constant variables for Rust installer URLs
$RUSTUP_32bit = "https://win.rustup.rs/i686"
$RUSTUP_64bit = "https://win.rustup.rs/x86_64"

# Check if we are running as an administrator, if not, relaunch as an administrator
$wid=[System.Security.Principal.WindowsIdentity]::GetCurrent()
$prp=new-object System.Security.Principal.WindowsPrincipal($wid)

if (-not $prp.IsInRole([System.Security.Principal.WindowsBuiltInRole]::Administrator))
{
    Write-Error "You must run this script as an administrator. Relaunch as an administrator? [Y/N]: " $false $false
    $choice = Read-Host
    
    if ($choice -eq "Y" -or $choice -eq "y")
    {
        try {
            Start-Process powershell.exe "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
        }
        catch {
            Write-Error "Failed to relaunch as administrator. Aborting!" $true
        }
        exit
    }
    else
    {
        Write-Error "User chose not to relaunch as an administrator. Admin rights are required to compile WebX. Aborting!" $true
    }
}

# Ask the user if they want to continue
Write-Host "This script will install the necessary tools to compile WebX on Windows. This includes Rust, Cargo, and the necessary dependencies." -ForegroundColor Green
Write-Host "Do you want to continue? [Y/N]: " -NoNewline
$choice = Read-Host

if ($choice -ne "Y" -and $choice -ne "y")
{
    Write-Error "User chose not to continue. Aborting!" $true
}

Write-Host "DO NOT CLOSE ANY WINDOWS THAT OPEN DURING THE INSTALLATION PROCESS! DOING SO WILL CAUSE THE INSTALLATION TO FAIL!" -ForegroundColor Red
Write-Host "Continuing in 5 seconds..." -ForegroundColor Yellow

Start-Sleep -Seconds 5
Write-Host "Starting installation..." -ForegroundColor Green

# Check if Rust is already installed
if (Test-Path "$env:USERPROFILE\.cargo\bin\rustup.exe")
{
    Write-Host "Rust is already installed. Checking for updates..." -ForegroundColor Green
    try {
        Start-Process -FilePath "$env:USERPROFILE\.cargo\bin\rustup.exe" -ArgumentList "update" -Wait
    }
    catch {
        Write-Error "Failed to update Rust: $($_.Exception.Message). Aborting!" $true

        # Keep the window open so the user can read the error message and wait for input to close
        Write-Host "Press any key to close this window..." -ForegroundColor Yellow
        $null = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
        exit
    }
}
else
{
    Write-Host "Rust is not installed. Downloading and installing Rust..." -ForegroundColor Green
    try {
        # Download the Rust installer
        $rustup = "$env:TEMP\rustup-init.exe"
        if ([System.Environment]::Is64BitOperatingSystem)
        {
            Invoke-WebRequest -Uri $RUSTUP_64bit -OutFile $rustup
        }
        else
        {
            Invoke-WebRequest -Uri $RUSTUP_32bit -OutFile $rustup
        }

        # Run the Rust installer
        Start-Process -FilePath $rustup -ArgumentList "-y" -Wait
        Remove-Item $rustup
    }
    catch {
        Write-Error "Failed to download or install Rust: $($_.Exception.Message). Aborting!" $true

        # Keep the window open so the user can read the error message and wait for input to close
        Write-Host "Press any key to close this window..." -ForegroundColor Yellow
        $null = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
        exit
    }
}

# Install Rust components
Write-Host "Installing Rust components..." -ForegroundColor Green
try {
    Start-Process -FilePath "$env:USERPROFILE\.cargo\bin\rustup.exe" -ArgumentList "toolchain install stable-gnu" -Wait
    Start-Process -FilePath "$env:USERPROFILE\.cargo\bin\rustup.exe" -ArgumentList "default stable-gnu" -Wait
}
catch {
    Write-Error "Failed to install Rust components: $($_.Exception.Message). Aborting!" $true

    # Keep the window open so the user can read the error message and wait for input to close
    Write-Host "Press any key to close this window..." -ForegroundColor Yellow
    $null = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
    exit
}

Write-Host "Preparing to install MSYS2..." -ForegroundColor Green

# Check if MSYS2 is already installed. Default location is C:\msys32 or C:\msys64 if running 64-bit
$msysname = if ([System.Environment]::Is64BitOperatingSystem) { "msys64" } else { "msys32" }

if (Test-Path "C:\$msysname\msys2.exe")
{
    Write-Host "msys2 is already installed. Skipping..." -ForegroundColor Green
}
else
{
    Write-Host "Downloading and installing msys2..." -ForegroundColor Green
    try {
        # Download the MSYS2 installer
        $msys2 = "$env:TEMP\msys2.exe"
        $msys2arch = if ([System.Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }

        Write-Host "Downloading MSYS2 installer from URI: https://repo.msys2.org/distrib/msys2-$msys2arch-latest.exe" -ForegroundColor Yellow
        Invoke-WebRequest -Uri "https://repo.msys2.org/distrib/msys2-$msys2arch-latest.exe" -OutFile $msys2

        Write-Host "IMPORTANT: PLEASE READ THE FOLLOWING INSTRUCTIONS CAREFULLY!" -ForegroundColor Yellow
        Write-Host "1. When the MSYS2 installer opens, press Enter to install MSYS2 to the default location (C:\$msysname)." -ForegroundColor Yellow
        Write-Host "2. After installation, close the MSYS2 installer and return to this script." -ForegroundColor Yellow
        Write-Host "3. Untick the 'Run MSYS2 now' checkbox and click Finish." -ForegroundColor Yellow
        Write-Host "Press any key to continue..." -ForegroundColor Yellow

        # Wait for user to read instructions
        $null = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

        # Run the MSYS2 installer
        Start-Process -FilePath $msys2 -ArgumentList "-y" -Wait
        Remove-Item $msys2
    }
    catch {
        Write-Error "Failed to download or install MSYS2: $($_.Exception.Message). Aborting!" $false
        
        # Keep the window open so the user can read the error message and wait for input to close
        Write-Host "Press any key to close this window..." -ForegroundColor Yellow
        $null = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
        exit
    }
}

# Print warning not to close any MSYS2 windows
Write-Host "DO NOT CLOSE ANY MSYS2 WINDOWS THAT OPEN DURING THE FOLLOWING STEPS! DOING SO WILL CAUSE THE INSTALLATION TO FAIL!" -ForegroundColor Red

# Sync MSYS2 packages
Write-Host "Running MSYS2 package sync and package installation..." -ForegroundColor Green
try {
    Start-Process "C:\$msysname\msys2_shell.cmd" -ArgumentList "-defterm -here -no-start -mingw32 -c `"pacman -Syu --noconfirm && pacman -S --noconfirm mingw-w64-x86_64-toolchain base-devel mingw-w64-x86_64-gtk4 mingw-w64-x86_64-gettext mingw-w64-x86_64-libxml2 mingw-w64-x86_64-librsvg mingw-w64-x86_64-pkgconf mingw-w64-x86_64-gcc mingw-w64-x86_64-libadwaita mingw-w64-x86_64-lua && exit`"" -Wait

}
catch {
    Write-Error "Failed to sync MSYS2 packages. Aborting!" $true
}

# Add MSYS2 directories to the PATH
Write-Host "Adding MSYS2 directories to the PATH..." -ForegroundColor Green
try {
    $msyspath = "C:\$msysname\mingw64\bin;C:\$msysname\mingw64\lib;C:\$msysname\mingw64\include"
    $oldpath = [System.Environment]::GetEnvironmentVariable("PATH", [System.EnvironmentVariableTarget]::Machine)
    $newpath = "$msyspath;$oldpath"
    [System.Environment]::SetEnvironmentVariable("PATH", $newpath, [System.EnvironmentVariableTarget]::Machine)
}
catch {
    Write-Error "Failed to add MSYS2 directories to the PATH. Aborting!" $true
}

Write-Host "Installation complete! You should now be able to cargo run (or cargo build) WebX." -ForegroundColor Green
Write-Host "Press any key to close this window..." -ForegroundColor Yellow

# Wait for user to read message
$null = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
exit
