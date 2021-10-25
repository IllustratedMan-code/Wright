use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ColorButton {
    pub color: RefCell<gdk::RGBA>,
}
impl Default for ColorButton {
    fn default() -> Self {
        Self {
            color: RefCell::new(gdk::RGBA::black()),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for ColorButton {
    const NAME: &'static str = "ColorButton";
    type Type = super::ColorButton;
    type ParentType = gtk4::Widget;

    fn class_init(klass: &mut Self::Class) {
        // The layout manager determines how child widgets are laid out.
        klass.set_layout_manager_type::<gtk4::BinLayout>();

        // Make it look like a GTK4 button.
        klass.set_css_name("button");
    }
}

impl ObjectImpl for ColorButton {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.add_css_class("button");
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(include_bytes!("style.css"));
        obj.style_context()
            .add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        // Connect a gesture to handle clicks.
        // can't fix transient window bug
        let window = glib::types::Type::from_name("GTK_TYPE_WINDOW");
        let parent = obj.root();
        let gesture = gtk4::GestureClick::new();
        gesture.connect_released(move |gesture, _, _, _| {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            let main_context = glib::MainContext::default();
            main_context.spawn_local(
                glib::clone!(@strong provider => async move {
                    let color_chooser = gtk4::ColorChooserDialogBuilder::new().build();
                    color_chooser.run_future().await;
                    color_chooser.close();
                    let rgba = color_chooser.rgba();
                    let color = format!("button {{background-color: rgba({r},{g},{b},{a});}}",r=rgba.red*255.0, g=rgba.green*255.0, b=rgba.blue*255.0, a=rgba.alpha*255.0) ;
                    println!("{}", color);
                    provider.load_from_data(color.as_bytes());
                }),
            );
        });
        obj.add_controller(&gesture);
    }

    fn dispose(&self, _obj: &Self::Type) {
        // Child widgets need to be manually unparented in `dispose()`.
    }
}

impl WidgetImpl for ColorButton {}
