use rand::Rng;

use crate::{cell::Cell, constants::CELL_MIN_ENERGY_TO_FUNCTION};

pub fn take_decision(cell: &mut Cell) {
    if cell.energy <= CELL_MIN_ENERGY_TO_FUNCTION {
        return;
    }
    match rand::thread_rng().gen_range(0..6) {
        0 => {
            cell.energy -= 1.0;
            cell.x += 1;
        }
        1 => {
            cell.energy -= 1.0;
            cell.x -= 1;
        }
        2 => {
            cell.energy -= 1.0;
            cell.y += 1;
        }
        3 => {
            cell.energy -= 1.0;
            cell.y -= 1;
        }
        4 => {
            // do nothing
        }
        // take energy
        5 => {
            // cell.energy-=1.0;
            // cell.energy += 1.0;
        }
        // give energy
        6 => {
            // cell.energy-=1.0;
            // cell.energy += 1.0;
        }
        _ => {}
    };
}

/*
 * Sensors (5):
 * level of danger: -1.0..1.0
 *  negative means incentive to move away
 *  positive means incentive to move towards
 * tile color (other cell or energy level)
 * one for each tile around the cell and the cell itself
 */

/*
 * Motors (10):
 * move up
 * move down
 * move left
 * move right
 * steal energy
 * give energy
 */
