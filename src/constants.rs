pub const WORLD_UNITS: u16 = 100;
pub const UNIT_PIXEL_SIZE: u16 = 10;
// pub const WORLD_UNITS: u16 = 30;
// pub const UNIT_PIXEL_SIZE: u16 = 20;
pub const OUT_OF_BOUND_ENERGY_LOSS: f32 = 5.0;
pub const CELL_MAX_ENERGY: f32 = 1000.0;
pub const CELL_MIN_ENERGY_TO_FUNCTION: f32 = 100.0;

// computed constants
pub const WORLD_PIXEL_SIZE: u32 = WORLD_UNITS as u32 * UNIT_PIXEL_SIZE as u32;
pub const MIN_WORLD_COORD: i32 = (WORLD_UNITS as f32 * -0.5) as i32;
pub const MAX_WORLD_COORD: i32 = (WORLD_UNITS as f32 * 0.5) as i32;
