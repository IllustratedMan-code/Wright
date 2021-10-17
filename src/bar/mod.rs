mod imp;

use gtk4::glib;

glib::wrapper! {
    pub struct Bar(ObjectSubclass<imp::Bar>)
        @extends gtk4::Widget;
}

impl Default for Bar {
    fn default() -> Self {
        Self::new()
    }
}

impl Bar {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
    }
}
