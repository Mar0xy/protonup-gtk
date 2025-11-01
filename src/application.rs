use gtk::prelude::*;
use libadwaita as adw;
use std::sync::Arc;

use crate::window::MainWindow;

const APP_ID: &str = "com.github.Mar0xy.ProtonUpGtk";

pub struct Application {
    app: adw::Application,
    _runtime: tokio::runtime::Runtime,
}

impl Application {
    pub fn new() -> Self {
        // Create a Tokio runtime for async operations
        let runtime = tokio::runtime::Runtime::new()
            .expect("Failed to create Tokio runtime");
        
        let runtime_handle = Arc::new(runtime.handle().clone());
        
        let app = adw::Application::builder()
            .application_id(APP_ID)
            .build();

        app.connect_activate(move |app| {
            let window = MainWindow::new(app, runtime_handle.clone());
            window.present();
        });

        Self { 
            app,
            _runtime: runtime,
        }
    }

    pub fn run(&self) -> i32 {
        self.app.run().into()
    }
}
