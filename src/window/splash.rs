// use glib::clone;
use glib::subclass::InitializingObject;
use gtk::ffi::gtk_window_destroy;
use gtk::prelude::ButtonExt;
// use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Button};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/bygfoot_rs/bygfoot/splash.ui")]
pub struct SplashWindow {
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SplashWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "SplashWindow";
    type Type = super::SplashWindow;
    type ParentType = gtk::Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for SplashWindow {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
   }
}

#[gtk::template_callbacks]
impl SplashWindow {
    #[template_callback]
    fn on_quit_clicked(&self, button: &Button) {
    }
}

// Trait shared by all widgets
impl WidgetImpl for SplashWindow {}

// Trait shared by all windows
impl WindowImpl for SplashWindow {}

// Trait shared by all application windows
impl ApplicationWindowImpl for SplashWindow {}