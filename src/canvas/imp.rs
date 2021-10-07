use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{cairo, gdk, glib, graphene, gsk};
use std::cell::{Cell, RefCell};
#[derive(Default)]
pub struct point {
    pub x: f64,
    pub y: f64,
    pub zoom_x: f64,
    pub zoom_y: f64,
    pub size: f64,
}

impl Copy for point {}
impl Clone for point {
    fn clone(&self) -> point {
        *self
    }
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
    pub zoom_lines: RefCell<Vec<Vec<Cell<point>>>>,
    pub offset_x: Cell<f64>,
    pub offset_y: Cell<f64>,
    pub is_drawing: Cell<bool>,
    pub is_offsetting: Cell<bool>,
    pub zoom: Cell<f64>,
    pub zoom_x: Cell<f64>,
    pub zoom_y: Cell<f64>,
}

#[glib::object_subclass]
impl ObjectSubclass for Canvas {
    const NAME: &'static str = "Canvas";
    type Type = super::Canvas;
    type ParentType = glib::Object;
    type Interfaces = (gdk::Paintable,);
}

impl ObjectImpl for Canvas {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        self.zoom.set(1.0);
    }
}

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
                c.set_source_rgb(0.8, 0.3, 0.3);
                c.arc(self.x.get(), self.y.get(), 30.0, 0.0, 3.14 * 2.);
                c.stroke().expect("Invalid cairo surface state");

                // lines
                c.set_source_rgb(0.3, 0.3, 0.3);
                for line in self.lines.borrow().iter() {
                    c.set_source_rgb(0.3, 0.3, 0.3);
                    self.draw_curve(&c, &line.borrow(), 0.333);
                }

                // stroke test
            }
            None => eprintln!("Context not created"),
        }
    }
}

pub trait Canvasimpl {
    fn change(&self, x: f64, y: f64);
    fn zoom(&self, origin_x: f64, x: f64) -> f64;
    fn draw_curve(&self, c: &cairo::Context, line: &Vec<point>, ratio: f64);
}

impl Canvasimpl for Canvas {
    fn change(&self, x: f64, y: f64) {
        self.x.set(x);
        self.y.set(y);
    }
    fn zoom(&self, origin_x: f64, x: f64) -> f64 {
        return (x - origin_x) * self.zoom.get() + origin_x;
    }
    fn draw_curve(&self, c: &cairo::Context, line: &Vec<point>, ratio: f64) {
        let x1 = line[0].zoom_x + self.offset_x.get();
        let y1 = line[0].zoom_y + self.offset_y.get();
        c.move_to(x1, y1);
        for point in 1..line.len() + 5 {
            let p = |point| -> usize {
                if point < 4 {
                    return 0;
                } else if point > line.len() - 1 {
                    return line.len() - 1;
                } else {
                    return point;
                }
            };
            let interpolate_x = |x1: f64, x2: f64, ratio: f64| -> f64 {
                return x1 * ratio + x2 * (1.0 - ratio);
            };
            let interpolate_y = |y1: f64, y2: f64, ratio: f64| -> f64 {
                return y1 * ratio + y2 * (1.0 - ratio);
            };
            let pre_x2 = interpolate_x(line[p(point)].zoom_x, line[p(point + 1)].zoom_x, ratio);
            let pre_x3 = interpolate_x(line[p(point + 1)].zoom_x, line[p(point)].zoom_x, ratio);
            let pre_x4 = interpolate_x(line[p(point + 1)].zoom_x, line[p(point + 2)].zoom_x, ratio);
            let x2 = pre_x2 + self.offset_x.get();
            let x3 = pre_x3 + self.offset_x.get();
            let x4 = interpolate_x(pre_x4, pre_x3, 0.7) + self.offset_x.get();
            let pre_y2 = interpolate_y(line[p(point)].zoom_y, line[p(point + 1)].zoom_y, ratio);
            let pre_y3 = interpolate_y(line[p(point + 1)].zoom_y, line[p(point)].zoom_y, ratio);
            let pre_y4 = interpolate_y(line[p(point + 1)].zoom_y, line[p(point + 2)].zoom_y, ratio);
            let y2 = pre_y2 + self.offset_y.get();
            let y3 = pre_y3 + self.offset_y.get();
            let y4 = interpolate_y(pre_y4, pre_y3, 0.7) + self.offset_y.get();
            c.curve_to(x2, y2, x3, y3, x4, y4);
        }

        c.stroke().expect("Invalid cairo surface state");
    }
}
