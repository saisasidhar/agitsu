use show_image::{ImageView, ImageInfo, create_window};
use image::{ImageReader, RgbImage};
use ndarray::{Array3, ArrayView3, Axis};

fn srgb_to_linear(x: &ArrayView3<f32>) -> Array3<f32> {
    const A: f32 = 0.055;
    const GAMMA: f32 = 1.8;

    x.map(|&val| {
        if val <= 0.04045 {
            val / 12.92
        } else {
            ((val + A) / (1.0 + A)).powf(GAMMA)
        }
    })
}

fn linear_to_srgb(x: &ArrayView3<f32>) -> Array3<f32> {
    const A: f32 = 0.055;
    const GAMMA: f32 = 1.8;

    x.map(|&val| {
        if val <= 0.0031308 {
            val * 12.92
        } else {
            (1.0 + A) * val.powf(1.0/GAMMA) - A
        }
    })
}

fn invert(lin: &ArrayView3<f32>, film_base_estimate: [f32; 3]) -> Array3<f32> {
    let shape = lin.raw_dim();
    let mut inv = Array3::<f32>::zeros(shape);

    for ch in 0..3 {
        let channel_film_base = film_base_estimate[ch];
        let channel = lin.index_axis(Axis(2), ch);
        let mut inv_channel = inv.index_axis_mut(Axis(2), ch);

        inv_channel.assign(&channel.map(|&val| ((channel_film_base - val) / channel_film_base).clamp(0.0, 1.0)));
    }

    inv
}

fn invert_color_negative(negative_image: &ArrayView3<f32>) -> Array3<f32> {
    let film_base_estimate: [f32;3] = [0.6454267, 0.4690254, 0.35811886];
    let linear_negative_image: Array3<f32> = srgb_to_linear(&negative_image.view());
    let linear_positive_image: Array3<f32> = invert(&linear_negative_image.view(), film_base_estimate);
    linear_positive_image.map(|&x| (x.clamp(0.0, 1.0) * 255.0).round() as u8);
    let positive_image: Array3<f32> = linear_to_srgb(&linear_positive_image.view());

    positive_image
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let negative_scan: RgbImage = ImageReader::open("negative.jpg")?.decode()?.to_rgb8();
    let (width, height) = negative_scan.dimensions();

    let negative_image: Array3<f32> = Array3::from_shape_fn(
        (height as usize, width as usize, 3),
        |(y, x, c)| negative_scan.as_raw()[(y * width as usize + x) * 3 + c] as f32 / 255.0,
    );

    let positive_image: Array3<f32>  = invert_color_negative(&negative_image.view());
    let positive_result: Array3<u8> = positive_image.map(|&x| (x.clamp(0.0, 1.0) * 255.0).round() as u8);

    let image_buffer: &[u8] = positive_result.as_slice().unwrap();
    let image_view: ImageView = ImageView::new(
        ImageInfo::rgb8(width, height),
        &image_buffer,
    );
    let window = create_window("Image Viewer", Default::default())?;
    window.set_image("image-001", image_view)?;
    for _ in window.event_channel()? {
    }
    Ok(())
}