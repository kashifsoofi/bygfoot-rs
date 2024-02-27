// use glib::clone;
use glib::subclass::InitializingObject;
// use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, DropDown, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/bygfoot_rs/bygfoot/startup.ui")]
pub struct StartupWindow {
    #[template_child]
    pub combo_country: TemplateChild<DropDown>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for StartupWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "StartupWindow";
    type Type = super::StartupWindow;
    type ParentType = gtk::Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for StartupWindow {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
   }
}

// Trait shared by all widgets
impl WidgetImpl for StartupWindow {}

// Trait shared by all windows
impl WindowImpl for StartupWindow {}

// Trait shared by all application windows
impl ApplicationWindowImpl for StartupWindow {}