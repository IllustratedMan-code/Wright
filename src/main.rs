mod canvas;

use canvas::Canvas;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{EventControllerMotion, GestureClick};

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
    let paintable = Canvas::new();
    let mouse = EventControllerMotion::new();
    let picture = gtk4::Picture::new();
    picture
        .set_property("keep-aspect-ratio", &false)
        .expect("Could not set property");
    picture.set_paintable(Some(&paintable));
    picture.add_controller(&mouse);
    mouse.connect_motion(glib::clone!(@weak paintable => move |_, x, y| {
        &paintable.change(x, y);
    }));
    let gesture = GestureClick::builder().button(0).build();
    picture.add_controller(&gesture);
    gesture.connect_pressed(
        glib::clone!(@weak paintable  => move |_, n, x, y| paintable.start_line(x , y)),
    );
    gesture.connect_released(
        glib::clone!(@weak paintable  => move |_, n, x, y| paintable.end_line(x , y)),
    );

    window.set_child(Some(&picture));
    window.show();
}
