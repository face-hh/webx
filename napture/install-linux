#!/usr/bin/env bash

echo "Building Napture..."


# Install dependencies based on linux distro

# Check for /etc/os-release file
if [ -f /etc/os-release ]; then
    # Extract and print the PRETTY_NAME value
    PRETTY_NAME=$(grep '^PRETTY_NAME=' /etc/os-release | cut -d '=' -f2- | tr -d '"')
    echo "Detected: $PRETTY_NAME"
    if [[ $PRETTY_NAME == "Ubuntu 24.04 LTS" ]]; then
        echo  "Installing dependencies..."
        sudo apt-get update
        sudo apt-get install curl git build-essential libssl-dev libglib2.0-dev libcairo2-dev libgraphene-1.0-dev libgtk-4-dev libadwaita-1-dev liblua5.4-dev
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        echo "Dependencies installed."
    else
        echo "Could not detect the Linux distribution and version. Now continuing with build, without attempting to install dependencies."
    fi
else
    echo "Could not detect the Linux distribution and version. Now continuing with build, without attempting to install dependencies."
fi


status=$(git status)

# Check if the status contains the phrase "Your branch is behind"
if [[ $status == *"Your branch is behind"* ]]; then
    echo "Your branch is behind the remote repository."
    echo "Pulling the latest changes..."
    git pull origin $(git rev-parse --abbrev-ref HEAD)
elif [[ $status == *"Your branch is up to date"* ]]; then
    echo "Your branch is up to date with the remote repository."
else
    echo "Failed to determine the repository status."
fi



# Assuming the script is in the same directory as "napture"
cd "$(dirname "$0")/napture" || exit 1

# Build Napture
cargo build --release || exit 1

echo "Installing Napture..."

# Copy files
sudo install -Dm755 ./target/release/webx /usr/bin/napture
sudo install -Dm644 ./io.github.face_hh.Napture.metainfo.xml -t /usr/share/metainfo/
sudo install -Dm644 ./io.github.face_hh.Napture.desktop -t /usr/share/applications/
sudo install -Dm644 ./io.github.face_hh.Napture.svg -t /usr/share/icons/hicolor/scalable/apps/

# Update desktop database
sudo update-desktop-database

echo "Napture installation completed."
