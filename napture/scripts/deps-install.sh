#!/usr/bin/env bash

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