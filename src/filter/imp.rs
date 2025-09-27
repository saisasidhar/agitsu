use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::ObjectSubclass;
use gstreamer::subclass::prelude::{ElementImpl, GstObjectImpl};
use gstreamer_base::subclass::BaseTransformMode;
use gstreamer_base::subclass::base_transform::BaseTransformImpl;
use gstreamer_base::{BaseTransform, gst};
use std::sync::LazyLock;

#[derive(Default)]
pub struct AgitsuFilter {}

impl AgitsuFilter {}

#[glib::object_subclass]
impl ObjectSubclass for AgitsuFilter {
    const NAME: &'static str = "AgitsuFilter";
    type Type = super::AgitsuFilter;
    type ParentType = BaseTransform;
}

impl ObjectImpl for AgitsuFilter {}

impl gstreamer_base::subclass::prelude::ElementImpl for AgitsuFilter {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: LazyLock<gst::subclass::ElementMetadata> = LazyLock::new(|| {
            gst::subclass::ElementMetadata::new(
                "Agitsu Filter",
                "Filter/Effect/Converter/Video",
                "Converts an RGB frame of film negative to positive",
                "Sai Sasidhar Maddali",
            )
        });

        Some(&*ELEMENT_METADATA)
    }
}

impl gstreamer_base::subclass::prelude::GstObjectImpl for AgitsuFilter {}

impl BaseTransformImpl for AgitsuFilter {
    const MODE: BaseTransformMode = BaseTransformMode::AlwaysInPlace;
    const PASSTHROUGH_ON_SAME_CAPS: bool = false;
    const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;
}
