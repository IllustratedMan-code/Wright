mod bar;
mod canvas;
mod color_button;
mod note;
use gtk4::prelude::*;
use note::Note;

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
    println!("{}", window.type_());
    window.set_title(Some("Wright"));
    window.set_default_size(500, 500);
    let note = Note::new();
    window.set_child(Some(&note));
    window.show();
}
