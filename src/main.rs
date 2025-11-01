mod backend;

#[cfg(feature = "gui")]
mod application;
#[cfg(feature = "gui")]
mod window;

#[cfg(feature = "gui")]
use application::Application;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "gui")]
    {
        let app = Application::new();
        let exit_code = app.run();
        std::process::exit(exit_code);
    }
    
    #[cfg(not(feature = "gui"))]
    {
        eprintln!("This binary was built without GUI support.");
        eprintln!("Please rebuild with: cargo build --features gui");
        std::process::exit(1);
    }
}
