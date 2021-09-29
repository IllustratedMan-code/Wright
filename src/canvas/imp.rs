use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gdk, glib, graphene, gsk};
use std::cell::{Cell, RefCell};
#[derive(Default, Copy, Clone)]
pub struct point {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

impl PartialEq for point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.size == other.size
    }
}

#[derive(Default)]
pub struct Canvas {
    pub x: Cell<f64>,
    pub y: Cell<f64>,
    pub lines: RefCell<Vec<RefCell<Vec<point>>>>,
    pub line_offset_x: Cell<f64>,
    pub line_offset_y: Cell<f64>,
    pub is_drawing: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for Canvas {
    const NAME: &'static str = "Canvas";
    type Type = super::Canvas;
    type ParentType = glib::Object;
    type Interfaces = (gdk::Paintable,);
}

impl ObjectImpl for Canvas {}

impl PaintableImpl for Canvas {
    fn snapshot(&self, _paintable: &Self::Type, snapshot: &gdk::Snapshot, width: f64, height: f64) {
        let snapshot = snapshot.downcast_ref::<gtk4::Snapshot>().unwrap();
        let context = snapshot.append_cairo(&graphene::Rect::new(
            0_f32,
            0_f32,
            width as f32,
            height as f32,
        ));
        match context {
            Some(c) => {
                // cursor
                c.set_source_rgb(0.3, 0.3, 0.3);
                c.arc(self.x.get(), self.y.get(), 30.0, 0.0, 3.14 * 2.);

                // lines
                for line in self.lines.borrow().iter() {
                    let points = line.borrow();
                    for point in points.iter() {
                        match points.first() {
                            Some(p) => {
                                if point == p {
                                    c.move_to(point.x, point.y)
                                } else {
                                    c.line_to(point.x, point.y);
                                    c.set_line_width(point.size);
                                }
                            }
                            None => (),
                        }
                    }
                }

                // stroke test
                c.stroke().expect("Invalid cairo surface state");
            }
            None => eprintln!("Context not created"),
        }
    }
}

pub trait Canvasimpl {
    fn change(&self, x: f64, y: f64);
}

impl Canvasimpl for Canvas {
    fn change(&self, x: f64, y: f64) {
        self.x.set(x);
        self.y.set(y);
    }
}
