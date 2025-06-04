use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: ThemeConfig,
    pub layout: LayoutConfig,
    pub effects: EffectsConfig,
    pub actions: Vec<ActionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub source: String, // "manual", "system", "file", "command"
    pub manual_colors: Option<ManualColors>,
    pub file_path: Option<PathBuf>,
    pub command: Option<String>,
    pub watch_file: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualColors {
    pub background: String,
    pub primary: String,
    pub secondary: String,
    pub text: String,
    pub danger: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub layout_type: String, // "horizontal", "vertical", "grid"
    pub button_size: u32,
    pub button_spacing: u32,
    pub margin: u32,
    pub columns: Option<u32>, // for grid layout
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectsConfig {
    pub blur: bool,
    pub animations: bool,
    pub hover_effects: bool,
    pub transition_duration: u32, // milliseconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfig {
    pub name: String,
    pub command: String,
    pub icon: String,
    pub keybind: Option<String>,
    pub confirm: bool,
    pub danger: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            layout: LayoutConfig::default(),
            effects: EffectsConfig::default(),
            actions: vec![
                ActionConfig {
                    name: "Lock".to_string(),
                    command: "hyprlock".to_string(),
                    icon: "system-lock-screen".to_string(),
                    keybind: Some("l".to_string()),
                    confirm: false,
                    danger: false,
                },
                ActionConfig {
                    name: "Logout".to_string(),
                    command: "hyprctl dispatch exit".to_string(),
                    icon: "system-log-out".to_string(),
                    keybind: Some("e".to_string()),
                    confirm: true,
                    danger: false,
                },
                ActionConfig {
                    name: "Suspend".to_string(),
                    command: "systemctl suspend".to_string(),
                    icon: "system-suspend".to_string(),
                    keybind: Some("s".to_string()),
                    confirm: false,
                    danger: false,
                },
                ActionConfig {
                    name: "Hibernate".to_string(),
                    command: "systemctl hibernate".to_string(),
                    icon: "system-suspend-hibernate".to_string(),
                    keybind: Some("h".to_string()),
                    confirm: false,
                    danger: false,
                },
                ActionConfig {
                    name: "Reboot".to_string(),
                    command: "systemctl reboot".to_string(),
                    icon: "system-reboot".to_string(),
                    keybind: Some("r".to_string()),
                    confirm: true,
                    danger: true,
                },
                ActionConfig {
                    name: "Shutdown".to_string(),
                    command: "systemctl poweroff".to_string(),
                    icon: "system-shutdown".to_string(),
                    keybind: Some("p".to_string()),
                    confirm: true,
                    danger: true,
                },
            ],
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            source: "manual".to_string(),
            manual_colors: Some(ManualColors::default()),
            file_path: None,
            command: None,
            watch_file: false,
        }
    }
}

impl Default for ManualColors {
    fn default() -> Self {
        Self {
            background: "rgba(30, 30, 46, 0.8)".to_string(),
            primary: "#89b4fa".to_string(),
            secondary: "#74c7ec".to_string(),
            text: "#cdd6f4".to_string(),
            danger: "#f38ba8".to_string(),
        }
    }
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            layout_type: "horizontal".to_string(),
            button_size: 80,
            button_spacing: 20,
            margin: 50,
            columns: Some(3),
        }
    }
}

impl Default for EffectsConfig {
    fn default() -> Self {
        Self {
            blur: true,
            animations: true,
            hover_effects: true,
            transition_duration: 200,
        }
    }
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            log::info!("Config file not found at {}, using defaults", path.display());
            Ok(Config::default())
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
