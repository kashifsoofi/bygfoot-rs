mod app;
mod splash;
mod startup;

use gtk::{gio, glib, prelude::*};

use crate::ui::splash::SplashWindow;
use crate::ui::startup::StartupWindow;

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

impl App {
    pub fn show_splash(&self) {
        let splash_window = SplashWindow::new(self);
        splash_window.present();
    }

    pub fn show_startup(&self) {
        let startup_window = StartupWindow::new(self);
        startup_window.present();
    }
}