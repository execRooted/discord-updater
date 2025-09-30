# Discord Updater for Linux

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple and efficient tool to update Discord on Linux systems, designed to fix the "lucky day" update issue.

## Features

-  **Automatic Detection**: Finds Discord installations across various package managers and installation methods
-  **Latest Updates**: Downloads and installs the latest Discord version from official sources
-  **Safe Updates**: Creates backups before making changes
-  **Cross-Platform**: Works with Discord installed via apt, pacman, snap, flatpak, or manual installation
-  **Clean Interface**: Clear terminal output with progress indicators
-  **Fast Installation**: Optimized for quick updates
-  **Backup Restore**: Restore from backup with `--restore` flag

## Installation

### Automated Installation (Recommended)

1. Clone or download this repository
2. Run the installer:

```bash
cd discord-updater
```
```
sudo ./install.sh
```

The installer will automatically:
- Install Rust if not present
- Build the Discord updater
- Install it system-wide to `/usr/local/bin/discord-updater`

## Usage

### Update Discord
```bash
sudo discord-updater
```

### Restore from Backup
```bash
sudo discord-updater --restore
```

### Show Help
```bash
discord-updater --help
```

## Uninstallation

```bash
cd discord-updater
```
```
sudo ./uninstall.sh
```

## Requirements

- Linux operating system
- Internet connection
- Root privileges for system-wide installations

## Supported Platforms

Automatically detects Discord installed via:
- Package managers (apt, pacman, yum, dnf)
- Snap and Flatpak
- Manual installations
- Custom locations



## License

This project is licensed under the MIT License.
This is open source, as all things should be;

---

**Note**: This tool is not affiliated with Discord Inc. Use at your own risk, though it only performs standard update operations.