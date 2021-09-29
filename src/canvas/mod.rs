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
    // Changes cursor location
    pub fn change(&self, x: f64, y: f64) {
        let canvas = imp::Canvas::from_instance(self);

        // creates lines if is_drawing is true
        if canvas.is_drawing.get() {
            let mut l = canvas.lines.borrow_mut();
            match l.last() {
                Some(p) => {
                    p.borrow_mut().push(imp::point {
                        x: self.inverse_zoom(
                            canvas.zoom_x.get(),
                            canvas.x.get(),
                            canvas.offset_x.get(),
                        ),
                        y: self.inverse_zoom(
                            canvas.zoom_y.get(),
                            canvas.y.get(),
                            canvas.offset_y.get(),
                        ),
                        size: 3.0,
                    });
                }
                None => (),
            }
            println!("len: {}", l.len())
        }

        // offsets lines if is_offsetting is true
        if canvas.is_offsetting.get() {
            canvas
                .offset_x
                .set(x - (canvas.x.get() - canvas.offset_x.get()));
            canvas
                .offset_y
                .set(y - (canvas.y.get() - canvas.offset_y.get()));
            println!(
                "Offset:{}, {}",
                canvas.offset_y.get(),
                canvas.offset_x.get()
            )
        }

        canvas.x.set(x);
        canvas.y.set(y);
        self.invalidate_contents();
    }

    // zoom functions
    pub fn zoom(&self, delta: f64) {
        let canvas = imp::Canvas::from_instance(self);
        canvas.zoom.set(canvas.zoom.get() + delta);
        canvas.zoom_x.set(imp::Canvasimpl::zoom(
            canvas,
            canvas.zoom_x.get(),
            canvas.x.get(),
            canvas.offset_x.get(),
        ));
        canvas.zoom_y.set(canvas.y.get());
        self.invalidate_contents();
    }
    fn inverse_zoom(&self, origin_x: f64, x: f64, offset: f64) -> f64 {
        let canvas = imp::Canvas::from_instance(self);
        return ((x - offset - origin_x) / canvas.zoom.get()) + origin_x;
    }

    //offset manager functions (how points are translated on the canvas)
    pub fn start_offset(&self) {
        let canvas = imp::Canvas::from_instance(self);
        canvas.is_offsetting.set(true);
    }
    pub fn end_offset(&self) {
        let canvas = imp::Canvas::from_instance(self);
        canvas.is_offsetting.set(false);
    }

    // line manager functions
    pub fn start_line(&self, x: f64, y: f64) {
        let canvas = imp::Canvas::from_instance(self);
        canvas.is_drawing.set(true);
        let mut l = canvas.lines.borrow_mut();
        let r = RefCell::new(vec![imp::point {
            x: self.inverse_zoom(canvas.zoom_x.get(), canvas.x.get(), canvas.offset_x.get()),
            y: self.inverse_zoom(canvas.zoom_y.get(), canvas.y.get(), canvas.offset_y.get()),
            size: 3.0,
        }]);
        l.push(r);
        println!("is: {}", canvas.is_drawing.get());
        self.invalidate_contents();
    }
    pub fn end_line(&self, x: f64, y: f64) {
        let canvas = imp::Canvas::from_instance(self);
        canvas.is_drawing.set(false);
        println!("is: {}", canvas.is_drawing.get());
        self.invalidate_contents();
    }
}
