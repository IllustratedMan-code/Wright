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
                //c.set_source_rgb(0.8, 0.3, 0.3);
                //c.arc(self.x.get(), self.y.get(), 30.0, 0.0, 3.14 * 2.);
                //c.stroke().expect("Invalid cairo surface state");

                // lines
                c.set_source_rgb(0.3, 0.3, 0.3);
                for line in self.lines.borrow().iter() {
                    self.draw_curve(&c, &line.borrow(), 1.0);
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
    fn draw_curve(&self, c: &cairo::Context, line: &Vec<point>, tension: f64);
}

impl Canvasimpl for Canvas {
    fn change(&self, x: f64, y: f64) {
        self.x.set(x);
        self.y.set(y);
    }
    fn zoom(&self, origin_x: f64, x: f64) -> f64 {
        return (x - origin_x) * self.zoom.get() + origin_x;
    }
    fn draw_curve(&self, c: &cairo::Context, line: &Vec<point>, tension: f64) {
        let x1 = line[0].zoom_x + self.offset_x.get();
        let y1 = line[0].zoom_y + self.offset_y.get();
        c.move_to(x1, y1);
        for point in 1..line.len() + 5 {
            let p = |point: i64| -> usize {
                if point < 4 {
                    return 0;
                } else if point > line.len() as i64 - 1 {
                    return line.len() - 1;
                } else {
                    return point as usize;
                }
            };
            let point = point as i64;
            let p0_x = line[p(point - 2)].zoom_x;
            let p0_y = line[p(point - 2)].zoom_y;
            let p1_x = line[p(point - 1)].zoom_x;
            let p1_y = line[p(point - 1)].zoom_y;
            let p2_x = line[p(point)].zoom_x;
            let p2_y = line[p(point)].zoom_y;
            let p3_x = line[p(point + 1)].zoom_x;
            let p3_y = line[p(point + 1)].zoom_y;
            let new_p1_x = self.offset_x.get() + p1_x + (p2_x - p0_x) / (6.0 * tension);
            let new_p1_y = self.offset_y.get() + p1_y + (p2_y - p0_y) / (6.0 * tension);
            let new_p2_x = self.offset_x.get() + p2_x - (p3_x - p1_x) / (6.0 * tension);
            let new_p2_y = self.offset_y.get() + p2_y - (p3_y - p1_y) / (6.0 * tension);
            let new_p3_x = self.offset_x.get() + p2_x;
            let new_p3_y = self.offset_y.get() + p2_y;
            c.set_source_rgb(0.3, 0.3, 0.3);
            c.set_line_width(line[p(point)].size);
            c.curve_to(new_p1_x, new_p1_y, new_p2_x, new_p2_y, new_p3_x, new_p3_y);
        }
        c.stroke_preserve().expect("hi");
        c.stroke().expect("invalid cairo surface state");
    }
}
