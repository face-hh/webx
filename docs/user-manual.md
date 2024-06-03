# WebX user manual

> **TL;DR** Download the executable / package for your system and run it. Easy!

This is the guide to install Napture, WebX's browser.

## Arch Linux
Run **`yay -S napture`**. It's available on AUR.

## Nix[OS]
**Flakes**: The repository provides a flake which exposes an overlay providing the webx package, so you could just add the input in your flake.nix file

```nix{3}
{
    inputs = {
        webx.url = "github:face-hh/webx";
    };
}
```

Then add it to your overlays and install it

```nix{3}
{ inputs, ... }: {
    nixpkgs.overlays = [
        inputs.webx.overlays.x86_64-linux.default
    ];
}
```

> For now, only tested on x86_64-linux, but may work on others aswell, just change the arch

Add it to either home.packages (home manager) or environment.systemPackages (global packages).

```nix{2}
home.packages = with pkgs; [
    webx
];
```

Then you could just launch it using `webx` in your terminal.

## Linux
- For now, you have to download [Rust](https://www.rust-lang.org/tools/install). Then, you just need to open `install-linux` as an executable (if you can't execute it, first do `sudo chmod +x ./install-linux`, then you should be able to install).
## macOS
- For now, you have to download [Rust](https://www.rust-lang.org/tools/install) and [Homebrew](https://brew.sh). Then, you just need to open `install-macos` as an executable (if you can't execute it, first do `chmod +x ./install-macos`, then you should be able to install).
## Windows
- Install the executable from the release tab. It's a self-extractor with WinRAR because it has a lot of DLLs.
::: info
Go [here](https://github.com/face-hh/webx/releases) for the releases tab. Always download the **Latest** one.
:::