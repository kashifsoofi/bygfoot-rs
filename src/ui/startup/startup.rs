// use glib::clone;
use glib::subclass::InitializingObject;
use gtk::prelude::GtkWindowExt;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, DropDown, CompositeTemplate};

use crate::ui::App;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "startup.ui")]
pub struct StartupWindow {
    application: App,

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
        klass.bind_template_callbacks();
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

#[gtk::template_callbacks]
impl StartupWindow {
    #[template_callback]
    fn on_back_clicked(&self, _: &Button) {
        let app = &self.application.clone();
        // self.obj().destroy();
        app.show_splash();
    }

    #[template_callback]
    fn on_cancel_clicked(&self, _: &Button) {
        let application = self.obj().application().unwrap();
        self.obj().destroy();
        application.quit();
    }
}

// Trait shared by all widgets
impl WidgetImpl for StartupWindow {}

// Trait shared by all windows
impl WindowImpl for StartupWindow {}

// Trait shared by all application windows
impl ApplicationWindowImpl for StartupWindow {}