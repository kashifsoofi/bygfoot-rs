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
        // We create our window at activation stage
        let obj=self.obj();
        obj.show_splash();
    }
}
impl GtkApplicationImpl for App {}
