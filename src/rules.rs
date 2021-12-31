use nalgebra::{matrix, SMatrix};
use crate::constants::*;

pub type UniverseState = SMatrix<u8, UNIVERSE_WIDTH, UNIVERSE_HEIGHT>;
pub type RuleKernel = SMatrix<u8, 3, 3>;

pub const CELL_IS_ALIVE: u8 = 1;
pub const CELL_IS_DEAD: u8 = 0;

pub fn convolve_full_wrap<const R1: usize, const C1: usize, const R2: usize, const C2: usize>(matrix: &SMatrix<u8, R1, C1>, kernel: &SMatrix<u8, R2, C2>) -> SMatrix<u8, R1, C1> {
    let matrix_shape = matrix.shape();
    let kernel_shape = kernel.shape();

    if matrix_shape > kernel_shape {
        panic!("convolve_full_wrap expects `self.shape() > kernel.shape()`, received {:?} and {:?} respectively.", matrix_shape, kernel_shape);
    }

    

    SMatrix::<u8, R1, C1>::zeros()
}

const RULE_KERNEL: RuleKernel = matrix![1, 1, 1;
                                        1, 0, 1;
                                        1, 1, 1];

pub struct Universe {
    next_generation: UniverseState,
    alive_neighbours: UniverseState,
}

impl Universe {
    fn seed_initial_generation() -> UniverseState {
        let mut initial_generation: UniverseState = UniverseState::new_random();

        let separate_point = u8::MAX / 2;
        initial_generation /= separate_point;

        initial_generation
    }

    pub fn new() -> Self {
        let initial_generation = Universe::seed_initial_generation(); 
        let initial_neighbours = convolve_full_wrap(&initial_generation, &RULE_KERNEL);

        Universe {
            next_generation: initial_generation,
            alive_neighbours: initial_neighbours,
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        self.next_generation.shape()
    }

    pub fn revive_cell(&mut self, cell_row: u8, cell_column: u8) {
        self.next_generation[(cell_row as usize, cell_column as usize)] = CELL_IS_ALIVE;
    }

    pub fn kill_cell(&mut self, cell_row: u8, cell_column: u8) {
        self.next_generation[(cell_row as usize, cell_column as usize)] = CELL_IS_DEAD;
    }

    pub fn get_cell_state(&self, cell_row: u8, cell_column: u8) -> u8 {
        return self.next_generation[(cell_row as usize, cell_column as usize)];
    }

    fn is_need_to_be_killed(current_cell: u8, number_of_alive_neighbours: u8) -> bool
    {
        return current_cell == CELL_IS_ALIVE && (number_of_alive_neighbours < 2 || number_of_alive_neighbours > 3);
    }

    fn is_need_to_be_alived(current_cell: u8, number_of_alive_neighbours: u8) -> bool
    {
        return current_cell == CELL_IS_DEAD && number_of_alive_neighbours == 3;
    }

    pub fn next_generation(&mut self) {
        self.alive_neighbours = convolve_full_wrap(&self.next_generation, &RULE_KERNEL);
        self.next_generation.zip_apply(&self.alive_neighbours, |current_cell_state, number_of_alive_neighbours| {
            if Universe::is_need_to_be_alived(*current_cell_state, number_of_alive_neighbours)
            {
                *current_cell_state = CELL_IS_ALIVE;
            }
            else if Universe::is_need_to_be_killed(*current_cell_state, number_of_alive_neighbours)
            {
                *current_cell_state = CELL_IS_DEAD;
            }
        });
    }
}
