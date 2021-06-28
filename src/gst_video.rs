use gst::prelude::*;
use gstreamer as gst;
use gtk::prelude::*;

pub struct Player {
    pub player: gst::Element,
    pub container: gtk::Widget,
}

impl Player {
    pub fn new(uri: &str) -> Result<Self, glib::BoolError> {
        let _ = gst::init();

        // gtksink creates an OpenGL GTK widget for rendering our video to
        let sink = gst::ElementFactory::make("gtkglsink", None)?;
        let glsinkbin = gst::ElementFactory::make("glsinkbin", None)?;
        glsinkbin.set_property("sink", &sink)?;

        // playbin automatically decodes and renders the video to our gtk sink
        let player = gst::ElementFactory::make("playbin", None)?;
        player.set_property("uri", &glib::Value::from(uri))?;
        player.set_property("video-sink", &glsinkbin)?;

        // Register a signal to listen for events from the player's pipeline bus
        if let Some(bus) = player.get_bus() {
            let player = player.downgrade();
            let _ = bus.add_watch_local(move |_, msg| {
                let player = match player.upgrade() {
                    Some(player) => player,
                    None => return glib::Continue(false),
                };

                match msg.view() {
                    // Loop video on end of stream
                    gst::MessageView::Eos(_) => {
                        let _ = player
                            .seek_simple(gst::SeekFlags::FLUSH, gst::ClockTime::from_seconds(0));
                    }

                    gst::MessageView::Error(err) => {
                        eprintln!(
                            "Gstreamer error from {:?}: {} ({:?})",
                            err.get_src().map(|s| s.get_path_string()),
                            err.get_error(),
                            err.get_debug()
                        );
                    }
                    _ => (),
                }

                glib::Continue(true)
            });
        }

        // Attach the sink widget, and begin playing the video, on widget realize
        let container = (cascade! {
            gtk::Box::new(gtk::Orientation::Vertical, 0);
            ..connect_realize(glib::clone!(@strong player => move |container| {
                let widget = sink.get_property("widget")
                    .unwrap()
                    .get::<gtk::Widget>()
                    .unwrap()
                    .unwrap();
                widget.set_hexpand(false);
                widget.set_halign(gtk::Align::Center);
                widget.set_valign(gtk::Align::Center);
                widget.set_vexpand(false);
                container.add(&widget);
                container.show_all();
                player.set_state(gst::State::Playing).unwrap();
            }));
        })
        .upcast();

        Ok(Self { player, container })
    }
}
