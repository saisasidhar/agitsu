use gstreamer::prelude::*;
use gstreamer_base::gst;
use gstreamer_base::subclass::prelude::*;

pub mod filter;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    filter::register(plugin)?;
    Ok(())
}

gst::plugin_define!(
    agitsu,
    "agitsu, a gstreamer plugin to invert negative frame to positive",
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "\0"),
    "MIT/X11",
    "agitsu",
    "0",
    "https://github.com/saisasidhar/agitsu"
);
