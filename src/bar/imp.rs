use crate::canvas::Canvas;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct Bar {
    canvas: RefCell<Option<gtk4::Picture>>,
    button: RefCell<Option<gtk4::Button>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Bar {
    const NAME: &'static str = "Bar";
    type Type = super::Note;
    type ParentType = gtk4::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_css_name("bar");
    }
}

impl ObjectImpl for Bar {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        let layout = gtk4::BoxLayout::new(gtk4::Orientation::Vertical);
        obj.set_layout_manager(Some(&layout));

        let button = gtk4::Button::new();

        button.set_parent(obj);
        note_canvas.set_parent(obj);

        *self.button.borrow_mut() = Some(button);

        obj.add_css_class("bar");
    }

    fn dispose(&self, _obj: &Self::Type) {
        // Canvas widgets need to be manually unparented in `dispose()`.
        if let Some(button) = self.button.borrow_mut().take() {
            button.unparent();
        }
    }
}

impl WidgetImpl for Note {}
