#!/bin/bash

set -e

clear
echo "Discord Updater Uninstaller"
echo "==========================="

if [ "$EUID" -ne 0 ]; then
    echo "This uninstaller requires root privileges."
    echo "Please enter your password to continue..."
    exec sudo "$0" "$@"
fi

if [ -f "/usr/local/bin/discord-updater" ]; then
    echo "Removing discord-updater from /usr/local/bin..."
    rm /usr/local/bin/discord-updater
    echo "Uninstallation complete!"
else
    echo "discord-updater is not installed."
fi
