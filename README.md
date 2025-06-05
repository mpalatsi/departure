![image](https://github.com/user-attachments/assets/16c4bcd9-3dcd-4b2f-9c67-da1fa5895eaf)


# Departure

A flexible logout application for Wayland with Material You theming support. Departure provides a modern, customizable logout interface with buttons, blur effects, and support for multiple theming systems including pywal, matugen, and manual colors.

![Departure Demo](departure-demo.gif)

## Features

- **Flexible Theming**: Support for manual colors, system themes, file-based themes (matugen), and command-based themes (pywal)
- **Modern UI**: Circular buttons with hover effects and smooth animations
- **Wayland Integration**: Uses gtk4-layer-shell for proper overlay behavior
- **Keyboard Shortcuts**: Configurable keybindings for quick actions
- **Confirmation Dialogs**: Optional confirmation for destructive actions
- **Live Theme Updates**: File watching for automatic theme updates (when using matugen)
- **Customizable Actions**: Configure your own commands and icons

## Installation

### Prerequisites

- Rust (latest stable)
- GTK4 development libraries
- gtk4-layer-shell

On Arch Linux:
```bash
sudo pacman -S rust gtk4 gtk4-layer-shell
```

### Building from Source

```bash
git clone https://github.com/mpalatsi/departure.git
cd departure
cargo build --release
sudo cp target/release/departure /usr/local/bin/
```

## Quick Start

1. Generate a default configuration:
```bash
departure --generate-config
```

2. Test the application:
```bash
departure
```

3. Add a keybinding to your Hyprland config:
```bash
bind = $mainMod SHIFT, E, exec, departure
```

## Configuration

The configuration file is located at `~/.config/departure/config.json`. It consists of four main sections:

### Theme Configuration

#### Manual Colors
```json
{
  "theme": {
    "source": "manual",
    "manual_colors": {
      "background": "rgba(30, 30, 46, 0.8)",
      "primary": "#89b4fa",
      "secondary": "#74c7ec",
      "text": "#cdd6f4",
      "danger": "#f38ba8"
    }
  }
}
```

#### File-based Themes (Matugen)
```json
{
  "theme": {
    "source": "file",
    "file_path": "/home/user/.config/matugen/colors.json",
    "watch_file": true
  }
}
```

#### Command-based Themes (Pywal)
```json
{
  "theme": {
    "source": "command",
    "command": "cat ~/.cache/wal/colors.json"
  }
}
```

### Layout Configuration

```json
{
  "layout": {
    "layout_type": "horizontal",
    "button_size": 80,
    "button_spacing": 20,
    "margin": 50,
    "columns": 3
  }
}
```

Available layout types:
- `horizontal`: Single row of buttons
- `vertical`: Single column of buttons  
- `grid`: Grid layout with configurable columns

### Effects Configuration

```json
{
  "effects": {
    "blur": true,
    "animations": true,
    "hover_effects": true,
    "transition_duration": 200
  }
}
```

### Actions Configuration

```json
{
  "actions": [
    {
      "name": "Lock",
      "command": "hyprlock",
      "icon": "system-lock-screen",
      "keybind": "l",
      "confirm": false,
      "danger": false
    }
  ]
}
```

## CLI Usage

```bash
# Generate default configuration
departure --generate-config

# Print current theme colors
departure --print-theme

# Use custom configuration file
departure --config /path/to/config.json

# Override theme source
departure --theme-source manual

# Enable debug logging
departure --debug
```

## Integration Examples

### Hyprland

Add to your `~/.config/hypr/hyprland.conf`:

```bash
# Keybinding
bind = $mainMod SHIFT, E, exec, departure

# Layer rules for blur effects (optional)
layerrule = blur, departure
layerrule = ignorezero, departure
```

### Pywal Integration

Update your pywal theme script to include departure:

```bash
#!/bin/bash
# Generate colors with pywal
wal -i /path/to/wallpaper

# Update departure config to use pywal
departure --theme-source command
```

Or configure departure to use pywal directly:

```json
{
  "theme": {
    "source": "command",
    "command": "cat ~/.cache/wal/colors.json"
  }
}
```

### Matugen Integration

Configure departure to watch matugen's output:

```json
{
  "theme": {
    "source": "file",
    "file_path": "/home/user/.config/matugen/colors.json",
    "watch_file": true
  }
}
```

Add to your matugen theme script:

```bash
#!/bin/bash
# Generate Material You theme
matugen image /path/to/wallpaper

# Departure will automatically detect the theme change
# due to file watching
```

## Custom Actions

You can add custom actions to the configuration:

```json
{
  "name": "Screenshot",
  "command": "grim ~/screenshot.png",
  "icon": "camera-photo",
  "keybind": "s",
  "confirm": false,
  "danger": false
}
```

## Troubleshooting

### Icons Not Showing

If system icons aren't displayed, install an icon theme:

```bash
sudo pacman -S papirus-icon-theme
```

### Blur Effects Not Working

Ensure your Hyprland configuration has blur enabled:

```bash
decoration {
    blur {
        enabled = true
        size = 15
        passes = 3
        vibrancy = 0.25
    }
}
```

### Application Not Appearing

Check that gtk4-layer-shell is properly installed and your compositor supports layer shell.

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Credits

- Built with GTK4 and Rust
- Uses gtk4-layer-shell for Wayland integration
- Inspired by wlogout but designed for greater flexibility
- Material You theming support via matugen integration 
