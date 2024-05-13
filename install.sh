#!/usr/bin/env bash

echo "Installing Napture..."

# Assuming the script is in the same directory as "napture"
cd "$(dirname "$0")/napture" || exit

# Build Napture
cargo build --release

# Copy settings file
sudo cp "src/resources/settings.gschema.xml" "/usr/share/glib-2.0/schemas/settings.gschema.xml"

# Compile schemas
sudo glib-compile-schemas .

# Define paths
exec_path="$(pwd)/target/release/webx"
icon_path="$(pwd)/file.png"

# Define the desktop entry content
content="[Desktop Entry]
Name=Napture
Exec=$exec_path
Icon=$icon_path
Type=Application
Categories=Utility;"

# Write desktop entry file using sudo tee
echo "$content" | sudo tee /usr/share/applications/napture.desktop >/dev/null

# Update desktop database
sudo update-desktop-database

echo "Napture installation completed."
