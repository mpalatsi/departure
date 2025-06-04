// Theme module

use crate::config::{ThemeConfig, ManualColors};
use anyhow::{Result, anyhow};

use std::process::Command;
use notify::{Watcher, RecursiveMode, RecommendedWatcher};
use std::sync::mpsc::channel;


#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: String,
    pub primary: String,
    pub secondary: String,
    pub text: String,
    pub danger: String,
}

#[derive(Clone)]
pub struct ThemeManager {
    config: ThemeConfig,
}

impl ThemeManager {
    pub fn new(config: ThemeConfig) -> Result<Self> {
        Ok(Self { config })
    }

    pub fn get_colors(&self) -> Result<ThemeColors> {
        match self.config.source.as_str() {
            "manual" => self.get_manual_colors(),
            "system" => self.get_system_colors(),
            "file" => self.get_file_colors(),
            "command" => self.get_command_colors(),
            _ => Err(anyhow!("Unknown theme source: {}", self.config.source)),
        }
    }

    fn get_manual_colors(&self) -> Result<ThemeColors> {
        let colors = self.config.manual_colors.as_ref()
            .ok_or_else(|| anyhow!("Manual colors not configured"))?;
        
        Ok(ThemeColors {
            background: colors.background.clone(),
            primary: colors.primary.clone(),
            secondary: colors.secondary.clone(),
            text: colors.text.clone(),
            danger: colors.danger.clone(),
        })
    }

    fn get_system_colors(&self) -> Result<ThemeColors> {
        // Try to get colors from GTK theme
        // This is a simplified implementation - in practice you'd want to
        // parse the actual GTK theme files or use GTK APIs
        log::info!("Using system theme colors (fallback to defaults)");
        
        // Fallback to default colors for now
        let default_colors = ManualColors::default();
        Ok(ThemeColors {
            background: default_colors.background,
            primary: default_colors.primary,
            secondary: default_colors.secondary,
            text: default_colors.text,
            danger: default_colors.danger,
        })
    }

    fn get_file_colors(&self) -> Result<ThemeColors> {
        let file_path = self.config.file_path.as_ref()
            .ok_or_else(|| anyhow!("File path not configured for file theme source"))?;

        if !file_path.exists() {
            return Err(anyhow!("Theme file does not exist: {}", file_path.display()));
        }

        let content = std::fs::read_to_string(file_path)?;
        
        // Try to parse as JSON first (for matugen)
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&content) {
            return self.parse_json_colors(&json_value);
        }

