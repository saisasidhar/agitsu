use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::ObjectSubclass;
use gstreamer_base::gst::{
    BufferRef, Caps, FlowError, FlowSuccess, PadDirection, PadPresence, PadTemplate,
};
use gstreamer_base::subclass::BaseTransformMode;
use gstreamer_base::subclass::base_transform::BaseTransformImpl;
use gstreamer_base::subclass::prelude::{ElementImpl, GstObjectImpl};
use gstreamer_base::{BaseTransform, gst};
use gstreamer_video::subclass::prelude::VideoFilterImpl;
use gstreamer_video::{VideoFilter, VideoFormat, VideoFrameExt, VideoFrameRef};
use ndarray::{ArrayView3, ArrayViewMut3, Zip};
use std::sync::LazyLock;

#[derive(Default)]
pub struct AgitsuFilter {}

impl AgitsuFilter {}

#[glib::object_subclass]
impl ObjectSubclass for AgitsuFilter {
    const NAME: &'static str = "AgitsuFilter";
    type Type = super::AgitsuFilter;
    type ParentType = VideoFilter;
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
            let src_pad_template =
                PadTemplate::new("src", PadDirection::Src, PadPresence::Always, &caps).unwrap();

            let caps = gstreamer_video::VideoCapsBuilder::new()
                .format(VideoFormat::Rgb)
                .build();
            let sink_pad_template =
                PadTemplate::new("sink", PadDirection::Sink, PadPresence::Always, &caps).unwrap();

            vec![src_pad_template, sink_pad_template]
        });

        PAD_TEMPLATES.as_ref()
    }
}

impl GstObjectImpl for AgitsuFilter {}

impl BaseTransformImpl for AgitsuFilter {
    const MODE: BaseTransformMode = BaseTransformMode::NeverInPlace;
    const PASSTHROUGH_ON_SAME_CAPS: bool = false;
    const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;
}

impl VideoFilterImpl for AgitsuFilter {
    fn transform_frame(
        &self,
        inframe: &VideoFrameRef<&BufferRef>,
        outframe: &mut VideoFrameRef<&mut BufferRef>,
    ) -> Result<FlowSuccess, FlowError> {
        let info = inframe.info();
        let width = info.width() as usize;
        let height = info.height() as usize;

        let in_data = inframe.plane_data(0).unwrap();
        let out_data = outframe.plane_data_mut(0).unwrap();

        let in_arr: ArrayView3<u8> =
            ArrayView3::from_shape((height, width, 3), in_data).map_err(|_| FlowError::Error)?;
        let mut out_arr: ArrayViewMut3<u8> =
            ArrayViewMut3::from_shape((height, width, 3), out_data)
                .map_err(|_| FlowError::Error)?;

        Zip::from(&mut out_arr)
            .and(&in_arr)
            .for_each(|out_px, &in_px| {
                *out_px = 255u8.saturating_sub(in_px);
            });

        Ok(FlowSuccess::Ok)
    }
}
