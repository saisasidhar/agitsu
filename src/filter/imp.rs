use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::ObjectSubclass;
use gstreamer_base::gst::{Caps, PadDirection, PadTemplate, PadPresence};
use gstreamer_base::subclass::BaseTransformMode;
use gstreamer_base::subclass::base_transform::BaseTransformImpl;
use gstreamer_base::{BaseTransform, gst};
use gstreamer_video::{VideoFormat};
use std::sync::LazyLock;
use gstreamer_base::subclass::prelude::{ElementImpl, GstObjectImpl};

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

impl ElementImpl for AgitsuFilter {
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

    fn pad_templates() -> &'static [PadTemplate] {
        static PAD_TEMPLATES: LazyLock<Vec<PadTemplate>> = LazyLock::new(|| {
            let caps = gstreamer_video::VideoCapsBuilder::new()
                .format_list([VideoFormat::Rgb])
                .build();
            let src_pad_template = PadTemplate::new(
                "src",
                PadDirection::Src,
                PadPresence::Always,
                &caps,
            )
                .unwrap();

            let caps = gstreamer_video::VideoCapsBuilder::new()
                .format(VideoFormat::Rgb)
                .build();
            let sink_pad_template = PadTemplate::new(
                "sink",
                PadDirection::Sink,
                PadPresence::Always,
                &caps,
            )
                .unwrap();

            vec![src_pad_template, sink_pad_template]
        });

        PAD_TEMPLATES.as_ref()
    }
}

impl GstObjectImpl for AgitsuFilter {}


impl BaseTransformImpl for AgitsuFilter {
    const MODE: BaseTransformMode = BaseTransformMode::AlwaysInPlace;
    const PASSTHROUGH_ON_SAME_CAPS: bool = false;
    const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;

    fn transform_caps(
        &self,
        direction: PadDirection,
        caps: &Caps,
        filter: Option<&Caps>,
    ) -> Option<Caps> {
        let caps = Caps::builder("video/x-raw")
            .field("format", &VideoFormat::Rgb.to_str())
            .build();
        Some(caps)
    }

    fn accept_caps(&self, direction: PadDirection, caps: &Caps) -> bool {
        caps.iter().any(|s| {
            s.name() == "video/x-raw" && s.get::<String>("format").map_or(false, |f| f == "RGB")
        })
    }
}
