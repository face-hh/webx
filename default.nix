{ stdenv, lib, pkgs ? import <nixpkgs> {}, ... }: pkgs.rustPlatform.buildRustPackage {
  pname = "webx";
  version = "git";

  src = "${pkgs.webx-src}/napture";

  nativeBuildInputs = lib.optionals stdenv.isLinux (with pkgs; [
    pkg-config
  ]);

  propagatedBuildInputs = lib.optionals stdenv.isLinux (with pkgs; [
    glib
    pango
    gdk-pixbuf
    openssl_3_3
    graphene
    gtk4
    libadwaita
    lua5_4_compat
  ]);

  cargoLock = {
    lockFile = "${pkgs.webx-src}/napture/Cargo.lock";
  };

  postInstall = ''
    mkdir -p $out/share/metainfo $out/share/applications $out/share/icons
    install -Dm644 $src/io.github.face_hh.Napture.metainfo.xml -t $out/share/metainfo/
    install -Dm644 $src/io.github.face_hh.Napture.desktop -t $out/share/applications/
    install -Dm644 $src/io.github.face_hh.Napture.svg -t $out/share/icons/hicolor/scalable/apps/

    # updating the `Exec` field
    substituteInPlace $out/share/applications/io.github.face_hh.Napture.desktop \
      --replace napture $out/bin/webx
  '';

  meta = {
    description = "An alternative for the World Wide Web";
    license = lib.licenses.unlicense;
    maintainers = [ lib.maintainers.alphatechnolog ]; # flake maintainer ig
    mainProgram = "webx";
  };
}
