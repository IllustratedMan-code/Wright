mod imp;

use gtk4::glib;

glib::wrapper! {
    pub struct Note(ObjectSubclass<imp::Note>)
        @extends gtk4::Widget;
}

impl Default for Note {
    fn default() -> Self {
        Self::new()
    }
}

impl Note {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
    }
}
