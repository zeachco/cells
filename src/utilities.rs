use crate::constants::UNIT_PIXEL_SIZE;

pub fn unit_to_pixel(unit: i32) -> f32 {
    return unit as f32 * UNIT_PIXEL_SIZE as f32;
}

pub fn coord_to_pixel(x: i32, y: i32) -> (f32, f32) {
    return (unit_to_pixel(x), unit_to_pixel(y));
}
