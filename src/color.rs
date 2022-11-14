use std::io::Write;

use crate::vec3::Color;


fn clamp<T>(x: T, min: T, max: T) -> T where T: PartialOrd {
    if x < min { return min }
    if x > max { return max }
    x
}

pub fn write_color(stream: &mut dyn Write, pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();

    
    let string = format!("{} {} {}\n", 
                         (256.0 * clamp(r, 0.0, 0.999)) as i32, 
                         (256.0 * clamp(g, 0.0, 0.999)) as i32, 
                         (256.0 * clamp(b, 0.0, 0.999)) as i32);

    stream.write_all(string.as_bytes())
        .expect("Cannot write pixel to given stream.");
}
