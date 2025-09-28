use gst::glib;
use gst::prelude::*;
use gstreamer_base::{BaseTransform, gst};
use gstreamer_video::VideoFilter;

mod imp;

glib::wrapper! {
    pub struct AgitsuFilter(ObjectSubclass<imp::AgitsuFilter>) @extends VideoFilter, BaseTransform, gst::Element, gst::Object;
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "agitsu",
        gst::Rank::NONE,
        AgitsuFilter::static_type(),
    )
}
