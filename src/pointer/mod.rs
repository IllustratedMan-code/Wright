mod imp;

use glib::Object;
use gtk4::glib;
use gtk4::EventControllerMotion;
use gtk4::Widget;

glib::wrapper! {
    pub struct Pointer(ObjectSubclass<imp::Canvas>)
        @extends gtk4::EventControllerMotion;
}

impl Pointer {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create 'Pointer'. ")
    }
}

impl Default for Pointer {
    fn default() -> Self {
        Self::new()
    }
}
