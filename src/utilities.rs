use crate::constants::WORLD_CELL_SIZE;

pub fn unit_to_pixel(unit: i16) -> i16 {
    return unit * WORLD_CELL_SIZE;
}

pub fn coord_to_pixel(x: i16, y: i16) -> (i16, i16) {
    return (unit_to_pixel(x), unit_to_pixel(y));
}
