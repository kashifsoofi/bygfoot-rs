mod splash;

use glib::Object;
use gtk::{gio, glib};

use crate::ui::App;

glib::wrapper! {
    pub struct SplashWindow(ObjectSubclass<splash::SplashWindow>)
        @extends gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root;
}

impl SplashWindow {
    pub fn new(app: &App) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }
}

