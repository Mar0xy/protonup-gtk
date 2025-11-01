# ProtonUp-GTK

A graphical tool for installing and managing compatibility tools like GE-Proton for Steam and Wine-GE for Lutris. Written in Rust with GTK4 and libadwaita.

![License](https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg)

## Overview

ProtonUp-GTK is a modern, native Linux application inspired by [ProtonUp-Qt](https://github.com/DavidoTek/ProtonUp-Qt). It provides a clean, intuitive interface for managing compatibility tools that enable Windows games to run on Linux through Steam and Lutris.

### Features

- 🎮 **Install and manage GE-Proton** for Steam
- 🍷 **Install and manage Wine-GE** for Lutris  
- 🌟 **Install and manage Spritz-Wine** for Lutris
- 🌅 **Install and manage dwproton** for Steam
- 📦 **Automatic download and extraction** of compatibility tools
- 🎨 **Beautiful libadwaita UI** that follows GNOME HIG
- ⚡ **Fast and lightweight**, written in Rust
- 🔔 **Toast notifications** for installation status
- ⚙️ **Preferences dialog** for configuration
- 🔄 **Auto-fetch on startup** - Tools load automatically when app launches
- 🔄 **Refresh tool list** to fetch latest versions
- 📋 **About dialog** with credits and license info
- 🎯 **Version selection** - Choose from last 4 releases of each tool
- 📂 **Expandable tool rows** - Click to see all available versions
- 🛠️ **Configurable paths** - Set custom Steam and Lutris installation directories

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

### First Launch (NEW: Auto-Fetch!)

1. Launch ProtonUp-GTK
2. **Tool list is automatically fetched** on startup
3. Toast notification: "Loaded X compatibility tools"
4. All tools are ready to browse and install immediately

### Installing a Compatibility Tool (NEW: Version Selection!)

1. Launch ProtonUp-GTK (tools auto-load on startup)
2. **Click on a tool name** to expand and see all available versions (last 4 releases)
3. Click the **Install** button next to the version you want
4. The button will show "Installing..." while downloading
5. A toast notification will confirm successful installation with version number
6. The tool is automatically installed to the appropriate directory:
   - **Steam tools** (GE-Proton, dwproton): `~/.steam/root/compatibilitytools.d/`
   - **Lutris tools** (Wine-GE, Spritz-Wine): `~/.local/share/lutris/runners/wine/`

**Example:**
```
1. Launch app (tools auto-load)
2. Click "GE-Proton" to expand
3. See versions: GE-Proton10-24, GE-Proton10-23, GE-Proton10-22, GE-Proton10-21
4. Click "Install" next to your preferred version
5. Toast: "GE-Proton GE-Proton10-24 installed successfully!"
```

### Refreshing Tool List

- Click the **Refresh Tool List** button to fetch the **latest 4 versions** of each tool
- Toast notification shows how many tools were loaded
- Each tool becomes an expandable row showing multiple versions
- **Note**: Tool list is automatically loaded on startup, refresh only needed to get latest versions

### Configuring Installation Paths (NEW!)

1. Click the menu button (⋮) in the top-right corner
2. Select **Preferences**
3. Click the **folder button (📁)** next to the path you want to change
4. Use the directory picker to select your desired installation directory
5. Toast notification confirms the change
6. Click the **clear button (✕)** to reset a path to default

Default paths:
- Steam: `~/.steam/root/compatibilitytools.d/`
- Lutris: `~/.local/share/lutris/runners/wine/`

### Accessing Preferences

1. Click the menu button (⋮) in the top-right corner
2. Select **Preferences**
3. View and edit installation paths
4. Configure other settings

### Viewing About Information

1. Click the menu button (⋮) in the top-right corner
2. Select **About**
3. View app version, credits, and license information

See [UI_FEATURES.md](UI_FEATURES.md) for detailed UI documentation.

## Supported Compatibility Tools

- **GE-Proton**: Proton with additional fixes and features from GloriousEggroll
- **Wine-GE**: Wine with gaming-specific patches
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