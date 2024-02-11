use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::ui::window::{SplashWindow, StartupWindow};

#[derive(Debug, Default)]
// By implementing Default we don't have to provide a `new` fn in our
// ObjectSubclass impl.
pub struct App {}

#[glib::object_subclass]
impl ObjectSubclass for App {
    const NAME: &'static str = "App";
    type Type = super::App;
    type ParentType = gtk::Application;
}

impl ObjectImpl for App {}
impl ApplicationImpl for App {
    fn activate(&self) {
        self.parent_activate();
        // We create our window at activation stage
        self.show_splash();
    }
}
impl GtkApplicationImpl for App {}

impl App {
    pub fn show_splash(&self) {
        let splash_window = SplashWindow::new(&*self.obj());
        splash_window.present();
    }

    pub fn show_startup(&self) {
        let startup_window = StartupWindow::new(&*self.obj());
        startup_window.present();
    }
}