use gtk::prelude::*;
use libadwaita as adw;
use adw::prelude::*;
use gtk::{Button, Box, Orientation, Label, ScrolledWindow};
use std::sync::{Arc, Mutex};

use crate::backend::{ToolManager, Downloader};

pub struct MainWindow {
    window: adw::ApplicationWindow,
    tool_manager: Arc<Mutex<ToolManager>>,
    downloader: Arc<Mutex<Downloader>>,
    toast_overlay: adw::ToastOverlay,
    runtime_handle: Arc<tokio::runtime::Handle>,
}

impl MainWindow {
    pub fn new(app: &adw::Application, runtime_handle: Arc<tokio::runtime::Handle>) -> Self {
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("ProtonUp-GTK")
            .default_width(900)
            .default_height(600)
            .build();

        let tool_manager = Arc::new(Mutex::new(ToolManager::new()));
        let downloader = Arc::new(Mutex::new(Downloader::new()));

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
        
        // Tool list section - will be populated dynamically
        let list_group = adw::PreferencesGroup::builder()
            .title("Compatibility Tools")
            .description("Select a version to install")
            .build();
        
        main_box.append(&list_group);
        
        // Add refresh button
        let button_box = Box::new(Orientation::Horizontal, 6);
        button_box.set_halign(gtk::Align::Center);
        let refresh_button = Button::builder()
            .label("Refresh Tool List")
            .build();
        
        let toast_overlay_refresh = toast_overlay.clone();
        let tool_manager_refresh = tool_manager.clone();
        let list_group_refresh = list_group.clone();
        let downloader_refresh = downloader.clone();
        let runtime_handle_refresh = runtime_handle.clone();
        
        // Store references to added expander rows so we can remove them on refresh
        let expander_rows: Arc<Mutex<Vec<adw::ExpanderRow>>> = Arc::new(Mutex::new(Vec::new()));
        let expander_rows_refresh = expander_rows.clone();
        
        refresh_button.connect_clicked(move |btn| {
            btn.set_sensitive(false);
            let toast_overlay = toast_overlay_refresh.clone();
            let tool_manager = tool_manager_refresh.clone();
            let list_group = list_group_refresh.clone();
            let button = btn.clone();
            let downloader = downloader_refresh.clone();
            let runtime_handle = runtime_handle_refresh.clone();
            let expander_rows = expander_rows_refresh.clone();
            
            glib::MainContext::default().spawn_local(async move {
                // Enter the Tokio runtime context for the async operations
                let _guard = runtime_handle.enter();
                
                let result = tool_manager.lock()
                    .expect("Failed to lock tool manager")
                    .fetch_tools_with_versions()
                    .await;
                
                button.set_sensitive(true);
                
                match result {
                    Ok(tools) => {
                        // Clear existing rows that we previously added
                        {
                            let mut rows = expander_rows.lock().expect("Failed to lock expander rows");
                            for row in rows.drain(..) {
                                list_group.remove(&row);
                            }
                        }
                        
                        // Add new rows with versions
                        for tool in &tools {
                            let expander = Self::add_tool_with_versions(
                                &list_group,
                                tool,
                                tool_manager.clone(),
                                downloader.clone(),
                                toast_overlay.clone(),
                                runtime_handle.clone(),
                            );
                            // Store the expander so we can remove it next time
                            expander_rows.lock().expect("Failed to lock expander rows").push(expander);
                        }
                        
                        let msg = format!("Loaded {} compatibility tools", tools.len());
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
            runtime_handle,
        }
    }

    fn add_tool_with_versions(
        list_group: &adw::PreferencesGroup,
        tool: &crate::backend::ToolWithVersions,
        tool_manager: Arc<Mutex<ToolManager>>,
        downloader: Arc<Mutex<Downloader>>,
        toast_overlay: adw::ToastOverlay,
        runtime_handle: Arc<tokio::runtime::Handle>,
    ) -> adw::ExpanderRow {
        // Create expander row for the tool
        let expander = adw::ExpanderRow::builder()
            .title(&tool.name)
            .subtitle(&tool.description)
            .build();
        
        // Add launcher badge
        let launcher_text = tool.launcher.to_string();
        let badge = Label::new(Some(&launcher_text));
        badge.add_css_class("caption");
        badge.add_css_class("dim-label");
        expander.add_suffix(&badge);
        
        // Add version rows
        for version in &tool.versions {
            let version_row = adw::ActionRow::builder()
                .title(&version.version)
                .build();
            
            // Check if this version is already installed
            let is_installed = tool_manager.lock()
                .expect("Failed to lock tool manager")
                .is_tool_installed(&version.version, &tool.launcher);
            
            let action_button = Button::builder()
                .label(if is_installed { "Delete" } else { "Install" })
                .valign(gtk::Align::Center)
                .build();
            
            if is_installed {
                action_button.add_css_class("destructive-action");
            } else {
                action_button.add_css_class("suggested-action");
            }
            
            // Clone for closure
            let download_url = version.download_url.clone();
            let version_str = version.version.clone();
            let tool_name = tool.name.clone();
            let launcher = tool.launcher.clone();
            let tool_manager_clone = tool_manager.clone();
            let downloader_clone = downloader.clone();
            let toast_overlay_clone = toast_overlay.clone();
            let button_clone = action_button.clone();
            let runtime_handle_clone = runtime_handle.clone();
            
            action_button.connect_clicked(move |_| {
                let download_url = download_url.clone();
                let version = version_str.clone();
                let tool_name = tool_name.clone();
                let launcher = launcher.clone();
                let tool_manager = tool_manager_clone.clone();
                let downloader = downloader_clone.clone();
                let toast_overlay = toast_overlay_clone.clone();
                let button = button_clone.clone();
                let runtime_handle = runtime_handle_clone.clone();
                
                // Check if we're deleting or installing
                let button_label = button.label().unwrap_or_default();
                let is_delete = button_label.as_str() == "Delete";
                
                button.set_sensitive(false);
                
                if is_delete {
                    // Handle deletion
                    button.set_label("Deleting...");
                    
                    glib::MainContext::default().spawn_local(async move {
                        let result = Self::delete_tool_version(
                            &version,
                            &launcher,
                            tool_manager,
                        ).await;
                        
                        match result {
                            Ok(message) => {
                                button.set_label("Install");
                                button.remove_css_class("destructive-action");
                                button.add_css_class("suggested-action");
                                button.set_sensitive(true);
                                
                                let toast = adw::Toast::new(&message);
                                toast.set_timeout(5);
                                toast_overlay.add_toast(toast);
                            }
                            Err(e) => {
                                button.set_label("Delete");
                                button.set_sensitive(true);
                                
                                let error_msg = format!("Deletion failed: {}", e);
                                let toast = adw::Toast::new(&error_msg);
                                toast.set_timeout(5);
                                toast_overlay.add_toast(toast);
                            }
                        }
                    });
                } else {
                    // Handle installation
                    button.set_label("Installing...");
                    
                    glib::MainContext::default().spawn_local(async move {
                        // Enter the Tokio runtime context for async operations
                        let _guard = runtime_handle.enter();
                        
                        let button_for_progress = button.clone();
                        let result = Self::install_tool_version(
                            &tool_name,
                            &version,
                            &download_url,
                            &launcher,
                            tool_manager,
                            downloader,
                            move |progress_msg| {
                                // We're already in the GLib main context, so we can update directly
                                button_for_progress.set_label(&progress_msg);
                            },
                        ).await;
                        
                        match result {
                            Ok(message) => {
                                button.set_label("Delete");
                                button.remove_css_class("suggested-action");
                                button.add_css_class("destructive-action");
                                button.set_sensitive(true);
                                
                                let toast = adw::Toast::new(&message);
                                toast.set_timeout(5);
                                toast_overlay.add_toast(toast);
                            }
                            Err(e) => {
                                button.set_label("Install");
                                button.set_sensitive(true);
                                
                                let error_msg = format!("Installation failed: {}", e);
                                let toast = adw::Toast::new(&error_msg);
                                toast.set_timeout(5);
                                toast_overlay.add_toast(toast);
                            }
                        }
                    });
                }
            });
            
            version_row.add_suffix(&action_button);
            expander.add_row(&version_row);
        }
        
        list_group.add(&expander);
        expander  // Return the expander so it can be tracked for removal
    }

    async fn install_tool_version<F>(
        tool_name: &str,
        version: &str,
        download_url: &str,
        launcher: &crate::backend::Launcher,
        tool_manager: Arc<Mutex<ToolManager>>,
        downloader: Arc<Mutex<Downloader>>,
        mut progress_callback: F,
    ) -> anyhow::Result<String>
    where
        F: FnMut(String),
    {
        // Get install path
        let install_path = tool_manager.lock()
            .expect("Failed to lock tool manager")
            .get_install_path(launcher)?;
        
        // Create install directory if it doesn't exist
        tokio::fs::create_dir_all(&install_path).await?;
        
        // Determine archive filename from URL
        let url_path = download_url.split('/').last()
            .ok_or_else(|| anyhow::anyhow!("Invalid download URL"))?;
        
        // Download to temp directory
        let temp_dir = std::env::temp_dir();
        let archive_path = temp_dir.join(url_path);
        
        // Download the file with progress
        progress_callback("Downloading (0%)".to_string());
        downloader.lock()
            .expect("Failed to lock downloader")
            .download_file_with_progress(download_url, &archive_path, |progress| {
                let msg = format!("Downloading ({:.0}%)", progress);
                progress_callback(msg);
            })
            .await?;
        
        // Extract to install path
        progress_callback("Extracting...".to_string());
        downloader.lock()
            .expect("Failed to lock downloader")
            .extract_archive(&archive_path, &install_path)
            .await?;
        
        // Clean up downloaded archive
        let _ = tokio::fs::remove_file(&archive_path).await;
        
        Ok(format!("{} {} installed successfully!", tool_name, version))
    }

    async fn delete_tool_version(
        version: &str,
        launcher: &crate::backend::Launcher,
        tool_manager: Arc<Mutex<ToolManager>>,
    ) -> anyhow::Result<String> {
        // Get install path
        let install_path = tool_manager.lock()
            .expect("Failed to lock tool manager")
            .get_install_path(launcher)?;
        
        // Find the directory for this version
        let version_path = install_path.join(version);
        
        if version_path.exists() {
            // Delete the directory
            tokio::fs::remove_dir_all(&version_path).await?;
            Ok(format!("{} deleted successfully!", version))
        } else {
            Err(anyhow::anyhow!("Tool version {} not found", version))
        }
    }

    async fn install_tool(
        tool_name: &str,
        tool_manager: Arc<Mutex<ToolManager>>,
        downloader: Arc<Mutex<Downloader>>,
    ) -> anyhow::Result<String> {
        // Fetch available tools to get download URL
        let tools = tool_manager.lock()
            .expect("Failed to lock tool manager")
            .fetch_available_tools()
            .await?;
        
        let tool = tools.iter()
            .find(|t| t.name == tool_name)
            .ok_or_else(|| anyhow::anyhow!("Tool '{}' not found", tool_name))?;
        
        // Get install path
        let install_path = tool_manager.lock()
            .expect("Failed to lock tool manager")
            .get_install_path(&tool.launcher)?;
        
        // Create install directory if it doesn't exist
        tokio::fs::create_dir_all(&install_path).await?;
        
        // Determine archive filename from URL
        let url_path = tool.download_url.split('/').last()
            .ok_or_else(|| anyhow::anyhow!("Invalid download URL"))?;
        
        // Download to temp directory
        let temp_dir = std::env::temp_dir();
        let archive_path = temp_dir.join(url_path);
        
        // Download the file
        downloader.lock()
            .expect("Failed to lock downloader")
            .download_file(&tool.download_url, &archive_path)
            .await?;
        
        // Extract to install path
        downloader.lock()
            .expect("Failed to lock downloader")
            .extract_archive(&archive_path, &install_path)
            .await?;
        
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

    fn show_preferences_dialog(window: &adw::ApplicationWindow, _toast_overlay: &adw::ToastOverlay) {
        let dialog = adw::PreferencesWindow::builder()
            .transient_for(window)
            .modal(true)
            .search_enabled(false)
            .build();
        
        dialog.set_title(Some("Preferences"));
        
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
            .version("0.3.0")
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
