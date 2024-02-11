use std::cell::{Cell, RefCell};

use glib::Properties;
use glib::subclass::InitializingObject;
use gtk::glib::{self, ObjectExt};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, Button, Label};

use crate::ui::App;

// Object holding the state
#[derive(CompositeTemplate, Default, Properties)]
#[properties(wrapper_type = super::SplashWindow)]
#[template(resource = "/org/bygfoot_rs/bygfoot/splash.ui")]
pub struct SplashWindow {
    // #[property(get, set)]
    application: App,

    #[template_child]
    label_hint_counter: TemplateChild<Label>,
    #[template_child]
    label_hint: TemplateChild<Label>,
    
    #[property(get, set)]
    hints: Cell<i32>,
    #[property(get, set)]
    hint_num: Cell<i32>,
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
#[glib::derived_properties]
impl ObjectImpl for SplashWindow {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
   }
}

#[gtk::template_callbacks]
impl SplashWindow {
    fn show_hint(&self) {
        let total_hints = (self.obj().hints() + 10) as usize;
        let mut hint_num = self.obj().hint_num();
        if hint_num < 0 {
            hint_num = (total_hints - 1) as i32;
        }
        if hint_num >= (total_hints as i32) {
            hint_num = 0;
        }
        self.obj().set_hint_num(hint_num);

        let hint_counter = format!("({}/{})", hint_num + 1, total_hints);
        self.label_hint_counter.set_label(&hint_counter);
    }

    #[template_callback]
    fn on_hint_back_clicked(&self, _: &Button) {
        let hint_num = self.obj().hint_num() - 1;
        self.obj().set_hint_num(hint_num);
        self.show_hint()
    }
    
    #[template_callback]
    fn on_hint_next_clicked(&self, _: &Button) {
        let hint_num = self.obj().hint_num() + 1;
        self.obj().set_hint_num(hint_num);
        self.show_hint()
    }
    
    #[template_callback]
    fn on_new_game_clicked(&self, _: &Button) {
        //self.application.show_startup();
        let startup_window = super::StartupWindow::new(&self.application);
        startup_window.present();
    }
    
    #[template_callback]
    fn on_load_game_clicked(&self, _: &Button) {
    }
    
    #[template_callback]
    fn on_resume_game_clicked(&self, _: &Button) {
    }
    
    #[template_callback]
    fn on_quit_clicked(&self, _: &Button) {
        self.application.quit();
    }
}

// Trait shared by all widgets
impl WidgetImpl for SplashWindow {}

// Trait shared by all windows
impl WindowImpl for SplashWindow {}

// Trait shared by all application windows
impl ApplicationWindowImpl for SplashWindow {}