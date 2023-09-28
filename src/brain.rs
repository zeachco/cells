use rand::Rng;

use crate::{action::CellAction, cell::Cell, constants::CELL_MIN_ENERGY_TO_FUNCTION};

struct Neurone {
    weight: f32,
    bias: f32,
    activation_fn_index: u8,
}

impl Neurone {
    fn new() -> Neurone {
        Neurone {
            weight: rand::thread_rng().gen_range(-1.0..1.0),
            bias: rand::thread_rng().gen_range(-1.0..1.0),
            activation_fn_index: rand::thread_rng().gen_range(0..3),
        }
    }

    fn receive_input(&self, input: f32) -> f32 {
        match self.activation_fn_index {
            0 => {
                // sigmoid
                1.0 / (1.0 + (-input).exp())
            }
            1 => {
                // tanh
                input.tanh()
            }
            2 => {
                // relu
                if input > 0.0 {
                    input
                } else {
                    0.0
                }
            }
            _ => {
                // sigmoid
                1.0 / (1.0 + (-input).exp())
            }
        }
    }
}

pub struct Brain {
    neurones: Vec<Neurone>,
    neural_network: nalgebra::DMatrix<f32>,
}

impl Brain {
    pub fn new() -> Brain {
        let initial_values: Vec<f32> = (1..=12)
            .map(|_| rand::thread_rng().gen_range(-1.0..1.0))
            .collect();
        let neural_network = nalgebra::DMatrix::from_row_slice(4, 3, &initial_values);
        Brain {
            neurones: vec![],
            neural_network,
        }
    }

    // receive inputs and output a decision
    pub fn handle_inputs(&self, _cell: &Cell) -> Vec<bool> {
        // feed input to first layer
        // let mut layer1: Vec<f32> = vec![];
        // for i in 0..3 {
        //     let mut neurone = &self.neurones[i];
        //     layer1.push(neurone.compute(cell.x as f32));
        //     layer1.push(neurone.compute(cell.y as f32));
        //     layer1.push(neurone.compute(cell.energy));
        // }
        return vec![true, false, true, false, true, false];
    }
}

pub fn take_decision(cell: &mut Cell) {
    if cell.energy <= CELL_MIN_ENERGY_TO_FUNCTION {
        return;
    }

    // cell.x += cell.vx;
    // cell.y += cell.vy;

    // return;

    // let actions () = cell.brain.handle_inputs(&cell);
    match rand::thread_rng().gen_range(0..6) {
        0 => cell.act(CellAction::Up),
        1 => cell.act(CellAction::Down),
        2 => cell.act(CellAction::Left),
        3 => cell.act(CellAction::Right),
        4 => cell.act(CellAction::Give),
        5 => cell.act(CellAction::Take),
        6 => cell.act(CellAction::Chill),
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
