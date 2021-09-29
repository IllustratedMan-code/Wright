use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;

pub struct Pointer {
    x: f64,
    y: f64,
}
impl Default for Pointer {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Pointer {
    const NAME: &'static str = "GtkNoteCanvas";
    type Type = super::Pointer;
    type ParentType = gtk4::EventControllerMotion;
}

impl ObjectImpl for Pointer {}

impl EventControllerImpl for Pointer {}
