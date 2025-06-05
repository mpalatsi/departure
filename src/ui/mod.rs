// UI module

use crate::config::{Config, ActionConfig};
use crate::theme::{ThemeManager, ThemeColors};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Dialog, Label, Orientation};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use anyhow::Result;
use std::process::Command;


pub struct DepartureApp {
    app: Application,
    config: Config,
    theme_manager: ThemeManager,
    window: Option<ApplicationWindow>,
}

impl DepartureApp {
    pub fn new(app: Application, config: Config, theme_manager: ThemeManager) -> Result<Self> {
        Ok(Self {
            app,
            config,
            theme_manager,
            window: None,
        })
    }

    pub fn show(&mut self) -> Result<()> {
        let window = ApplicationWindow::builder()
            .application(&self.app)
            .title("Departure")
            .default_width(400)
            .default_height(200)
            .build();

        // Initialize layer shell
        if gtk4_layer_shell::is_supported() {
            window.init_layer_shell();
            window.set_layer(Layer::Overlay);
            window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);
            
            // Explicitly disable exclusive zone to cover waybar
            window.auto_exclusive_zone_enable();
            window.set_exclusive_zone(-1);
            
            // Set namespace for layer rules
            window.set_namespace("departure");
            
            log::info!("Layer shell initialized successfully");
        } else {
            log::warn!("Layer shell not supported, falling back to regular window");
        }
        
