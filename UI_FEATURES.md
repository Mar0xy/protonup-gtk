# UI Features Documentation (v0.3.0)

## Main Window

The ProtonUp-GTK main window features:

### Header Bar
- **Application Title**: "ProtonUp-GTK"
- **Menu Button**: Access to Preferences and About dialogs (⋮ icon in top-right)

### Main Content Area

#### Welcome Section
- **Title**: "ProtonUp-GTK" (large, bold)
- **Subtitle**: "Install and manage compatibility tools for Steam and Lutris" (dim text)

#### Compatibility Tools List (NEW: Version Selection!)
A preferences group titled "Compatibility Tools" with description "Select a version to install"

**New Multi-Version Interface:**
- Each tool is now an **Expander Row** that can be clicked to expand
- Shows tool name, description, and launcher badge in collapsed state
- When expanded, displays the **last 4 available versions**
- Each version has its own **Install** button

**Tool Structure:**
```
▼ GE-Proton                                              [Steam]
  Proton compatibility tool with additional fixes
  ├─ GE-Proton9-15                                    [Install]
  ├─ GE-Proton9-14                                    [Install]
  ├─ GE-Proton9-13                                    [Install]
  └─ GE-Proton9-12                                    [Install]
```

**Expander Row Header:**
- **Tool Name** (e.g., "GE-Proton") - clickable to expand/collapse
- **Description** (e.g., "Proton compatibility tool with additional fixes")
- **Launcher Badge** - Shows "Steam" or "Lutris" (right side, dim)

**Version Sub-Rows** (visible when expanded):
- **Version Name** (e.g., "GE-Proton9-15")
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
- Fetches **last 4 versions** for each tool from GitHub/Forgejo APIs
- Shows toast notification with results
- Example: "Loaded 5 compatibility tools"

### Toast Notifications

Appear at the bottom of the window with:
- **Success messages**: Green background, 3-second timeout
  - Example: "GE-Proton GE-Proton9-15 installed successfully!"
- **Error messages**: Red background, 5-second timeout
  - Example: "Installation failed: Network error"
- **Info messages**: Blue background, 3-second timeout
  - Example: "Loaded 5 compatibility tools"

## Preferences Dialog

Accessed via Menu → Preferences

### General Page
Icon: preferences-system-symbolic

#### Installation Paths Group
Description: "Configure where compatibility tools are installed. Leave empty for defaults."

- **Steam Tools Path**
  - Entry field showing current path (editable)
  - Default: `~/.steam/root/compatibilitytools.d`
  - Users can set custom path by typing and pressing Enter
  - Clear the field and press Enter to reset to default
  - Toast notification confirms changes

- **Lutris Runners Path**
  - Entry field showing current path (editable)
  - Default: `~/.local/share/lutris/runners/wine`
  - Users can set custom path by typing and pressing Enter
  - Clear the field and press Enter to reset to default
  - Toast notification confirms changes

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
- **Version**: 0.3.0
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

### Installing a Tool (NEW: Version Selection!)

1. Click **Refresh Tool List** button to load available versions
2. Click on a **tool name** to expand and see available versions
3. Click the **Install** button next to desired version
4. Button changes to "Installing..." and becomes disabled
5. Background async task:
   - Downloads the specific version's archive to `/tmp`
   - Extracts to appropriate directory:
     - Steam tools → `~/.steam/root/compatibilitytools.d/`
     - Lutris tools → `~/.local/share/lutris/runners/wine/`
   - Cleans up temporary archive
6. Toast notification shows result:
   - Success: "Tool-Name version-number installed successfully!"
   - Error: "Installation failed: [error details]"
7. Button returns to "Install" state

**Example Flow:**
```
1. Click "Refresh Tool List"
2. Click "GE-Proton" to expand
3. See versions: GE-Proton9-15, GE-Proton9-14, etc.
4. Click "Install" next to GE-Proton9-14
5. Toast: "GE-Proton GE-Proton9-14 installed successfully!"
```

### Application Startup (NEW: Auto-Fetch)

1. Application window opens
2. **Automatically** fetches tool list in background
3. Tool list populates with expandable rows (last 4 versions each)
4. Toast notification: "Loaded X compatibility tools"
5. Tools remain visible until manual refresh
6. No need to click "Refresh" on first launch

### Refreshing Tool List (Enhanced)

1. Click **Refresh Tool List** button
2. Button becomes disabled
3. Background async task fetches **last 4 releases** for each tool
4. Tool list populates with expandable rows
5. Toast shows: "Loaded X compatibility tools"
6. Button re-enabled
7. Click any tool to expand and see versions

### Configuring Installation Paths (NEW)

1. Click menu button (⋮) in header
2. Select "Preferences"
3. Modal preferences window appears
4. **Edit Steam or Lutris paths** by typing in entry fields
5. Press Enter to apply changes
6. Toast notification confirms: "Steam path updated" or "Lutris path updated"
7. Clear field and press Enter to reset to default path
8. Close preferences window

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
