use gtk::prelude::*;
use libadwaita as adw;
use adw::prelude::*;
use gtk::{Button, Box, Orientation, Label, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::{ToolManager, Downloader};

pub struct MainWindow {
    window: adw::ApplicationWindow,
    tool_manager: Rc<RefCell<ToolManager>>,
    downloader: Rc<RefCell<Downloader>>,
    toast_overlay: adw::ToastOverlay,
}

impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("ProtonUp-GTK")
            .default_width(900)
            .default_height(600)
            .build();

        let tool_manager = Rc::new(RefCell::new(ToolManager::new()));
        let downloader = Rc::new(RefCell::new(Downloader::new()));

        // Create toast overlay for notifications
        let toast_overlay = adw::ToastOverlay::new();

        let header_bar = adw::HeaderBar::builder().build();
        
        // Add menu button for settings
        let menu_button = gtk::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .build();
        header_bar.pack_end(&menu_button);
        
        // Create main content
        let toolbar = adw::ToolbarView::new();
        toolbar.add_top_bar(&header_bar);
        
        // Create main container
        let main_box = Box::new(Orientation::Vertical, 12);
        main_box.set_margin_top(12);
        main_box.set_margin_bottom(12);
        main_box.set_margin_start(12);
        main_box.set_margin_end(12);
        
        // Welcome section
        let welcome_box = Box::new(Orientation::Vertical, 6);
        let title_label = Label::new(Some("ProtonUp-GTK"));
        title_label.add_css_class("title-1");
        let subtitle_label = Label::new(Some("Install and manage compatibility tools for Steam and Lutris"));
        subtitle_label.add_css_class("dim-label");
        welcome_box.append(&title_label);
        welcome_box.append(&subtitle_label);
        main_box.append(&welcome_box);
        
        // Tool list section
        let list_group = adw::PreferencesGroup::builder()
            .title("Compatibility Tools")
            .description("Available compatibility tools for installation")
            .build();
        
        // Create list of tools with install functionality
        let tools = vec![
            ("GE-Proton", "Proton compatibility tool for Steam", "Steam"),
            ("Wine-GE", "Wine compatibility tool for Lutris", "Lutris"),
            ("Luxtorpeda", "Steam Play compatibility tool", "Steam"),
            ("Spritz-Wine", "Wine builds optimized for gaming performance", "Lutris"),
            ("dwproton", "Dawn Wine Proton - Proton fork with improvements", "Steam"),
        ];
        
        for (name, description, launcher) in tools {
            let row = adw::ActionRow::builder()
                .title(name)
                .subtitle(description)
                .build();
            
            let badge = Label::new(Some(launcher));
            badge.add_css_class("caption");
            badge.add_css_class("dim-label");
            row.add_suffix(&badge);
            
            let install_button = Button::builder()
                .label("Install")
                .valign(gtk::Align::Center)
                .build();
            install_button.add_css_class("suggested-action");
            
            // Clone needed variables for the closure
            let tool_name = name.to_string();
            let toast_overlay_clone = toast_overlay.clone();
            let tool_manager_clone = tool_manager.clone();
            let downloader_clone = downloader.clone();
            let install_button_clone = install_button.clone();
            
            install_button.connect_clicked(move |_| {
                let tool_name = tool_name.clone();
                let toast_overlay = toast_overlay_clone.clone();
                let tool_manager = tool_manager_clone.clone();
                let downloader = downloader_clone.clone();
                let button = install_button_clone.clone();
                
                // Disable button during installation
                button.set_sensitive(false);
                button.set_label("Installing...");
                
                // Spawn async task to handle installation
                glib::MainContext::default().spawn_local(async move {
                    let result = Self::install_tool(
                        &tool_name,
                        tool_manager,
                        downloader,
                    ).await;
                    
                    // Re-enable button
                    button.set_sensitive(true);
                    button.set_label("Install");
                    
                    // Show result notification
                    match result {
                        Ok(message) => {
                            let toast = adw::Toast::new(&message);
                            toast.set_timeout(3);
                            toast_overlay.add_toast(toast);
                        }
                        Err(e) => {
                            let error_msg = format!("Installation failed: {}", e);
                            let toast = adw::Toast::new(&error_msg);
                            toast.set_timeout(5);
                            toast_overlay.add_toast(toast);
                        }
                    }
                });
            });
            
            row.add_suffix(&install_button);
            list_group.add(&row);
        }
        
        main_box.append(&list_group);
        
        // Add refresh button
        let button_box = Box::new(Orientation::Horizontal, 6);
        button_box.set_halign(gtk::Align::Center);
        let refresh_button = Button::builder()
            .label("Refresh Tool List")
            .build();
        
        let toast_overlay_refresh = toast_overlay.clone();
        let tool_manager_refresh = tool_manager.clone();
        refresh_button.connect_clicked(move |btn| {
            btn.set_sensitive(false);
            let toast_overlay = toast_overlay_refresh.clone();
            let tool_manager = tool_manager_refresh.clone();
            let button = btn.clone();
            
            glib::MainContext::default().spawn_local(async move {
                let result = tool_manager.borrow_mut().fetch_available_tools().await;
                button.set_sensitive(true);
                
                match result {
                    Ok(tools) => {
                        let msg = format!("Found {} compatibility tools", tools.len());
                        let toast = adw::Toast::new(&msg);
                        toast.set_timeout(3);
                        toast_overlay.add_toast(toast);
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to refresh: {}", e);
                        let toast = adw::Toast::new(&error_msg);
                        toast.set_timeout(5);
                        toast_overlay.add_toast(toast);
                    }
                }
            });
        });
        
        button_box.append(&refresh_button);
        main_box.append(&button_box);
        
        // Set up scrolled window
        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&main_box));
        scrolled.set_vexpand(true);
        
        toolbar.set_content(Some(&scrolled));
        toast_overlay.set_child(Some(&toolbar));
        
        window.set_content(Some(&toast_overlay));

        // Setup menu
        Self::setup_menu(&menu_button, &window, &toast_overlay);

        Self { 
            window,
            tool_manager,
            downloader,
            toast_overlay,
        }
    }

    async fn install_tool(
        tool_name: &str,
        tool_manager: Rc<RefCell<ToolManager>>,
        downloader: Rc<RefCell<Downloader>>,
    ) -> anyhow::Result<String> {
        // Fetch available tools to get download URL
        let tools = tool_manager.borrow_mut().fetch_available_tools().await?;
        
        let tool = tools.iter()
            .find(|t| t.name == tool_name)
            .ok_or_else(|| anyhow::anyhow!("Tool '{}' not found", tool_name))?;
        
        // Get install path
        let install_path = tool_manager.borrow().get_install_path(&tool.launcher)?;
        
        // Create install directory if it doesn't exist
        tokio::fs::create_dir_all(&install_path).await?;
        
        // Determine archive filename from URL
        let url_path = tool.download_url.split('/').last()
            .ok_or_else(|| anyhow::anyhow!("Invalid download URL"))?;
        
        // Download to temp directory
        let temp_dir = std::env::temp_dir();
        let archive_path = temp_dir.join(url_path);
        
        // Download the file
        downloader.borrow().download_file(&tool.download_url, &archive_path).await?;
        
        // Extract to install path
        downloader.borrow().extract_archive(&archive_path, &install_path).await?;
        
        // Clean up downloaded archive
        let _ = tokio::fs::remove_file(&archive_path).await;
        
        Ok(format!("{} {} installed successfully!", tool.name, tool.version))
    }

    fn setup_menu(menu_button: &gtk::MenuButton, window: &adw::ApplicationWindow, toast_overlay: &adw::ToastOverlay) {
        let menu = gtk::gio::Menu::new();
        
        menu.append(Some("Preferences"), Some("app.preferences"));
        menu.append(Some("About"), Some("app.about"));
        
        menu_button.set_menu_model(Some(&menu));
        
        // Create actions
        let preferences_action = gtk::gio::SimpleAction::new("preferences", None);
        let window_clone = window.clone();
        let toast_overlay_clone = toast_overlay.clone();
        preferences_action.connect_activate(move |_, _| {
            Self::show_preferences_dialog(&window_clone, &toast_overlay_clone);
        });
        
        let about_action = gtk::gio::SimpleAction::new("about", None);
        let window_clone = window.clone();
        about_action.connect_activate(move |_, _| {
            Self::show_about_dialog(&window_clone);
        });
        
        let app = window.application().unwrap();
        app.add_action(&preferences_action);
        app.add_action(&about_action);
    }

    fn show_preferences_dialog(window: &adw::ApplicationWindow, toast_overlay: &adw::ToastOverlay) {
        let dialog = adw::PreferencesWindow::builder()
            .transient_for(window)
            .modal(true)
            .search_enabled(false)
            .build();
        
        dialog.set_title("Preferences");
        
        // General settings page
        let page = adw::PreferencesPage::builder()
            .title("General")
            .icon_name("preferences-system-symbolic")
            .build();
        
        // Paths group
        let paths_group = adw::PreferencesGroup::builder()
            .title("Installation Paths")
            .description("Configure where compatibility tools are installed")
            .build();
        
        // Steam path
        let steam_row = adw::ActionRow::builder()
            .title("Steam Tools Path")
            .subtitle("~/.steam/root/compatibilitytools.d")
            .build();
        paths_group.add(&steam_row);
        
        // Lutris path
        let lutris_row = adw::ActionRow::builder()
            .title("Lutris Runners Path")
            .subtitle("~/.local/share/lutris/runners/wine")
            .build();
        paths_group.add(&lutris_row);
        
        page.add(&paths_group);
        
        // Auto-update group
        let update_group = adw::PreferencesGroup::builder()
            .title("Updates")
            .description("Automatic update settings")
            .build();
        
        let auto_check_row = adw::ActionRow::builder()
            .title("Check for Updates")
            .subtitle("Automatically check for new tool versions")
            .build();
        
        let auto_check_switch = gtk::Switch::builder()
            .valign(gtk::Align::Center)
            .build();
        auto_check_row.add_suffix(&auto_check_switch);
        auto_check_row.set_activatable_widget(Some(&auto_check_switch));
        
        update_group.add(&auto_check_row);
        page.add(&update_group);
        
        dialog.add(&page);
        dialog.present();
    }

    fn show_about_dialog(window: &adw::ApplicationWindow) {
        let about = adw::AboutWindow::builder()
            .transient_for(window)
            .application_name("ProtonUp-GTK")
            .application_icon("com.github.Mar0xy.ProtonUpGtk")
            .developer_name("Mar0xy")
            .version("0.2.0")
            .comments("Install and manage compatibility tools for Steam and Lutris")
            .website("https://github.com/Mar0xy/protonup-gtk")
            .issue_url("https://github.com/Mar0xy/protonup-gtk/issues")
            .license_type(gtk::License::Gpl30)
            .build();
        
        about.add_credit_section(
            Some("Compatibility Tools"),
            &[
                "GE-Proton by GloriousEggroll",
                "Wine-GE by GloriousEggroll",
                "Luxtorpeda by luxtorpeda-dev",
                "Spritz-Wine by NelloKudo",
                "dwproton by Dawn Wine",
            ],
        );
        
        about.present();
    }

    pub fn present(&self) {
        self.window.present();
    }
}
