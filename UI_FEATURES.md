# UI Features Documentation

## Main Window

The ProtonUp-GTK main window features:

### Header Bar
- **Application Title**: "ProtonUp-GTK"
- **Menu Button**: Access to Preferences and About dialogs (⋮ icon in top-right)

### Main Content Area

#### Welcome Section
- **Title**: "ProtonUp-GTK" (large, bold)
- **Subtitle**: "Install and manage compatibility tools for Steam and Lutris" (dim text)

#### Compatibility Tools List
A preferences group titled "Compatibility Tools" with description "Available compatibility tools for installation"

Each tool row contains:
- **Tool Name** (e.g., "GE-Proton")
- **Description** (e.g., "Proton compatibility tool for Steam")
- **Launcher Badge** - Shows "Steam" or "Lutris" (right side, dim)
- **Install Button** - Green "suggested-action" button
  - States:
    - Normal: "Install" (clickable)
    - Active: "Installing..." (disabled during download)
    - After completion: Returns to "Install"

##### Supported Tools:
1. **GE-Proton** (Steam) - Proton compatibility tool with additional fixes
2. **Wine-GE** (Lutris) - Wine with additional game fixes
3. **Luxtorpeda** (Steam) - Steam Play compatibility tool for native Linux games
4. **Spritz-Wine** (Lutris) - Wine builds optimized for gaming performance
5. **dwproton** (Steam) - Dawn Wine Proton - Proton fork with improvements

#### Refresh Button
- Centered button labeled "Refresh Tool List"
- Fetches latest versions from GitHub/Forgejo APIs
- Shows toast notification with results

### Toast Notifications

Appear at the bottom of the window with:
- **Success messages**: Green background, 3-second timeout
  - Example: "GE-Proton 8.25-1 installed successfully!"
- **Error messages**: Red background, 5-second timeout
  - Example: "Installation failed: Network error"
- **Info messages**: Blue background, 3-second timeout
  - Example: "Found 5 compatibility tools"

## Preferences Dialog

Accessed via Menu → Preferences

### General Page
Icon: preferences-system-symbolic

#### Installation Paths Group
Description: "Configure where compatibility tools are installed"

- **Steam Tools Path**
  - Subtitle: `~/.steam/root/compatibilitytools.d`
  - (Currently display-only)

- **Lutris Runners Path**
  - Subtitle: `~/.local/share/lutris/runners/wine`
  - (Currently display-only)

#### Updates Group
Description: "Automatic update settings"

- **Check for Updates**
  - Subtitle: "Automatically check for new tool versions"
  - Toggle switch (currently non-functional, UI placeholder)

## About Dialog

Accessed via Menu → About

Shows:
- **Application Name**: ProtonUp-GTK
- **Icon**: com.github.Mar0xy.ProtonUpGtk
- **Version**: 0.1.0
- **Description**: Install and manage compatibility tools for Steam and Lutris
- **Developer**: Mar0xy
- **License**: GPL-3.0-or-later
- **Website**: https://github.com/Mar0xy/protonup-gtk
- **Issue Tracker**: https://github.com/Mar0xy/protonup-gtk/issues

### Credits Section: "Compatibility Tools"
- GE-Proton by GloriousEggroll
- Wine-GE by GloriousEggroll
- Luxtorpeda by luxtorpeda-dev
- Spritz-Wine by NelloKudo
- dwproton by Dawn Wine

## User Interactions

### Installing a Tool

1. Click the **Install** button next to any tool
2. Button changes to "Installing..." and becomes disabled
3. Background async task:
   - Fetches tool metadata from GitHub/Forgejo API
   - Downloads archive to `/tmp`
   - Extracts to appropriate directory:
     - Steam tools → `~/.steam/root/compatibilitytools.d/`
     - Lutris tools → `~/.local/share/lutris/runners/wine/`
   - Cleans up temporary archive
4. Toast notification shows result:
   - Success: "Tool-Name version installed successfully!"
   - Error: "Installation failed: [error details]"
5. Button returns to "Install" state

### Refreshing Tool List

1. Click **Refresh Tool List** button
2. Button becomes disabled
3. Background async task fetches latest releases
4. Toast shows: "Found X compatibility tools"
5. Button re-enabled

### Opening Preferences

1. Click menu button (⋮) in header
2. Select "Preferences"
3. Modal preferences window appears
4. View/configure installation paths and settings

### Viewing About Info

1. Click menu button (⋮) in header
2. Select "About"
3. About window shows app info and credits

## Technical Implementation

- **Framework**: GTK4 with libadwaita
- **Async Runtime**: glib::MainContext for GTK-compatible async operations
- **Notifications**: adw::ToastOverlay with adw::Toast
- **Dialogs**: adw::PreferencesWindow, adw::AboutWindow
- **HTTP Client**: reqwest (async)
- **Archive Extraction**: tar + flate2 + xz2

## Design Principles

- **GNOME HIG Compliance**: Follows GNOME Human Interface Guidelines
- **libadwaita Widgets**: Uses modern Adwaita widgets (ActionRow, PreferencesGroup, etc.)
- **Responsive**: Works at different window sizes
- **Non-blocking**: All I/O operations are async
- **User Feedback**: Immediate visual feedback for all actions
