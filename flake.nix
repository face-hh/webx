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

    # same as the one in the overlay but with the current pkgs iteration (current system).
    package = pkgs.callPackage ./default.nix {};

    overlays = [mainOverlay];

    mainOverlay = (final: _: {
      inherit (inputs) webx-src;
      webx = final.callPackage ./default.nix {};
    });
  in {
    packages.default = package;
    overlays.default = mainOverlay;

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
