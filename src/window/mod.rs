mod startup;

use glib::Object;
use gtk::{gio, glib, Application};

glib::wrapper! {
    pub struct StartupWindow(ObjectSubclass<startup::StartupWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl StartupWindow {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }
}