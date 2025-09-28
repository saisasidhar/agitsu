use ndarray::{Array3, ArrayView3, Axis};

pub fn srgb_to_linear(x: &ArrayView3<f32>) -> Array3<f32> {
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

pub fn linear_to_srgb(x: &ArrayView3<f32>) -> Array3<f32> {
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

pub fn invert(lin: &ArrayView3<f32>, film_base_estimate: [f32; 3]) -> Array3<f32> {
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

pub fn invert_color_negative(negative_image: &ArrayView3<f32>) -> Array3<f32> {
    let film_base_estimate: [f32;3] = [0.6454267, 0.4690254, 0.35811886];
    let linear_negative_image: Array3<f32> = srgb_to_linear(&negative_image.view());
    let linear_positive_image: Array3<f32> = invert(&linear_negative_image.view(), film_base_estimate);
    linear_positive_image.map(|&x| (x.clamp(0.0, 1.0) * 255.0).round() as u8);
    let positive_image: Array3<f32> = linear_to_srgb(&linear_positive_image.view());

    positive_image
}
