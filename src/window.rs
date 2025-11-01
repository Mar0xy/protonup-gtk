use gtk::prelude::*;
use libadwaita as adw;
use adw::prelude::*;
use gtk::{Button, Box, Orientation, Label, ScrolledWindow, ListBox};

pub struct MainWindow {
    window: adw::ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("ProtonUp-GTK")
            .default_width(900)
            .default_height(600)
            .build();

        let header_bar = adw::HeaderBar::builder().build();
        
        // Create main content
        let content = Box::new(Orientation::Vertical, 0);
        
        // Create toolbar box
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
        
        // Create list of tools
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
        button_box.append(&refresh_button);
        main_box.append(&button_box);
        
        // Set up scrolled window
        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&main_box));
        scrolled.set_vexpand(true);
        
        toolbar.set_content(Some(&scrolled));
        
        window.set_content(Some(&toolbar));

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
