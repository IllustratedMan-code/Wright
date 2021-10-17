use crate::bar::Bar;
use crate::canvas::Canvas;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct Note {
    /// Reference to the canvas widget.
    ///
    /// In our case it's a text label for the button. Since this example only uses a
    /// `gtk4::Label`, the type could've been `Option<gtk::Label>`. However, a real button might
    /// switch between a label widget and an icon widget, and in general your widget can contain
    /// arbitrary canvasren. Thus we used `Option<gtk4::Widget>` to show how to handle any widget
    /// and to make the example easier to tweak and play with.
    ///
    /// Widgets automatically store strong references to their canvasren, added in `set_parent()`
    /// and removed in `unparent()`. Therefore, this field could be a `WeakRef<gtk4::Widget>`.
    /// Using a strong reference is just a little clearer.
    canvas: RefCell<Option<gtk4::Picture>>,
    button: RefCell<Option<gtk4::Button>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Note {
    const NAME: &'static str = "Note";
    type Type = super::Note;
    type ParentType = gtk4::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_css_name("note");
    }
}

impl ObjectImpl for Note {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        let layout = gtk4::BoxLayout::new(gtk4::Orientation::Vertical);
        obj.set_layout_manager(Some(&layout));

        let note_canvas = gtk4::Picture::new();
        let canvas = Canvas::new();
        note_canvas.set_paintable(Some(&canvas));

        let mouse = gtk4::EventControllerMotion::new();
        let left_click = gtk4::GestureClick::builder().button(1).build();
        let right_click = gtk4::GestureClick::builder().button(3).build();
        let wheel = gtk4::EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);

        mouse.connect_motion(glib::clone!(@weak canvas => move |_, x, y| {
            let main_context = glib::MainContext::default();
            main_context.spawn_local(glib::clone!(@weak canvas => async move {
                canvas.change(x, y).await;
        }))}));

        left_click.connect_pressed(
            glib::clone!(@weak canvas  => move |_, n, x, y| canvas.start_line(x , y)),
        );
        left_click.connect_released(
            glib::clone!(@weak canvas  => move |_, n, x, y| canvas.end_line(x , y)),
        );

        right_click.connect_pressed(
            glib::clone!(@weak canvas  => move |_, _n, _x, _y| canvas.start_offset()),
        );
        right_click.connect_released(
            glib::clone!(@weak canvas  => move |_, _n, _x, _y| canvas.end_offset()),
        );

        wheel.connect_scroll(move |event_controller, x, y| {
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
            let main_context = glib::MainContext::default();
            // The main loop executes the asynchronous block
            main_context.spawn_local(glib::clone!(@weak canvas => async move {
                // Deactivate the button until the operation is done
                canvas.zoom(y/10.0).await;
            }));
            gtk4::Inhibit(false)
        });

        note_canvas.add_controller(&mouse);
        note_canvas.add_controller(&left_click);
        note_canvas.add_controller(&right_click);
        note_canvas.add_controller(&wheel);
        note_canvas.set_vexpand(true);
        note_canvas.set_hexpand(true);
        note_canvas.set_cursor_from_name(Some("crosshair"));

        let button = gtk4::Button::new();

        button.set_parent(obj);
        note_canvas.set_parent(obj);

        *self.canvas.borrow_mut() = Some(note_canvas);
        *self.button.borrow_mut() = Some(button);

        obj.add_css_class("note");
    }

    fn dispose(&self, _obj: &Self::Type) {
        // Canvas widgets need to be manually unparented in `dispose()`.
        if let Some(canvas) = self.canvas.borrow_mut().take() {
            canvas.unparent();
        }
        if let Some(button) = self.button.borrow_mut().take() {
            button.unparent();
        }
    }
}

impl WidgetImpl for Note {}
