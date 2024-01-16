mod window;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::StartupWindow;

const APP_ID: &str = "org.bygfoot_rs.bygfoot";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("bygfoot.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let startup_window = SplashWindow::new(app);
    startup_window.present();
}