mod ui;
mod store;
mod domain;

use gtk::{glib, prelude::*};
use ui::App;

fn main() -> glib::ExitCode {
    // Create a new application
    let app = App::default();
    app.run()
}
