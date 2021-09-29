mod imp;

use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gdk, glib};
use std::cell::{Cell, RefCell};

glib::wrapper! {
    pub struct Canvas(ObjectSubclass<imp::Canvas>) @implements gdk::Paintable;
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Canvas {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a Canvas")
    }
    pub fn change(&self, x: f64, y: f64) {
        let _self = imp::Canvas::from_instance(self);
        if _self.is_drawing.get() {
            let mut l = _self.lines.borrow_mut();
            match l.last() {
                Some(p) => {
                    p.borrow_mut().push(imp::point {
                        x: x,
                        y: y,
                        size: 3.0,
                    });
                }
                None => (),
            }
            println!("len: {}", l.len())
        }
        _self.x.set(x);
        _self.y.set(y);
        self.invalidate_contents();
    }
    pub fn start_line(&self, x: f64, y: f64) {
        let _self = imp::Canvas::from_instance(self);
        _self.is_drawing.set(true);
        let mut l = _self.lines.borrow_mut();
        let r = RefCell::new(vec![imp::point {
            x: x,
            y: y,
            size: 3.0,
        }]);
        l.push(r);
        println!("is: {}", _self.is_drawing.get());
        self.invalidate_contents();
    }
    pub fn end_line(&self, x: f64, y: f64) {
        let _self = imp::Canvas::from_instance(self);
        _self.is_drawing.set(false);
        println!("is: {}", _self.is_drawing.get());
        self.invalidate_contents();
    }
}
