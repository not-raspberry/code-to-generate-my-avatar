/// Module drawing a gradient following the Hilbert curve path.
use image::{DynamicImage, GenericImage, Rgba};
use hilbert;

/// Blend c1 with c2, with ratio.
/// Ratio [0, 1] is the amount of the first color to apply. The rest will
/// be c2.
fn blend(c1: Rgba<u8>, c2: Rgba<u8>, ratio: f32) -> Rgba<u8> {
    assert!(0.0 <= ratio);
    assert!(ratio <= 1.0);
    let c1_strength = ratio;
    let c2_strength = 1.0 - c1_strength;

    let avg = |a, b| (a as f32 * c1_strength  + b as f32 * c2_strength) as u8;

    Rgba(
        [avg(c1.data[0], c2.data[0]),
         avg(c1.data[1], c2.data[1]),
         avg(c1.data[2], c2.data[2]),
         avg(c1.data[3], c2.data[3])]
    )
}

pub fn hilbert_pixels() -> DynamicImage {
    let order: u32 = 8;
    let size: u32  = 2u32.pow(order);
    let pixels_count = size * size;
    let mut image = DynamicImage::new_rgb8(size, size);
    let (initial_color, final_color) = (Rgba([0xe3, 0x0b, 0x5d, 0xff]),
                                        Rgba([0x0, 0x0, 0x0, 0x0]));

    for (index, position) in hilbert::HilbertCurvePixels::new(order).enumerate() {
        let blend_ratio = index as f32 / pixels_count as f32;
        let color = blend(initial_color, final_color, blend_ratio);
        image.put_pixel(position.x, position.y, color);
    }
    image
}