        // Try to parse as simple key-value pairs
        self.parse_simple_colors(&content)
    }

    fn parse_json_colors(&self, json: &serde_json::Value) -> Result<ThemeColors> {
        // Handle matugen format
        if let Some(colors) = json.get("colors") {
            if let Some(_primary) = colors.get("primary") {
                return Ok(ThemeColors {
                    background: self.extract_color(colors, &["surface", "background"], "rgba(30, 30, 46, 0.8)"),
                    primary: self.extract_color(colors, &["primary"], "#89b4fa"),
                    secondary: self.extract_color(colors, &["secondary", "tertiary"], "#74c7ec"),
                    text: self.extract_color(colors, &["on_surface", "on_background", "text"], "#cdd6f4"),
                    danger: self.extract_color(colors, &["error", "danger"], "#f38ba8"),
                });
            }
        }

        // Handle simple JSON format
        Ok(ThemeColors {
            background: self.extract_color(json, &["background"], "rgba(30, 30, 46, 0.8)"),
            primary: self.extract_color(json, &["primary"], "#89b4fa"),
            secondary: self.extract_color(json, &["secondary"], "#74c7ec"),
            text: self.extract_color(json, &["text"], "#cdd6f4"),
            danger: self.extract_color(json, &["danger"], "#f38ba8"),
        })
    }

    fn extract_color(&self, json: &serde_json::Value, keys: &[&str], default: &str) -> String {
        for key in keys {
            if let Some(value) = json.get(key) {
                if let Some(color_str) = value.as_str() {
                    return color_str.to_string();
                }
                // Handle nested objects (like matugen's hex/rgb structure)
                if let Some(hex) = value.get("hex") {
                    if let Some(hex_str) = hex.as_str() {
                        return hex_str.to_string();
                    }
                }
            }
        }
        default.to_string()
    }

    fn parse_simple_colors(&self, content: &str) -> Result<ThemeColors> {
        let mut colors = ThemeColors {
            background: "rgba(30, 30, 46, 0.8)".to_string(),
            primary: "#89b4fa".to_string(),
            secondary: "#74c7ec".to_string(),
            text: "#cdd6f4".to_string(),
            danger: "#f38ba8".to_string(),
        };

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_lowercase();
                let value = value.trim().trim_matches('"').trim_matches('\'');

                match key.as_str() {
                    "background" => colors.background = value.to_string(),
                    "primary" => colors.primary = value.to_string(),
                    "secondary" => colors.secondary = value.to_string(),
                    "text" => colors.text = value.to_string(),
                    "danger" => colors.danger = value.to_string(),
                    _ => {}
                }
            }
        }

        Ok(colors)
    }

    fn get_command_colors(&self) -> Result<ThemeColors> {
        let command = self.config.command.as_ref()
            .ok_or_else(|| anyhow!("Command not configured for command theme source"))?;

        log::debug!("Executing theme command: {}", command);
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Theme command failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        let stdout = String::from_utf8(output.stdout)?;
        
        // Try to parse as JSON first
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&stdout) {
            return self.parse_json_colors(&json_value);
        }

        // Try to parse as simple format
        self.parse_simple_colors(&stdout)
    }

    pub fn generate_css(&self, colors: &ThemeColors) -> String {
        format!(
            r#"
/* Futuristic Aurora Glass Cards Theme */
window {{
    background-color: transparent;
    font-family: sans-serif;
}}

/* Semi-transparent background for glow effects and compositor blur */
.departure-background {{
    background: {background};
}}

/* Glassmorphic card buttons with enhanced glow */
.departure-button {{
    background: rgba(255, 255, 255, 0.08);
    border: 2px solid rgba(0, 245, 255, 0.3);
    border-radius: 16px;
    color: {text};
    font-weight: 700;
    font-size: 11px;
    letter-spacing: 1px;
    text-transform: uppercase;
    box-shadow: 
        0 15px 35px rgba(0, 0, 0, 0.4),
        0 5px 15px rgba(0, 0, 0, 0.3),
        0 0 20px rgba(0, 245, 255, 0.2),
        0 0 40px rgba(0, 245, 255, 0.1),
        inset 0 1px 0 rgba(255, 255, 255, 0.2);
    transition: all 300ms ease;
    padding: 16px;
    opacity: 0.85;
}}

/* Hover effects with enhanced glow */
.departure-button:hover {{
    background: rgba(255, 255, 255, 0.15);
    border-color: rgba(0, 245, 255, 0.6);
    box-shadow: 
        0 20px 40px rgba(0, 0, 0, 0.5),
        0 8px 25px rgba(0, 245, 255, 0.3),
        0 0 30px rgba(0, 245, 255, 0.4),
        0 0 60px rgba(0, 245, 255, 0.2),
        0 0 100px rgba(0, 245, 255, 0.1),
        inset 0 1px 0 rgba(255, 255, 255, 0.3);
    opacity: 1.0;
    transform: translateY(-2px);
}}

/* Active state */
.departure-button:active {{
    transform: translateY(0px);
    opacity: 0.8;
}}

/* Danger variant */
.departure-button.danger {{
    border-color: rgba(255, 107, 107, 0.4);
}}

.departure-button.danger:hover {{
    border-color: rgba(255, 107, 107, 0.7);
    box-shadow: 
        0 20px 40px rgba(0, 0, 0, 0.5),
        0 8px 25px rgba(255, 107, 107, 0.4),
        inset 0 1px 0 rgba(255, 255, 255, 0.3);
}}

/* Button text styling */
.departure-button-text {{
    font-size: 10px;
    font-weight: 700;
    color: {text};
    text-shadow: 0 1px 3px rgba(0, 0, 0, 0.7);
    opacity: 0.9;
}}

.departure-button:hover .departure-button-text {{
    opacity: 1;
}}

/* Fallback text styling */
.departure-button-fallback {{
    font-size: 36px;
    font-weight: 900;
    color: {text};
    text-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
}}

/* Confirmation dialog */
.departure-confirmation {{
    background: rgba(0, 0, 0, 0.9);
    color: {text};
    border: 2px solid rgba(0, 245, 255, 0.4);
    border-radius: 16px;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.7);
    padding: 24px;
}}

.departure-confirmation button {{
    background: rgba(255, 255, 255, 0.1);
    color: {text};
    border: 1px solid rgba(255, 255, 255, 0.3);
    border-radius: 8px;
    padding: 12px 20px;
    margin: 8px;
    font-weight: 600;
}}

.departure-confirmation button:hover {{
    background: rgba(255, 255, 255, 0.2);
    border-color: rgba(0, 245, 255, 0.5);
}}

.departure-confirmation button.danger {{
    border-color: rgba(255, 107, 107, 0.5);
}}

.departure-confirmation button.danger:hover {{
    border-color: rgba(255, 107, 107, 0.8);
}}

/* Simple animations */
@keyframes slideIn {{
    from {{ 
        opacity: 0; 
        transform: translateY(20px);
    }}
    to {{ 
        opacity: 1; 
        transform: translateY(0);
    }}
}}

/* Apply animations */
.departure-button {{
    animation: slideIn 400ms ease-out;
}}

/* Staggered animation delays */
.departure-button:nth-child(1) {{ animation-delay: 0ms; }}
.departure-button:nth-child(2) {{ animation-delay: 80ms; }}
.departure-button:nth-child(3) {{ animation-delay: 160ms; }}
.departure-button:nth-child(4) {{ animation-delay: 240ms; }}
.departure-button:nth-child(5) {{ animation-delay: 320ms; }}
"#,
            text = colors.text,
            background = colors.background,
        )
    }

    pub fn start_file_watcher(&self) -> Result<()> {
        if !self.config.watch_file {
            return Ok(());
        }

        let file_path = self.config.file_path.as_ref()
            .ok_or_else(|| anyhow!("File path not configured for file watching"))?;

        let (tx, _rx) = channel();
        let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;
        watcher.watch(file_path, RecursiveMode::NonRecursive)?;

        // In a real implementation, you'd want to handle this in a separate thread
        // and notify the UI when the theme changes
        log::info!("Started watching theme file: {}", file_path.display());
        
        Ok(())
    }
}
