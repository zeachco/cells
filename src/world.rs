pub const WORLD_SIZE: u32 = 100;
pub const WORLD_CELL_SIZE: u32 = 10;
pub const WORLD_CELL_COUNT: u32 = WORLD_SIZE / WORLD_CELL_SIZE;

pub struct Cell {
    id: u32,
    // other properties
}

pub struct WorldProperties {
    /**
     * define at which rate the energy is diffused to the direct neighbors.
     */
    enery_diffusion: f32,
    /**
     * Available enery in this part of the world
     * motor functions will consume energy relative to availability
     * everything over 0.3 is wasted
     * everything over 0.4 cause mutation exponentially up to 1.0
     * everything under 0.2 exponentially cause the cell to lose energy up to 0.0 which causes a full stop
     */
    energy: f32,
}

pub struct WorldObject {
    cells: Vec<Cell>,
    properties: WorldProperties,
}
