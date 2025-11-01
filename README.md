# ProtonUp-GTK

A graphical tool for installing and managing compatibility tools like GE-Proton for Steam and Wine-GE for Lutris. Written in Rust with GTK4 and libadwaita.

![License](https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg)

## Overview

ProtonUp-GTK is a modern, native Linux application inspired by [ProtonUp-Qt](https://github.com/DavidoTek/ProtonUp-Qt). It provides a clean, intuitive interface for managing compatibility tools that enable Windows games to run on Linux through Steam and Lutris.

### Features

- 🎮 **Install and manage GE-Proton** for Steam
- 🍷 **Install and manage Wine-GE** for Lutris  
- 🎯 **Install and manage Luxtorpeda** for Steam
- 🌟 **Install and manage Spritz-Wine** for Lutris
- 🌅 **Install and manage dwproton** for Steam
- 📦 **Automatic download and extraction** of compatibility tools
- 🎨 **Beautiful libadwaita UI** that follows GNOME HIG
- ⚡ **Fast and lightweight**, written in Rust
- 🔔 **Toast notifications** for installation status
- ⚙️ **Preferences dialog** for configuration
- 🔄 **Refresh tool list** to fetch latest versions
- 📋 **About dialog** with credits and license info

## Screenshots

*Coming soon*

## Building from Source

### Prerequisites

#### Fedora / RHEL / CentOS
```bash
sudo dnf install gtk4-devel libadwaita-devel gcc pkg-config
```

#### Ubuntu / Debian
```bash
sudo apt install libgtk-4-dev libadwaita-1-dev build-essential pkg-config
```

#### Arch Linux
```bash
sudo pacman -S gtk4 libadwaita base-devel
```

### Install Rust

If you don't have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build and Run

```bash
# Clone the repository
git clone https://github.com/Mar0xy/protonup-gtk.git
cd protonup-gtk

# Build and run
cargo run --release
```

## Installation

### From Source
```bash
cargo install --path .
```

### Flatpak (Coming Soon)

The application will be available on Flathub.

## Usage

### Installing a Compatibility Tool

1. Launch ProtonUp-GTK
2. Browse the list of available compatibility tools
3. Click the **Install** button next to the tool you want
4. The button will show "Installing..." while downloading
5. A toast notification will confirm successful installation
6. The tool is automatically installed to the appropriate directory:
   - **Steam tools** (GE-Proton, Luxtorpeda, dwproton): `~/.steam/root/compatibilitytools.d/`
   - **Lutris tools** (Wine-GE, Spritz-Wine): `~/.local/share/lutris/runners/wine/`

### Refreshing Tool List

- Click the **Refresh Tool List** button to fetch the latest versions
- Toast notification shows how many tools were found

### Accessing Preferences

1. Click the menu button (⋮) in the top-right corner
2. Select **Preferences**
3. View installation paths and configure settings

### Viewing About Information

1. Click the menu button (⋮) in the top-right corner
2. Select **About**
3. View app version, credits, and license information

See [UI_FEATURES.md](UI_FEATURES.md) for detailed UI documentation.

## Supported Compatibility Tools

- **GE-Proton**: Proton with additional fixes and features from GloriousEggroll
- **Wine-GE**: Wine with gaming-specific patches
- **Luxtorpeda**: Steam Play compatibility tool for running native Linux versions of games
- **Spritz-Wine**: Wine builds optimized for gaming performance
- **dwproton**: Dawn Wine Proton - Proton fork with improvements

## Development

### Project Structure

```
protonup-gtk/
├── src/
│   ├── main.rs              # Application entry point
│   ├── application.rs       # GTK Application setup
│   ├── window.rs            # Main window UI
│   └── backend/
│       ├── mod.rs           # Backend module
│       ├── tool_manager.rs  # Compatibility tool management
│       └── downloader.rs    # Download and extraction logic
├── Cargo.toml               # Rust dependencies
└── README.md                # This file
```

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Related Projects

- [ProtonUp-Qt](https://github.com/DavidoTek/ProtonUp-Qt) - The inspiration for this project, written in Python with Qt
- [ProtonUp](https://github.com/AUNaseef/protonup) - The original CLI tool
- [GE-Proton](https://github.com/GloriousEggroll/proton-ge-custom) - Proton with additional fixes
- [Wine-GE](https://github.com/GloriousEggroll/wine-ge-custom) - Wine with game-specific patches

## License

This project is licensed under the GNU General Public License v3.0 or later - see the LICENSE file for details.

## Disclaimer

ProtonUp-GTK is an independent tool for managing gaming compatibility tools. It is not directly affiliated with Valve, Steam, Lutris, or the compatibility tool creators.