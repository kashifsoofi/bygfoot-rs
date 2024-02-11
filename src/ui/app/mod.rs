use gtk::{glib, prelude::*, subclass::prelude::*};

mod splash;

use splash::SplashWindow;

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
        let startup_window = SplashWindow::new(&*self.obj());
        startup_window.present();
    }
}
impl GtkApplicationImpl for App {}