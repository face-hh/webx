@echo off
if "%1" == "next" goto :next


:: BatchGotAdmin
:-------------------------------------
REM  --> Check for permissions
IF "%PROCESSOR_ARCHITECTURE%" EQU "amd64" (
>nul 2>&1 "%SYSTEMROOT%\SysWOW64\cacls.exe" "%SYSTEMROOT%\SysWOW64\config\system"
) ELSE (
>nul 2>&1 "%SYSTEMROOT%\system32\cacls.exe" "%SYSTEMROOT%\system32\config\system"
)

REM --> If error flag set, we do not have admin.
if '%errorlevel%' NEQ '0' (
    echo Requesting administrative privileges...
    goto UACPrompt
) else ( goto gotAdmin )

:UACPrompt
    echo Set UAC = CreateObject^("Shell.Application"^) > "%temp%\getadmin.vbs"
    set params= %*
    echo UAC.ShellExecute "cmd.exe", "/c ""%~s0"" %params:"=""%", "", "runas", 1 >> "%temp%\getadmin.vbs"

    "%temp%\getadmin.vbs"
    del "%temp%\getadmin.vbs"
    exit /B

:gotAdmin
    pushd "%CD%"
    CD /D "%~dp0"
:--------------------------------------

@echo off
title Bussin Napture Windows Compiler - By NEOAPPS
rustc --version
IF %ERRORLEVEL% NEQ 0 (
    echo Rust is not installed. Please install Rust before proceeding. >error.txt
    set pc=i686
    if "%ProgramFiles(x86)%" == "C:\Program Files (x86)" set pc=x86_64
    echo Downloading Rust Installer for your PC...
if %pc% NEQ i686 (    
powershell -Command "Invoke-WebRequest -Uri https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -OutFile rustup-init.exe"
) else (
powershell -Command "Invoke-WebRequest -Uri https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe -OutFile rustup-init.exe"
)
    echo Downloaded rustup-init.exe. Complete installation then open the compiler again. >>error.txt
    rustup-init
    setx PATH "%PATH%;%USERPROFILE%\.cargo\bin" /m
    exit /b
)
rustup toolchain install stable-gnu
rustup default stable-gnu
SET MSYS2_URL=https://github.com/msys2/msys2-installer/releases/download/nightly-x86_64/msys2-base-x86_64-latest.sfx.exe
SET MSYS2_EXE=msys2-installer.exe
if not exist %MSYS2_EXE% powershell -Command "Invoke-WebRequest -Uri %MSYS2_URL% -OutFile %MSYS2_EXE%"
if not exist C:\msys64 %MSYS2_EXE% -y -oC:\
start C:\msys64\msys2_shell.cmd -defterm -here -no-start -mingw32 -c "pacman -Syu --noconfirm && pacman -S --noconfirm mingw-w64-x86_64-toolchain base-devel mingw-w64-x86_64-gtk4 mingw-w64-x86_64-gettext mingw-w64-x86_64-libxml2 mingw-w64-x86_64-librsvg mingw-w64-x86_64-pkgconf mingw-w64-x86_64-gcc mingw-w64-x86_64-libadwaita mingw-w64-x86_64-lua && exit"
pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-gettext mingw-w64-x86_64-libxml2 mingw-w64-x86_64-librsvg mingw-w64-x86_64-pkgconf mingw-w64-x86_64-gcc
setx PATH "C:\msys64\usr\bin;%PATH%" /m
setx GSK_RENDERER cairo /m
REM if not exist pkg-config_0.26-1_win32.zip powershell -Command "Invoke-WebRequest -Uri http://ftp.gnome.org/pub/gnome/binaries/win32/dependencies/pkg-config_0.26-1_win32.zip -OutFile pkg-config_0.26-1_win32.zip"
REM powershell -Command "Expand-Archive -Path pkg-config_0.26-1_win32.zip"
REM move pkg-config_0.26-1_win32\bin C:\msys64\mingw64
pacman -S mingw-w64-x86_64-gdk-pixbuf2
setx PKG_CONFIG_PATH "C:\msys64\mingw64\lib\pkgconfig" /m


:next
echo Move your 'webx-master' folder here and then
pause
cd webx-master
cd napture
cargo run
echo Job Done.
pause
