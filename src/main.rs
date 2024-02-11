mod ui;
mod store;

use gtk::{gio, glib, prelude::*};
use ui::App;

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("bygfoot.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = App::default();
    app.run()
}