        // Set anchors to cover full screen for blur effect
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Bottom, true);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);

        // Set margins to 0 for full screen coverage
        window.set_margin(Edge::Top, 0);
        window.set_margin(Edge::Bottom, 0);
        window.set_margin(Edge::Left, 0);
        window.set_margin(Edge::Right, 0);

        // Get theme colors and apply CSS
        let colors = self.theme_manager.get_colors()?;
        self.apply_theme(&window, &colors)?;

        // Create overlay container for dimming effect
        let overlay = gtk4::Overlay::new();
        
        // Create background for dimming (semi-transparent)
        let background = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        background.add_css_class("departure-background");
        background.set_hexpand(true);
        background.set_vexpand(true);
        
        // Create main container
        let main_box = self.create_main_layout(&colors)?;
        
        overlay.set_child(Some(&background));
        overlay.add_overlay(&main_box);
        window.set_child(Some(&overlay));

        // Set up keyboard shortcuts
        self.setup_keyboard_shortcuts(&window)?;

        // Connect window close event for debugging
        window.connect_close_request(|window| {
            log::info!("Window close requested");
            gtk4::glib::Propagation::Proceed
        });

        // Connect destroy signal for debugging
        window.connect_destroy(|_| {
            log::info!("Window destroyed");
        });

        // Show window
        window.present();
        self.window = Some(window);

        log::info!("Departure window created and presented successfully");

        Ok(())
    }

    fn create_main_layout(&self, colors: &ThemeColors) -> Result<Box> {
        let orientation = match self.config.layout.layout_type.as_str() {
            "vertical" => Orientation::Vertical,
            "horizontal" => Orientation::Horizontal,
            "grid" => Orientation::Horizontal, // We'll handle grid separately
            _ => Orientation::Horizontal,
        };

        let main_box = Box::new(orientation, self.config.layout.button_spacing as i32);
        main_box.set_halign(gtk4::Align::Center);
        main_box.set_valign(gtk4::Align::Center);
        main_box.set_hexpand(false);
        main_box.set_vexpand(false);

        if self.config.layout.layout_type == "grid" {
            self.create_grid_layout(&main_box, colors)?;
        } else {
            self.create_linear_layout(&main_box, colors)?;
        }

        Ok(main_box)
    }

    fn create_linear_layout(&self, container: &Box, colors: &ThemeColors) -> Result<()> {
        for action in &self.config.actions {
            let button = self.create_action_button(action, colors)?;
            container.append(&button);
        }
        Ok(())
    }

    fn create_grid_layout(&self, container: &Box, colors: &ThemeColors) -> Result<()> {
        let columns = self.config.layout.columns.unwrap_or(3);
        let mut current_row: Option<Box> = None;
        let mut current_column = 0;

        for action in &self.config.actions {
            if current_column == 0 {
                current_row = Some(Box::new(Orientation::Horizontal, self.config.layout.button_spacing as i32));
                current_row.as_ref().unwrap().set_halign(gtk4::Align::Center);
                container.append(current_row.as_ref().unwrap());
            }

            let button = self.create_action_button(action, colors)?;
            current_row.as_ref().unwrap().append(&button);

            current_column = (current_column + 1) % columns;
        }

        Ok(())
    }

    fn create_action_button(&self, action: &ActionConfig, _colors: &ThemeColors) -> Result<Button> {
        let button = Button::new();
        
        // Set button size
        button.set_size_request(
            self.config.layout.button_size as i32,
            self.config.layout.button_size as i32,
        );

        // Add CSS classes
        button.add_css_class("departure-button");
        if action.danger {
            button.add_css_class("danger");
        }

        // Try to load icon - handle both file paths and system icon names
        let button_content = if action.icon.starts_with('/') || action.icon.contains('.') {
            // File path - try to load custom PNG/SVG file
            if std::path::Path::new(&action.icon).exists() {
                log::info!("Loading custom icon from file: {}", action.icon);
                let image = gtk4::Image::from_file(&action.icon);
                image.set_pixel_size(48); // Set a good size for custom icons
                Some(image.upcast::<gtk4::Widget>())
            } else {
                log::warn!("Custom icon file not found: {}", action.icon);
                None
            }
        } else {
            // System icon name
            log::info!("Loading system icon: {}", action.icon);
            let image = gtk4::Image::from_icon_name(&action.icon);
            image.set_pixel_size(48); // Match custom icon size
            Some(image.upcast::<gtk4::Widget>())
        };

        // Create card-style layout with icon and text
        let card_container = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        card_container.set_halign(gtk4::Align::Center);
        card_container.set_valign(gtk4::Align::Center);

        // Add icon to card
        if let Some(content) = button_content {
            card_container.append(&content);
        } else {
            // Fallback to first letter of action name
            let fallback_label = gtk4::Label::new(Some(&action.name.chars().next().unwrap_or('?').to_string()));
            fallback_label.add_css_class("departure-button-fallback");
            card_container.append(&fallback_label);
            log::info!("Using fallback text for action: {}", action.name);
        }

        // Add text label to card
        let text_label = gtk4::Label::new(Some(&action.name));
        text_label.add_css_class("departure-button-text");
        card_container.append(&text_label);

        button.set_child(Some(&card_container));

        // Set tooltip
        button.set_tooltip_text(Some(&format!("{} ({})", action.name, action.keybind.as_deref().unwrap_or("no key"))));

        // Connect click handler
        let action_clone = action.clone();
        let config_clone = self.config.clone();
        let app_clone = self.app.clone();
        
        button.connect_clicked(move |button| {
            let window = button.root().and_then(|root| root.downcast::<ApplicationWindow>().ok());
            
            if action_clone.confirm {
                if let Some(window) = window {
                    Self::show_confirmation_dialog(&window, &action_clone, &config_clone, &app_clone);
                }
            } else {
                Self::execute_action(&action_clone, &app_clone);
            }
        });

        Ok(button)
    }

    fn show_confirmation_dialog(parent: &ApplicationWindow, action: &ActionConfig, _config: &Config, app: &Application) {
        let dialog = Dialog::builder()
            .title(&format!("Confirm {}", action.name))
            .modal(true)
            .transient_for(parent)
            .build();

        dialog.add_css_class("departure-confirmation");

        let content_area = dialog.content_area();
        let message = Label::new(Some(&format!("Are you sure you want to {}?", action.name.to_lowercase())));
        message.set_margin_top(20);
        message.set_margin_bottom(20);
        message.set_margin_start(20);
        message.set_margin_end(20);
        content_area.append(&message);

        // Add buttons
        let button_box = Box::new(Orientation::Horizontal, 10);
        button_box.set_halign(gtk4::Align::Center);
        button_box.set_margin_bottom(20);

        let cancel_button = Button::with_label("Cancel");
        let confirm_button = Button::with_label(&action.name);
        
        if action.danger {
            confirm_button.add_css_class("danger");
        }

        // Connect handlers
        let dialog_clone = dialog.clone();
        cancel_button.connect_clicked(move |_| {
            dialog_clone.close();
        });

        let action_clone = action.clone();
        let app_clone = app.clone();
        let dialog_clone = dialog.clone();
        confirm_button.connect_clicked(move |_| {
            Self::execute_action(&action_clone, &app_clone);
            dialog_clone.close();
        });

        button_box.append(&cancel_button);
        button_box.append(&confirm_button);
        content_area.append(&button_box);

        dialog.present();
    }

    fn execute_action(action: &ActionConfig, app: &Application) {
        log::info!("Executing action: {} -> {}", action.name, action.command);
        
        let result = Command::new("sh")
            .arg("-c")
            .arg(&action.command)
            .spawn();

        match result {
            Ok(_) => {
                log::info!("Successfully executed: {}", action.command);
                // Close the application after executing the action
                app.quit();
            }
            Err(e) => {
                log::error!("Failed to execute {}: {}", action.command, e);
                // You might want to show an error dialog here
            }
        }
    }

    fn setup_keyboard_shortcuts(&self, window: &ApplicationWindow) -> Result<()> {
        let controller = gtk4::EventControllerKey::new();
        
        let actions = self.config.actions.clone();
        let app = self.app.clone();
        
        controller.connect_key_pressed(move |_, key, _, _| {
            let key_name = key.name().map(|s| s.to_string().to_lowercase());
            
            if let Some(key_str) = key_name {
                for action in &actions {
                    if let Some(keybind) = &action.keybind {
                        if keybind.to_lowercase() == key_str {
                            if action.confirm {
                                // For confirmation actions, we'd need access to the window
                                // This is simplified - in practice you'd want better handling
                                log::info!("Confirmation required for action: {}", action.name);
                            } else {
                                Self::execute_action(action, &app);
                            }
                            return gtk4::glib::Propagation::Stop;
                        }
                    }
                }
            }
            
            // ESC key to close
            if key == gtk4::gdk::Key::Escape {
                app.quit();
                return gtk4::glib::Propagation::Stop;
            }
            
            gtk4::glib::Propagation::Proceed
        });

        window.add_controller(controller);
        Ok(())
    }

    fn apply_theme(&self, window: &ApplicationWindow, colors: &ThemeColors) -> Result<()> {
        let css = self.theme_manager.generate_css(colors);
        
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(&css);

        let display = gtk4::prelude::WidgetExt::display(window);
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        Ok(())
    }
}
