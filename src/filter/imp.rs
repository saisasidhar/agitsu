use crate::filter::processing;
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
use ndarray::{Array3, ArrayView3, ArrayViewMut3, Zip};
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
        let film_base: [f32; 3] = [0.6454267, 0.4690254, 0.35811886];
        let info = inframe.info();
        let width = info.width() as usize;
        let height = info.height() as usize;

        let in_data = inframe.plane_data(0).unwrap();
        let out_data = outframe.plane_data_mut(0).unwrap();

        let in_arr: Array3<f32> = Array3::from_shape_fn((height, width, 3), |(y, x, c)| {
            in_data[(y * width + x) * 3 + c] as f32 / 255.0
        });
        let out_arr: Array3<f32> = processing::invert(&in_arr.view(), film_base);

        for (out_px, &val) in out_data.iter_mut().zip(out_arr.iter()) {
            *out_px = (val.clamp(0.0, 1.0) * 255.0).round() as u8;
        }

        Ok(FlowSuccess::Ok)
    }
}
