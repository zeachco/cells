use nannou::Draw;

use sha2::{Digest, Sha256};

use crate::{
    cell::Cell,
    constants::{
        MAX_WORLD_COORD, MIN_WORLD_COORD, OUT_OF_BOUND_ENERGY_LOSS, UNIT_PIXEL_SIZE, WORLD_UNITS,
    },
    utilities::coord_to_pixel,
};

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
    properties: WorldProperties,
}

pub fn generate_world() {
    let mut world_matrix: Vec<Vec<WorldObject>> = Vec::new();

    for _x in 0..WORLD_UNITS {
        let mut world_row: Vec<WorldObject> = Vec::new();
        for _y in 0..WORLD_UNITS {
            let world_object = WorldObject {
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

pub fn getWorldRead(cell: &Cell) {
    let x = cell.x;
    let y = cell.y;
}

pub fn draw_tiles(draw: &Draw) {
    let mut x = MIN_WORLD_COORD + 1;
    let mut y = MIN_WORLD_COORD + 1;
    while x < MAX_WORLD_COORD {
        println!("x: {}, y: {}", x, y);
        while y < MAX_WORLD_COORD {
            // let complex_object = vec![1, 2, 3, 4, 5];
            // let mut hasher = Sha256::new();
            // hasher.update(&complex_object);
            // let result = hasher.finalize();
            // let hex_string = hex::encode(result);

            // // Take first 6 characters to form a color code
            // let color_code = &hex_string[0..6];
            // println!("Color Code: #{}", color_code);

            // // Convert to RGB color using `palette`
            // let color: Srgb<u8> = Srgb::new(
            //     u8::from_str_radix(&color_code[0..2], 16).unwrap(),
            //     u8::from_str_radix(&color_code[2..4], 16).unwrap(),
            //     u8::from_str_radix(&color_code[4..6], 16).unwrap(),
            // );

            // println!("RGB Color: {:?}", color);

            let (dx, dy) = coord_to_pixel(x, y);
            let (w, h) = (UNIT_PIXEL_SIZE as f32, UNIT_PIXEL_SIZE as f32);
            draw.rect()
                .color(nannou::color::named::RED)
                .w(w - 2.0)
                .h(h - 2.0)
                .x(dx - w / 2.0 + 1.0)
                .y(dy - h / 2.0 + 1.0);
            y += 1;
        }
        x += 1;
    }
}
