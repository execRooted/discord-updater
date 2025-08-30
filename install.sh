#!/bin/bash

set -e

clear
echo "Discord Updater Installer"
echo "========================="

if [ "$EUID" -ne 0 ]; then
    echo "This installer requires root privileges."
    echo "Please enter your password to continue..."
    exec sudo "$0" "$@"
fi

if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

echo "Building discord-updater..."
cargo build --release

echo "Installing discord-updater to /usr/local/bin..."
cp target/release/discord-update /usr/local/bin/discord-updater

chmod +x /usr/local/bin/discord-updater

echo "Installation complete!"
echo "You can now run 'discord-updater' from anywhere."
echo "To update Discord, simply type: discord-updater"