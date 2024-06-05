#!/usr/bin/env bash

echo "Building Napture..."

./scripts/git-status-check.sh

# Install deps using Homebrew
brew install gtk4 graphene glib libadwaita lua pkg-config || exit 1

# Specifies required environment variable HOMEBREW_CELLAR when it is not set
arch_name=$(uname -m)
if [ "$arch_name" = "x86_64" ]; then
  # Intel Mac
  export HOMEBREW_CELLAR="/usr/local/Cellar"
elif [ "$arch_name" = "arm64" ]; then
  # Apple Silicon Mac
  export HOMEBREW_CELLAR="/opt/homebrew/Cellar"
else
  echo "Unsupported architecture: $arch_name"
  exit 1
fi

# Build the project
RUSTFLAGS="-L $HOMEBREW_CELLAR" cargo build --release || exit 1

# Make an app bundle
mkdir -p target/release/Napture.app/Contents/MacOS
cp target/release/webx target/release/Napture.app/Contents/MacOS
cp ./Info.plist target/release/Napture.app/Contents

# Thanks https://stackoverflow.com/a/31883126/9376340

mkdir -p target/release/Napture.app/Contents/Resources/AppIcon.iconset

# Normal screen icons
for SIZE in 16 32 64 128 256 512; do
    sips -z $SIZE $SIZE ./file.png --out target/release/Napture.app/Contents/Resources/AppIcon.iconset/icon_${SIZE}x${SIZE}.png || exit 1
done

# Retina display icons
for SIZE in 32 64 256 512; do
    sips -z $SIZE $SIZE ./file.png --out target/release/Napture.app/Contents/Resources/AppIcon.iconset/icon_$(expr $SIZE / 2)x$(expr $SIZE / 2)x2.png || exit 1
done

# Make a multi-resolution Icon
iconutil -c icns -o target/release/Napture.app/Contents/Resources/AppIcon.icns target/release/Napture.app/Contents/Resources/AppIcon.iconset || exit 1
rm -rf target/release/Napture.app/Contents/Resources/AppIcon.iconset

# Sign the app bundle
codesign --force --deep --sign - target/release/Napture.app || exit 1

# Move to Application folder

echo "Installing Napture..."

rm -rf /Applications/Napture.app || true
mv target/release/Napture.app /Applications || exit 1

echo "Napture installation completed."
