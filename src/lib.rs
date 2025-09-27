use gstreamer::{glib, plugin_define};
use gstreamer::prelude::*;
use gstreamer_base::subclass::prelude::*;


fn plugin_init(plugin: &gstreamer::Plugin) -> Result<(), glib::BoolError> {
    Ok(())
}

plugin_define!(
    agitsu,
    "agitsu, a gstreamer plugin to invert negative frame to positive",
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "\0"),
    "MIT/X11",
    "agitsu",
    "0",
    "https://github.com/saisasidhar/agitsu"
);