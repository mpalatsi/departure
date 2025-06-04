use gtk4::prelude::*;

use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;

mod config;
mod theme;
mod ui;

use config::Config;
use theme::ThemeManager;
use ui::DepartureApp;

#[derive(Parser)]
#[command(name = "departure")]
#[command(about = "A flexible logout application for Wayland with Material You theming support")]
#[command(version = "0.1.0")]
struct Cli {
    /// Path to configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,
    
    /// Generate default configuration file
    #[arg(long)]
    generate_config: bool,
    
    /// Print current theme colors and exit
    #[arg(long)]
    print_theme: bool,
    
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
    
    /// Override theme source (manual, system, file, command)
    #[arg(long)]
    theme_source: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    if cli.debug {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }
    
    log::info!("Starting departure logout application");
    
    // Determine config path
    let config_path = cli.config.unwrap_or_else(|| {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("departure");
        path.push("config.json");
        path
    });
    
    // Generate config if requested
    if cli.generate_config {
        return generate_default_config(&config_path);
    }
    
    // Load configuration
    let mut config = Config::load(&config_path)?;
    
    // Override theme source if specified
    if let Some(source) = cli.theme_source {
        config.theme.source = source;
    }
    
    // Initialize theme manager
    let theme_manager = ThemeManager::new(config.theme.clone())?;
    
    // Print theme and exit if requested
    if cli.print_theme {
        let colors = theme_manager.get_colors()?;
        println!("Current theme colors:");
        println!("  Background: {}", colors.background);
        println!("  Primary: {}", colors.primary);
        println!("  Secondary: {}", colors.secondary);
        println!("  Text: {}", colors.text);
        println!("  Danger: {}", colors.danger);
        return Ok(());
    }
    
    // Force Wayland backend for proper layer shell support
    std::env::set_var("GDK_BACKEND", "wayland");
    
    // Initialize GTK
    gtk4::init()?;
    
    // Create and run the application
    let app = gtk4::Application::builder()
        .application_id("com.github.departure")
        .build();
    
    app.connect_activate(move |app| {
        let mut departure_app = match DepartureApp::new(app.clone(), config.clone(), theme_manager.clone()) {
            Ok(app) => app,
            Err(e) => {
                log::error!("Failed to create departure app: {}", e);
                app.quit();
                return;
            }
        };
        
        if let Err(e) = departure_app.show() {
            log::error!("Failed to show departure app: {}", e);
            app.quit();
        }
    });
    
    let exit_code = app.run();
    log::info!("Departure application exited with code: {:?}", exit_code);
    
    Ok(())
}

fn generate_default_config(path: &PathBuf) -> Result<()> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let default_config = Config::default();
    let json = serde_json::to_string_pretty(&default_config)?;
    std::fs::write(path, json)?;
    
    println!("Generated default configuration at: {}", path.display());
    println!("Edit this file to customize your departure experience.");
    
    Ok(())
}
