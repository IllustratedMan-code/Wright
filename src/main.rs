mod canvas;

use canvas::Canvas;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{EventControllerMotion, EventControllerScroll, GestureClick, Inhibit};

fn main() {
    let application = gtk4::Application::new(
        Some("com.github.gtk-rs.examples.paintable"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
    window.set_title(Some("Custom Paintable"));
    window.set_default_size(500, 500);
    let canvas = Canvas::new();
    let mouse = EventControllerMotion::new();
    let picture = gtk4::Picture::new();
    picture
        .set_property("keep-aspect-ratio", &false)
        .expect("Could not set property");
    picture.set_paintable(Some(&canvas));
    picture.add_controller(&mouse);
    mouse.connect_motion(glib::clone!(@weak canvas => move |_, x, y| {
        &canvas.change(x, y);
    }));
    let gesture = GestureClick::builder().button(1).build();
    let gesture2 = GestureClick::builder().button(3).build();
    let gesture3 = EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
    picture.add_controller(&gesture);
    picture.add_controller(&gesture2);
    picture.add_controller(&gesture3);
    gesture
        .connect_pressed(glib::clone!(@weak canvas  => move |_, n, x, y| canvas.start_line(x , y)));
    gesture
        .connect_released(glib::clone!(@weak canvas  => move |_, n, x, y| canvas.end_line(x , y)));
    gesture2
        .connect_pressed(glib::clone!(@weak canvas  => move |_, n, x, y| canvas.start_offset()));
    gesture2.connect_released(glib::clone!(@weak canvas  => move |_, n, x, y| canvas.end_offset()));
    gesture3.connect_scroll(move |_, x, y| {
        canvas.zoom(y / 10.0);
        Inhibit(false)
    });

    window.set_child(Some(&picture));
    window.show();
}
