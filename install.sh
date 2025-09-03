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

detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        echo "$ID"
    elif [ -f /etc/debian_version ]; then
        echo "debian"
    elif [ -f /etc/redhat-release ]; then
        echo "rhel"
    else
        echo "unknown"
    fi
}

install_build_deps() {
    local distro=$(detect_distro)
    echo "Detected distribution: $distro"
    case "$distro" in
        ubuntu|debian|linuxmint|pop)
            echo "Installing build dependencies for Debian/Ubuntu..."
            apt update
            apt install -y build-essential pkg-config libssl-dev
            ;;
        arch|manjaro|endeavouros)
            echo "Installing build dependencies for Arch Linux..."
            pacman -Syu --noconfirm base-devel pkg-config openssl
            ;;
        fedora)
            echo "Installing build dependencies for Fedora..."
            dnf groupinstall -y "Development Tools"
            dnf install -y pkg-config openssl-devel
            ;;
        centos|rhel|almalinux|rocky)
            echo "Installing build dependencies for CentOS/RHEL..."
            yum groupinstall -y "Development Tools"
            yum install -y pkgconfig openssl-devel
            ;;
        opensuse|sles)
            echo "Installing build dependencies for openSUSE..."
            zypper install -y -t pattern devel_basis
            zypper install -y pkg-config libopenssl-devel
            ;;
        *)
            echo "Unknown distribution. Please install build tools manually (build-essential or equivalent, pkg-config, libssl-dev)."
            echo "Continuing with installation..."
            ;;
    esac
}

install_build_deps

install_rust() {
    local distro=$(detect_distro)
    echo "Installing Rust..."
    case "$distro" in
        ubuntu|debian|linuxmint|pop)
            if apt install -y rustc cargo; then
                echo "Rust installed via apt."
                return 0
            fi
            ;;
        arch|manjaro|endeavouros)
            if pacman -S --noconfirm rust; then
                echo "Rust installed via pacman."
                return 0
            fi
            ;;
        fedora)
            if dnf install -y rust cargo; then
                echo "Rust installed via dnf."
                return 0
            fi
            ;;
        centos|rhel|almalinux|rocky)
            if yum install -y rust cargo; then
                echo "Rust installed via yum."
                return 0
            fi
            ;;
        opensuse|sles)
            if zypper install -y rust cargo; then
                echo "Rust installed via zypper."
                return 0
            fi
            ;;
    esac
    echo "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    export PATH="$HOME/.cargo/bin:$PATH"
}

if ! command -v cargo &> /dev/null; then
    install_rust
else
    echo "Rust is already installed."
fi

echo "Building discord-updater..."
cargo build --release

echo "Installing discord-updater to /usr/local/bin..."
cp target/release/discord-update /usr/local/bin/discord-updater

chmod +x /usr/local/bin/discord-updater

echo "Installation complete!"
echo "You can now run 'discord-updater' from anywhere."
echo "To update Discord, simply type: discord-updater"