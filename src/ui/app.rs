use gtk::{glib, prelude::*, subclass::prelude::*};

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
        let window = gtk::ApplicationWindow::new(&*self.obj());
        window.set_default_size(600, 350);
        window.set_title(Some("Application Subclass"));

        let label = gtk::Label::new(Some("Hello"));
        label.add_css_class("title-2");
        window.set_child(Some(&label));
        window.present();
    }
}
impl GtkApplicationImpl for App {}