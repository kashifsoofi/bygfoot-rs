use std::cell::{Cell, RefCell};

use glib::Properties;
use glib::subclass::InitializingObject;
use gtk::glib::{self, ObjectExt};
use gtk::{prelude::*, StringObject};
use gtk::subclass::prelude::*;
use gtk::{Align, CompositeTemplate, Button, Label, ListView, ListItem,
            StringList, SignalListItemFactory, NoSelection,};

use crate::store::hints_store::FileHintsStore;
use crate::store::help_store::FileHelpStore;
use crate::ui::App;

// Object holding the state
#[derive(CompositeTemplate, Default, Properties)]
#[properties(wrapper_type = super::SplashWindow)]
#[template(resource = "/org/bygfoot_rs/bygfoot/splash.ui")]
pub struct SplashWindow {
    // #[property(get, set)]
    application: App,

    #[template_child]
    listview_contributors: TemplateChild<ListView>, 
    #[template_child]
    label_hint_counter: TemplateChild<Label>,
    #[template_child]
    label_hint: TemplateChild<Label>,
    
    #[property(get, set)]
    hints: RefCell<Vec<String>>,
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

        let obj = self.obj();

        let hints_store = FileHintsStore::new();
        let hint_num = hints_store.load_hint_number();
        obj.set_hint_num(hint_num);

        let hints = hints_store.get_hints();
        obj.set_hints(hints);

        self.show_hint();

        self.show_contributors();
   }
}

#[gtk::template_callbacks]
impl SplashWindow {
    fn show_contributors(&self) {
        let help_store = FileHelpStore::new();
        let contributors = help_store.get_contributors();

        let model = StringList::new(&[]);
        for c in contributors.iter() {
            model.append(format!("\n<span size='large'>{}</span>", c.title).as_str());
            for e in c.entries.iter() {
                model.append(e.as_str());
            }
        }

        let selection_model = NoSelection::new(Some(model));
        self.listview_contributors.set_model(Some(&selection_model));

        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = Label::new(None);
            label.set_halign(Align::Start);
            label.set_can_target(false);
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&label));

            // Bind `list_item->item->string` to `label->label`
            // list_item
            //     .property_expression("item")
            //     .chain_property::<StringObject>("string")
            //     .bind(&label, "markup", Widget::NONE);
        });

        factory.connect_bind(move |_, list_item| {
            // Get `Label` from `ListItem`
            let label = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<Label>()
                .expect("The child has to be a `Label`.");

            // Get `String` from `ListItem`
            let item = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<StringObject>()
                .expect("The item has to be a StringObject");

            label.set_markup(item.string().as_str());
        });

        self.listview_contributors.set_factory(Some(&factory));
    }

    fn show_hint(&self) {
        let obj = self.obj();
        let total_hints = obj.hints().len();

        let hint_num = obj.hint_num();
        let hint_counter = format!("({}/{})", hint_num + 1, total_hints);
        self.label_hint_counter.set_label(&hint_counter);

        let hints = obj.hints();
        let hint = hints[hint_num as usize].as_str();
        self.label_hint.set_label(hint);
    }

    #[template_callback]
    fn on_hint_back_clicked(&self, _: &Button) {
        let obj = self.obj();
        let mut hint_num = obj.hint_num();
        if hint_num == 0 {
            hint_num = obj.hints().len() as i32 - 1;
        } else {
            hint_num -= 1;
        }

        obj.set_hint_num(hint_num);
        self.show_hint()
    }
    
    #[template_callback]
    fn on_hint_next_clicked(&self, _: &Button) {
        let obj = self.obj();        
        let hint_num = (obj.hint_num() + 1) % obj.hints().len() as i32;
        obj.set_hint_num(hint_num);
        self.show_hint()
    }
    
    #[template_callback]
    fn on_new_game_clicked(&self, _: &Button) {
        self.application.show_startup();
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