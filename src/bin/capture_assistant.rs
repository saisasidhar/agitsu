use gphoto2::file::CameraFile;
use gphoto2::{Context, Result};
use show_image::{ImageInfo, ImageView, create_window};
use std::time::Duration;

#[show_image::main]
fn main() -> Result<()> {
    let window = create_window("Image Viewer", Default::default()).unwrap();

    let camera_context = Context::new()?;
    let camera = camera_context
        .autodetect_camera()
        .wait()
        .expect("Failed to autodetect camera");

    loop {
        let preview_file: CameraFile = match camera.capture_preview().wait() {
            Ok(file) => file,
            Err(error) => panic!("Failed to capture camera preview: {}", error),
        };

        let preview_image = preview_file
            .get_data(&camera_context)
            .wait()
            .expect("Failed to read preview image");

        match image::load_from_memory(&preview_image) {
            Ok(img) => {
                let rgb = img.to_rgb8();
                // 960x640 for Canon EOS RP
                let (width, height) = rgb.dimensions();
                let raw_pixels = rgb.into_raw();
                let image_buffer: &[u8] = raw_pixels.as_slice();
                let image_view: ImageView =
                    ImageView::new(ImageInfo::rgb8(width, height), &image_buffer);
                window.set_image("frame", image_view).unwrap();
            }
            Err(_) => panic!("Failed to load preview image"),
        }
        std::thread::sleep(Duration::from_millis(21));
    }

    Ok(())
}
