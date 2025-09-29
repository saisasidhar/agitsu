use image::{ImageReader, RgbImage};
use ndarray::Array3;
use show_image::{ImageInfo, ImageView, create_window};

use agitsu::filter::processing;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let film_base: [f32; 3] = [0.6454267, 0.4690254, 0.35811886];
    let neg_scan: RgbImage = ImageReader::open("negative.jpg")?.decode()?.to_rgb8();
    let (width, height) = neg_scan.dimensions();

    let neg_image: Array3<f32> =
        Array3::from_shape_fn((height as usize, width as usize, 3), |(y, x, c)| {
            neg_scan.as_raw()[(y * width as usize + x) * 3 + c] as f32 / 255.0
        });

    let pos_image: Array3<f32> = processing::invert_color_negative(&neg_image.view(), film_base);
    let pos_preview: Array3<u8> = pos_image.map(|&x| (x.clamp(0.0, 1.0) * 255.0).round() as u8);

    let image_buffer: &[u8] = pos_preview.as_slice().unwrap();
    let image_view: ImageView = ImageView::new(ImageInfo::rgb8(width, height), &image_buffer);
    let window = create_window("Image Viewer", Default::default())?;
    window.set_image("image-001", image_view)?;
    for _ in window.event_channel()? {}
    Ok(())
}
