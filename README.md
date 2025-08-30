# Discord Updater for Linux

A simple and efficient tool to update Discord on Linux systems, designed to fix the "lucky day" update issue and provide manual control over Discord updates.

## Features

- 🚀 **Automatic Detection**: Finds Discord installations across various package managers and installation methods
- 🔄 **Latest Updates**: Downloads and installs the latest Discord version from official sources
- 🛡️ **Safe Updates**: Creates backups before making changes
- 🔧 **Cross-Platform**: Works with Discord installed via apt, pacman, snap, flatpak, or manual installation
- 🖥️ **Clean Interface**: Clear terminal output with progress indicators
- ⚡ **Fast Installation**: Optimized for quick updates

## Requirements

- Linux operating system
- Rust (automatically installed by installer if missing)
- Internet connection
- Root privileges for system-wide installations

## Installation

### Option 1: Automated Installation (Recommended)

```bash
cd discord-update
./install.sh
```

The installer will:
- Check for and install Rust if needed
- Build the Discord updater
- Install it system-wide to `/usr/local/bin/discord-updater`
- Set up proper permissions

### Option 2: Manual Installation

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Build and install
cd discord-update
cargo build --release
sudo cp target/release/discord-update /usr/local/bin/discord-updater
sudo chmod +x /usr/local/bin/discord-updater
```

## Usage

### Update Discord

```bash
discord-updater
```

For system-wide installations requiring root:

```bash
sudo discord-updater
```

### What happens during update:

1. **Detection**: Automatically finds your Discord installation
2. **Download**: Downloads the latest Discord version
3. **Backup**: Creates a backup of your current installation
4. **Installation**: Replaces the old version with the new one
5. **Completion**: Prompts you to restart Discord

## Supported Installation Methods

The updater automatically detects Discord installed via:

- **Package Managers**: apt, pacman, yum, dnf
- **Snap**: `snap install discord`
- **Flatpak**: `flatpak install flathub com.discordapp.Discord`
- **Manual Installation**: Downloaded and installed manually
- **Custom Locations**: Any standard Linux installation path

## Troubleshooting

### "Discord installation not found"

If you get this error, Discord might be installed in a non-standard location. The updater searches these paths:

- `/opt/discord`
- `/usr/share/discord`
- `/usr/local/share/discord`
- `/usr/bin/discord`
- `/usr/local/bin/discord`
- `/snap/discord`
- `~/.local/share/discord`
- `~/.discord`

**Solution**: Ensure Discord is properly installed and try running the updater with `sudo`.

### "Permission denied" or "Operation not permitted"

**Solution**: Run with `sudo`:
```bash
sudo discord-updater
```

### "Cross-device link" error

This is automatically handled by the updater - no action needed.

### Discord doesn't start after update

**Solution**: Manually restart Discord or reboot your system.

### Build fails

**Solution**: Ensure you have the required build tools:
```bash
# Ubuntu/Debian
sudo apt install build-essential

# Arch Linux
sudo pacman -S base-devel

# Fedora
sudo dnf groupinstall "Development Tools"
```

## Uninstallation

To remove the Discord updater:

```bash
cd discord-update
./uninstall.sh
```

This will:
- Remove the `discord-updater` binary
- Clean up any associated files
- Remove the alias if it was created

## How It Works

1. **Smart Detection**: Uses multiple methods to locate Discord installations
2. **Official Downloads**: Downloads directly from Discord's official API
3. **Safe Replacement**: Backs up existing installation before replacement
4. **Cross-Filesystem**: Handles installations across different mount points
5. **Clean Process**: No leftover files or temporary data

## Security

- Downloads from official Discord sources only
- No data collection or telemetry
- Open source code for transparency
- Requires explicit user permission for system changes

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is open source and available under the MIT License.

## Support

If you encounter issues:

1. Check the troubleshooting section above
2. Ensure Discord is properly installed
3. Try running with `sudo`
4. Check system logs for additional error information

## Changelog

### Version 1.0.0
- Initial release
- Automatic Discord detection
- Safe update process with backups
- Cross-platform Linux support
- Clean terminal interface

---

**Note**: This tool is not affiliated with Discord Inc. Use at your own risk, though it only performs standard update operations.