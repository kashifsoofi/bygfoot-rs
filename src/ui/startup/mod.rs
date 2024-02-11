mod startup;

use glib::Object;
use gtk::{gio, glib};

use crate::ui::App;

glib::wrapper! {
    pub struct StartupWindow(ObjectSubclass<startup::StartupWindow>)
        @extends gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl StartupWindow {
    pub fn new(app: &App) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }
}
