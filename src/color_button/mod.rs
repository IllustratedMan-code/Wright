mod imp;

use gtk4::glib;

glib::wrapper! {
    pub struct ColorButton(ObjectSubclass<imp::ColorButton>)
        @extends gtk4::Widget;
}

impl Default for ColorButton {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorButton {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
    }
}
