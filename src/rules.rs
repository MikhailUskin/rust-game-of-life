use nalgebra::{matrix, SMatrix};
use std::cmp;
use crate::constants::*;

pub type UniverseState = SMatrix<u8, UNIVERSE_WIDTH, UNIVERSE_HEIGHT>;
pub type RuleKernel = SMatrix<u8, 3, 3>;

pub const CELL_IS_ALIVE: u8 = 1;
pub const CELL_IS_DEAD: u8 = 0;

pub fn convolve_full_wrap<const R1: usize, const C1: usize, const R2: usize, const C2: usize>(matrix: &SMatrix<u8, R1, C1>, kernel: &SMatrix<u8, R2, C2>) -> SMatrix<u8, R1, C1> {
    let matrix_shape = matrix.shape();
    let kernel_shape = kernel.shape();

    if kernel_shape > matrix_shape {
        panic!("convolve_full_wrap expects `self.shape() > kernel.shape()`, received {:?} and {:?} respectively.", matrix_shape, kernel_shape);
    }

    let mut convolve_result = SMatrix::<u8, R1, C1>::zeros();

    let matrix_height = matrix.shape().0;
    let matrix_width = matrix.shape().1;

    let kernel_height = kernel.shape().0;
    let kernel_width = kernel.shape().1;

    let kernel_height_half = kernel_height / 2;
    let kernel_width_half = kernel_width / 2;

    for row_index in 0..matrix_height
    {
        for column_index in 0..matrix_width
        {
            let convolve_position = (row_index, column_index);

            let local_slice_row = if convolve_position.0 > kernel_height_half { convolve_position.0 - kernel_height_half } else { 0 };
            let local_slice_column = if convolve_position.1 > kernel_height_half { convolve_position.1 - kernel_height_half } else { 0 };
            let local_slice_position = (local_slice_row, local_slice_column);

            let upper_left_offset = (convolve_position.0 - local_slice_position.0, convolve_position.1 - local_slice_position.1);

            let max_bottow_right_position = (convolve_position.0 + kernel_height_half, convolve_position.1 + kernel_width_half);
            let bottom_right_offset = (cmp::min(max_bottow_right_position.0, matrix_height) - convolve_position.0, 
                cmp::min(max_bottow_right_position.1, matrix_width) - convolve_position.1);

            let local_shape = (bottom_right_offset.0 - upper_left_offset.0 + 1, bottom_right_offset.1 - upper_left_offset.1 + 1);
            let local_matrix = matrix.slice(local_slice_position, local_shape);

            let kernel_center = (kernel_height_half, kernel_width_half);
            let local_kernel_position = (kernel_center.0 - upper_left_offset.0, kernel_center.1 - upper_left_offset.1);
            let local_kernel_shape = (bottom_right_offset.0 - upper_left_offset.0 + 1, bottom_right_offset.1 - upper_left_offset.1 + 1);
            let local_kernel = kernel.slice(local_kernel_position, local_kernel_shape);
            convolve_result[(row_index, column_index)] = local_matrix.dot(&local_kernel);

            //let wrapped_columns_slice = matrix.fixed_slice::<>();
            //let wrapped_columns_kernel = kernel.fixed_slice::<>();
            //wrapped_columns_slice.dot(wrapped_columns_kernel);

            //let wrapped_rows_slice = matrix.fixed_slice::<>();
            //let wrapped_rows_kernel = kernel.fixed_slice::<>();
            //wrapped_rows_slice.dot(wrapped_rows_kernel);
        }
    }

    convolve_result
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
