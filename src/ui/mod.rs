mod app;

use gtk::{gio, glib};

const APP_ID: &str = "org.bygfoot_rs.bygfoot";

glib::wrapper! {
    pub struct App(ObjectSubclass<app::App>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for App {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .build()
    }
}