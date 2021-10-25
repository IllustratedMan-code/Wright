use crate::color_button::ColorButton;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct Bar {
    button: RefCell<Option<ColorButton>>,
    button2: RefCell<Option<ColorButton>>,
    scale: RefCell<Option<gtk4::Scale>>,
    chooser: RefCell<Option<ColorButton>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Bar {
    const NAME: &'static str = "Bar";
    type Type = super::Bar;
    type ParentType = gtk4::Widget;
    fn class_init(klass: &mut Self::Class) {
        klass.set_css_name("bar");
    }
}

impl ObjectImpl for Bar {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        let layout = gtk4::BoxLayout::new(gtk4::Orientation::Horizontal);
        layout.set_spacing(10);
        obj.set_layout_manager(Some(&layout));

        let button = ColorButton::new();
        button.set_size_request(50, -1);
        button.set_parent(obj);
        *self.button.borrow_mut() = Some(button);

        let button2 = ColorButton::new();
        button2.set_size_request(50, -1);
        button2.set_parent(obj);
        *self.button2.borrow_mut() = Some(button2);

        let chooser = ColorButton::new();
        chooser.set_size_request(50, -1);
        chooser.set_parent(obj);
        *self.chooser.borrow_mut() = Some(chooser);

        let scale = gtk4::Scale::with_range(gtk4::Orientation::Horizontal, 0.0, 100.0, 1.0);
        scale.set_size_request(200, -1);
        scale.set_parent(obj);
        *self.scale.borrow_mut() = Some(scale);

        obj.add_css_class("bar");

        let provider = gtk4::CssProvider::new();
        provider.load_from_data(include_bytes!("style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk4::StyleContext::add_provider_for_display(
            &gtk4::gdk::Display::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn dispose(&self, _obj: &Self::Type) {
        // Canvas widgets need to be manually unparented in `dispose()`.
        if let Some(button) = self.button.borrow_mut().take() {
            button.unparent();
        }
        if let Some(button2) = self.button2.borrow_mut().take() {
            button2.unparent();
        }
        if let Some(scale) = self.scale.borrow_mut().take() {
            scale.unparent();
        }
    }
}

impl WidgetImpl for Bar {}
