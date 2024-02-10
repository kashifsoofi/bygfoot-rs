mod ui;
mod window;
mod store;

use gtk::{glib, prelude::*};
use ui::App;
use window::{StartupWindow, SplashWindow};
use store::store::FileStore;


fn main() -> glib::ExitCode {
    // Register and include resources
    // gio::resources_register_include!("bygfoot.gresource")
    //     .expect("Failed to register resources.");

    // Create a new application
    let app = App::default();
    app.run()
}

fn build_ui(app: &gtk::Application) {
    // Create new window and present it
    let startup_window = SplashWindow::new(app);
    startup_window.present();
}