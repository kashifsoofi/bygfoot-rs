use gtk::{glib, subclass::prelude::*};

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
        // C: the splash/startup decision block of main() (see main.rs)
        let obj = self.obj();
        crate::decide_start_window(&obj);
    }
}
impl GtkApplicationImpl for App {}
