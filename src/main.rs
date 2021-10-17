mod canvas;
mod note;
use note::Note;

use canvas::Canvas;
use glib::MainContext;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{EventControllerMotion, EventControllerScroll, GestureClick, Inhibit};
use std::time::SystemTime;

fn main() {
    let application = gtk4::Application::new(
        Some("com.github.gtk-rs.examples.paintable"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk4::Application) {
    gtk4::gdk::set_allowed_backends("x11");
    let window = gtk4::ApplicationWindow::new(application);
    window.set_title(Some("Custom Paintable"));
    window.set_default_size(500, 500);
    let canvas = Canvas::new();
    let mouse = EventControllerMotion::new();
    let stylus = gtk4::GestureStylus::new();
    let picture = gtk4::Picture::new();
    picture
        .set_property("keep-aspect-ratio", &false)
        .expect("Could not set property");
    picture.set_paintable(Some(&canvas));
    picture.add_controller(&mouse);
    let now = std::cell::Cell::new(SystemTime::now());
    stylus.connect_motion(glib::clone!(@weak canvas => move |event_controller, x, y| {
        let main_context = MainContext::default();
        main_context.spawn_local(glib::clone!(@weak canvas => async move {
            canvas.change(x, y).await;
    }))}));
    stylus.connect_down(glib::clone!(@weak canvas  => move |_, x, y| canvas.start_line(x , y)));
    stylus.connect_up(glib::clone!(@weak canvas  => move |_,  x, y| canvas.end_line(x , y)));
    mouse.connect_motion(glib::clone!(@weak canvas => move |event_controller, x, y| {
    let main_context = MainContext::default();
    main_context.spawn_local(glib::clone!(@weak canvas => async move {
        canvas.change(x, y).await;
    }))}));
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
    gesture3.connect_scroll(move |event_controller, x, y| {
        let event = event_controller.current_event();
        let history: Vec<gtk4::gdk::TimeCoord>;
        match event {
            Some(e) => {
                history = e.history();
                println!("{}", history.len());
                for i in history {
                    println!("asdf{}", i)
                }
            }
            None => (println! {"none"}),
        }
        let main_context = MainContext::default();
        // The main loop executes the asynchronous block
        main_context.spawn_local(glib::clone!(@weak canvas => async move {
            // Deactivate the button until the operation is done
            canvas.zoom(y/10.0).await;
        }));
        Inhibit(false)
    });
    let box_frame = gtk4::Frame::new(Some("Box"));
    let b = gtk4::Button::new();
    let n = gtk4::Button::new();
    let new_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .margin_bottom(0)
        .opacity(1.0)
        .build();
    new_box.set_homogeneous(false);
    box_frame.set_child(Some(&new_box));
    let note = Note::new();
    window.set_child(Some(&note));
    picture.set_valign(gtk4::Align::Fill);
    new_box.append(&picture);
    println!("{}", picture.valign());
    picture.set_vexpand(true);
    new_box.append(&b);
    picture.set_cursor_from_name(Some("crosshair"));
    window.show();
}
