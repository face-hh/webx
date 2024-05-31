{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    webx-src = {
      url = "github:face-hh/webx";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, ... } @inputs: flake-utils.lib.eachDefaultSystem(system: let
    pkgs = import nixpkgs {inherit system overlays;};
    package = pkgs.callPackage ./default.nix {};

    overlays = [
      (_: _: {
        inherit (inputs) webx-src;
      })
    ];
  in {
    packages.default = package;

    overlays.default = (final: _: {
      webx = final.callPackage ./default.nix {};
    });

    devShells.default = let inherit (pkgs) mkShell; in mkShell {
      name = "dev";
      nativeBuildInputs = [pkgs.pkg-config];
      buildInputs = with pkgs; [package cargo rustc];

      propagatedBuildInputs = with pkgs; [
        glib
        pango
        gdk-pixbuf
        openssl_3_3
        graphene
        gtk4
        libadwaita
        lua5_4_compat
      ];
    };
  });
}
