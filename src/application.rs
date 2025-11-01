use gtk::prelude::*;
use libadwaita as adw;

use crate::window::MainWindow;

const APP_ID: &str = "com.github.Mar0xy.ProtonUpGtk";

pub struct Application {
    app: adw::Application,
}

impl Application {
    pub fn new() -> Self {
        let app = adw::Application::builder()
            .application_id(APP_ID)
            .build();

        app.connect_activate(|app| {
            let window = MainWindow::new(app);
            window.present();
        });

        Self { app }
    }

    pub fn run(&self) -> i32 {
        self.app.run().into()
    }
}
