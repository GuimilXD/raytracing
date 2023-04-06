use crate::vec3::Color;

fn clamp<T>(x: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn format_pixel_color(pixel_color: Color, samples_per_pixel: i32) -> (u8, u8, u8) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();

    (
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    )
}
