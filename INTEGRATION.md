# Departure Integration with Your Existing Setup

## Overview

Departure has been successfully integrated into your CachyOS Hyprland setup with Material You theming. It replaces wlogout with a more flexible and themeable solution that works seamlessly with your existing pywal/matugen workflow.

## What Was Built

### Core Application
- **Name**: departure
- **Location**: `/usr/local/bin/departure`
- **Config**: `~/.config/departure/config.json`
- **Language**: Rust with GTK4 + gtk4-layer-shell

### Key Features
- ✅ Circular buttons with hover effects
- ✅ Blur background support (compatible with Hyprland blur)
- ✅ Material You theming via matugen integration
- ✅ Keyboard shortcuts (l, e, s, h, r, p)
- ✅ Confirmation dialogs for destructive actions
- ✅ File watching for live theme updates
- ✅ Multiple theme sources (manual, file, command, system)

## Integration with Your Workflow

### Current Setup
Your existing setup with these components now includes departure:
- **Waybar**: Material You themed
- **Rofi**: Material You themed  
- **Hyprland**: Material You themed
- **Mako**: Material You themed
- **Departure**: Material You themed (NEW)

### Theme Script Enhancement
Created `~/.config/hypr/scripts/pywal-departure-theme.sh` which:
1. Generates Material You colors with matugen
2. Updates all applications including departure
3. Configures departure to watch matugen's color file
4. Reloads services automatically

## Usage

### Basic Commands
```bash
# Launch departure (add to your keybinds)
departure

# Generate default config
departure --generate-config

# Check current theme colors
departure --print-theme

# Debug mode
departure --debug
```

### Keybindings
Add to your `~/.config/hypr/hyprland.conf`:
```bash
# Replace your existing wlogout binding with:
bind = $mainMod SHIFT, E, exec, departure

# Optional: Add layer rules for blur
layerrule = blur, departure
layerrule = ignorezero, departure
```

### Current Configuration
Your departure is configured for:
- **Theme Source**: File-based (watches matugen output)
- **Layout**: Horizontal layout with 6 buttons
- **Actions**: Lock, Logout, Suspend, Hibernate, Reboot, Shutdown
- **Effects**: Blur, animations, hover effects enabled

## File Structure

```
~/.config/departure/
├── config.json                 # Main configuration

~/.config/hypr/scripts/
├── pywal-departure-theme.sh     # Enhanced theme script with departure
└── pywal-random-theme.sh        # Your original script (unchanged)

~/.config/matugen/
└── colors.json                 # Watched by departure for live updates
```

## Theme Integration Flow

1. **Wallpaper Change** → `pywal-departure-theme.sh`
2. **Matugen Generation** → `~/.config/matugen/colors.json`
3. **File Watching** → Departure auto-updates colors
4. **Application Updates** → Waybar, Rofi, Hyprland, Mako themed
5. **Live Preview** → All apps match new Material You palette

## Customization Examples

### Adding Custom Actions
Edit `~/.config/departure/config.json`:
```json
{
  "name": "Screenshot",
  "command": "grim ~/Pictures/screenshot-$(date +%Y%m%d-%H%M%S).png",
  "icon": "camera-photo",
  "keybind": "t",
  "confirm": false,
  "danger": false
}
```

### Changing Layout
```json
{
  "layout": {
    "layout_type": "grid",    // or "vertical"
    "button_size": 100,       // larger buttons
    "button_spacing": 25,
    "margin": 75,
    "columns": 2              // 2x3 grid
  }
}
```

### Manual Color Override
If you want to temporarily use manual colors:
```bash
departure --theme-source manual
```

## Troubleshooting

### Icons Not Showing
```bash
# Install icon theme if needed
sudo pacman -S papirus-icon-theme

# Check available icons
ls /usr/share/icons/
```

### Blur Not Working
Ensure Hyprland blur is enabled and add layer rules:
```bash
decoration {
    blur {
        enabled = true
        size = 15
        passes = 3
    }
}

layerrule = blur, departure
layerrule = ignorezero, departure
```

### Theme Not Updating
Check if matugen is working:
```bash
# Test matugen manually
matugen image ~/Pictures/Wallpapers/your-wallpaper.jpg -j json

# Check departure file watching
departure --debug
```

## Migration from wlogout

Your wlogout configuration has been preserved but departure offers:
- ✅ Better Material You integration
- ✅ File watching for live updates  
- ✅ More flexible layouts
- ✅ Better Wayland support
- ✅ Rust performance and stability
- ✅ Easier customization

## Next Steps

1. **Test the integration**:
   ```bash
   ~/.config/hypr/scripts/pywal-departure-theme.sh
   departure
   ```

2. **Update your keybinds** to use `departure` instead of `wlogout`

3. **Customize actions** in the config file as needed

4. **Share your setup** - departure is designed to work for anyone with any theming approach!

## Benefits for the Community

This implementation serves as a reference for:
- **Pywal users**: Command-based theming integration
- **Matugen users**: File-based theming with live updates  
- **Manual users**: Direct color specification
- **System theme users**: GTK theme integration

The modular design means anyone can adopt departure regardless of their theming preference, making it a truly flexible logout solution for the Wayland ecosystem. 