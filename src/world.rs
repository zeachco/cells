use crate::cell::Cell;

pub const WORLD_SIZE: u32 = 100;
pub const WORLD_CELL_SIZE: u32 = 5;
pub const WORLD_CELL_COUNT: u32 = WORLD_SIZE / WORLD_CELL_SIZE;

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

pub fn generate_world() {
    let mut world_matrix: Vec<Vec<WorldObject>> = Vec::new();

    for x in 0..WORLD_CELL_COUNT {
        let mut world_row: Vec<WorldObject> = Vec::new();
        for y in 0..WORLD_CELL_COUNT {
            let world_object = WorldObject {
                cells: Vec::new(),
                properties: WorldProperties {
                    enery_diffusion: 0.1,
                    energy: 0.5,
                },
            };
            world_row.push(world_object);
        }
        world_matrix.push(world_row);
    }

    // TODO remove debug
    println!("{:?}", world_matrix[0][0].properties.energy)
}
