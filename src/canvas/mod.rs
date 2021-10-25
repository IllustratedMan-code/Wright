mod imp;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gdk, glib};
use std::cell::RefCell;

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
    pub async fn change(&self, x: f64, y: f64) {
        let canvas = imp::Canvas::from_instance(self);

        // creates lines if is_drawing is true
        if canvas.is_drawing.get() {
            let l = canvas.lines.borrow_mut();
            match l.last() {
                Some(p) => {
                    p.borrow_mut().push(imp::point {
                        x: canvas.x.get() - canvas.offset_x.get(),
                        y: canvas.y.get() - canvas.offset_y.get(),
                        zoom_x: canvas.x.get() - canvas.offset_x.get(),
                        zoom_y: canvas.y.get() - canvas.offset_y.get(),
                        size: 3.0,
                    });
                }
                None => (),
            }
        }

        // offsets lines if is_offsetting is true
        if canvas.is_offsetting.get() {
            canvas
                .offset_x
                .set(x - (canvas.x.get() - canvas.offset_x.get()));
            canvas
                .offset_y
                .set(y - (canvas.y.get() - canvas.offset_y.get()));
        }

        canvas.x.set(x);
        canvas.y.set(y);
        self.invalidate_contents();
    }

    // zoom functions
    pub async fn zoom(&self, delta: f64) {
        let canvas = imp::Canvas::from_instance(self);
        canvas.zoom_x.set(imp::Canvasimpl::zoom(
            canvas,
            canvas.zoom_x.get(),
            canvas.x.get(),
        ));
        canvas.zoom_y.set(imp::Canvasimpl::zoom(
            canvas,
            canvas.zoom_y.get(),
            canvas.y.get(),
        ));
        canvas.zoom.set(delta);
        for line in canvas.lines.borrow_mut().iter() {
            let mut l = line.borrow_mut();
            for point in 0..l.len() {
                l[point] = imp::point {
                    x: l[point].x,
                    y: l[point].y,
                    zoom_x: imp::Canvasimpl::zoom(
                        canvas,
                        l[point].zoom_x,
                        canvas.x.get() - canvas.offset_x.get(),
                    ),
                    zoom_y: imp::Canvasimpl::zoom(
                        canvas,
                        l[point].zoom_y,
                        canvas.y.get() - canvas.offset_y.get(),
                    ),
                    size: l[point].size * (1.0 - delta),
                }
            }
        }

        self.invalidate_contents();
    }
    //offset manager functions (how points are translated on the canvas)
    pub fn start_offset(&self) {
        let canvas = imp::Canvas::from_instance(self);

        if !canvas.is_offsetting.get() {
            canvas.is_offsetting.set(true);
        } else {
            canvas.is_offsetting.set(false);
        }
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
            x: canvas.x.get() - canvas.offset_x.get(),
            y: canvas.y.get() - canvas.offset_y.get(),
            zoom_x: canvas.x.get() - canvas.offset_x.get(),
            zoom_y: canvas.y.get() - canvas.offset_y.get(),
            size: 3.0,
        }]);
        l.push(r);
        self.invalidate_contents();
    }
    pub fn end_line(&self, x: f64, y: f64) {
        let canvas = imp::Canvas::from_instance(self);
        canvas.is_drawing.set(false);
        self.invalidate_contents();
    }
}
