use nalgebra::{matrix, SMatrix};
use rand::distributions::{Uniform};
use crate::constants::*;

pub type RuleKernel = SMatrix<u8, RULE_KERNEL_WIDTH, RULE_KERNEL_HEIGHT>;
pub type UniverseWrapped = SMatrix<u8, UNIVERSE_WRAPPED_WIDTH, UNIVERSE_WRAPPED_HEIGHT>;

const RULE_KERNEL_WIDTH_HALF: usize = RULE_KERNEL_WIDTH / 2;
const RULE_KERNEL_HEIGHT_HALF: usize = RULE_KERNEL_HEIGHT / 2;
const UNIVERSE_WRAPPED_WIDTH: usize = UNIVERSE_WIDTH + (2 * RULE_KERNEL_WIDTH_HALF);
const UNIVERSE_WRAPPED_HEIGHT: usize = UNIVERSE_HEIGHT + (2 * RULE_KERNEL_HEIGHT_HALF);

pub const CELL_IS_POPULATED: u8 = 1;
pub const CELL_IS_FREE: u8 = 0;

pub fn convolve_torus<const R1: usize, const C1: usize, const R2: usize, const C2: usize>(torus_wrapped_plane: & mut SMatrix<u8, R1, C1>, kernel: &SMatrix<u8, R2, C2>) -> SMatrix<u8, R1, C1>
{
    let kernel_height = kernel.shape().0;
    let kernel_width = kernel.shape().1;

    let kernel_height_half = kernel_height / 2;
    let kernel_width_half = kernel_width / 2;

    let torus_wrapped_plane_shape = torus_wrapped_plane.shape();

    let torus_plane_height = torus_wrapped_plane_shape.0 - (2 * kernel_height_half);
    let torus_plane_width = torus_wrapped_plane_shape.1 - (2 * kernel_width_half);

    let torus_plane_shape = (torus_plane_height, torus_plane_width);
    let kernel_shape = kernel.shape();

    if kernel_shape > torus_plane_shape {
        panic!("'convolve_on_torus' expects `torus_plane_shape.shape() > kernel.shape()`, received {:?} and {:?} respectively.", 
            torus_plane_shape, kernel_shape);
    }

    let mut reflect_slice = |source_position, target_position, slice_shape| {
        let slice_source = torus_wrapped_plane.slice(source_position, slice_shape).clone_owned();
        let mut slice_target = torus_wrapped_plane.slice_mut(target_position, slice_shape);
        slice_target.copy_from(&slice_source);
    };

    // Corner shape (suppose kernel is square)

    let corner_slice_shape = (kernel_height_half, kernel_width_half);

    // Upper left corner

    let upper_left_slice_target_position = (0, 0);
    let upper_left_slice_source_position = (upper_left_slice_target_position.0 + torus_plane_height, upper_left_slice_target_position.1 + torus_plane_width);
    reflect_slice(upper_left_slice_source_position, upper_left_slice_target_position, corner_slice_shape);

    // Upper right corner

    let upper_right_slice_target_position = (0, kernel_width_half + torus_plane_width);
    let upper_right_slice_source_position = (upper_right_slice_target_position.0 + torus_plane_height, upper_right_slice_target_position.1 - torus_plane_width);
    reflect_slice(upper_right_slice_source_position, upper_right_slice_target_position, corner_slice_shape);

    // Bottom right corner

    let bottom_right_slice_target_position = (kernel_height_half + torus_plane_height, kernel_width_half + torus_plane_width);
    let bottom_right_slice_source_position = (bottom_right_slice_target_position.0 - torus_plane_height, bottom_right_slice_target_position.1 - torus_plane_width);
    reflect_slice(bottom_right_slice_source_position, bottom_right_slice_target_position, corner_slice_shape);

    // Bottom left corner

    let bottom_left_slice_target_position = (kernel_height_half + torus_plane_height, 0);
    let bottom_left_slice_source_position = (bottom_left_slice_target_position.0 - torus_plane_height, bottom_left_slice_target_position.1 + torus_plane_width);
    reflect_slice(bottom_left_slice_source_position, bottom_left_slice_target_position, corner_slice_shape);

    // Horizontal side shape

    let horizontal_side_shape = (torus_plane_height, kernel_width_half);

    // Left side

    let left_side_target_position = (kernel_height_half, 0);
    let left_side_source_position = (left_side_target_position.0, left_side_target_position.1 + torus_plane_width);
    reflect_slice(left_side_source_position, left_side_target_position, horizontal_side_shape);

    // Right side

    let right_side_target_position = (kernel_height_half, kernel_width_half + torus_plane_width);
    let right_side_source_position = (right_side_target_position.0, right_side_target_position.1 - torus_plane_width);
    reflect_slice(right_side_source_position, right_side_target_position, horizontal_side_shape);

    // Vertical side shape

    let vertical_side_shape = (kernel_height_half, torus_plane_width);

    // Upper side

    let upper_side_target_position = (0, kernel_width_half);
    let upper_side_source_position = (upper_side_target_position.0 + torus_plane_height, upper_side_target_position.1);
    reflect_slice(upper_side_source_position, upper_side_target_position, vertical_side_shape);

    // Bottom side

    let bottom_side_target_position = (kernel_height_half + torus_plane_height, kernel_width_half);
    let bottom_side_source_position = (bottom_side_target_position.0 - torus_plane_height, bottom_side_target_position.1);
    reflect_slice(bottom_side_source_position, bottom_side_target_position, vertical_side_shape);

    // Convolve

    let mut convolve_result = SMatrix::<u8, R1, C1>::zeros();
    let min_row_index = kernel_height_half;
    let max_row_index = torus_plane_height + kernel_height_half;
    let min_column_index = kernel_width_half;
    let max_column_index = torus_plane_width + kernel_width_half;

    for row_index in min_row_index..max_row_index
    {
        for column_index in min_column_index..max_column_index
        {
            let convolve_slice_position = (row_index - kernel_height_half, column_index - kernel_width_half);
            let matrix_slice = torus_wrapped_plane.slice(convolve_slice_position, kernel.shape());

            let convolve_target_position = (row_index, column_index);
            convolve_result[convolve_target_position] = matrix_slice.dot(&kernel);
        }
    }

    convolve_result
}

const RULE_KERNEL: RuleKernel = matrix![1, 1, 1;
                                        1, 0, 1;
                                        1, 1, 1];

pub struct Universe {
    next_generation_wrapped: UniverseWrapped,
    alive_neighbours_wrapped: UniverseWrapped,
}

impl Universe {
    fn seed_initial_generation() -> UniverseWrapped {
        let mut random_generator = rand::thread_rng();
        let uniform_range = Uniform::new_inclusive(0, 1);
        let initial_generation: UniverseWrapped = UniverseWrapped::from_distribution(&uniform_range, &mut random_generator);
 
        initial_generation
    }

    pub fn new_random() -> Self {
        let mut initial_generation = Universe::seed_initial_generation(); 
        let initial_neighbours = convolve_torus(&mut initial_generation, &RULE_KERNEL);

        Universe {
            next_generation_wrapped: initial_generation,
            alive_neighbours_wrapped: initial_neighbours,
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (UNIVERSE_HEIGHT, UNIVERSE_WIDTH)
    }

    fn get_plane_position(cell_row: u8, cell_column: u8) -> (usize, usize) {
        ((cell_row as usize) + RULE_KERNEL_HEIGHT_HALF, (cell_column as usize) + RULE_KERNEL_WIDTH_HALF)
    }

    pub fn populate_cell(&mut self, cell_row: u8, cell_column: u8) {
        let position = Universe::get_plane_position(cell_row, cell_column);
        self.next_generation_wrapped[position] = CELL_IS_POPULATED;
    }

    pub fn free_cell(&mut self, cell_row: u8, cell_column: u8) {
        let position = Universe::get_plane_position(cell_row, cell_column);
        self.next_generation_wrapped[position] = CELL_IS_FREE;
    }

    pub fn get_cell_state(&self, cell_row: u8, cell_column: u8) -> u8 {
        let position = Universe::get_plane_position(cell_row, cell_column);
        return self.next_generation_wrapped[position];
    }

    fn is_need_to_be_killed(current_cell: u8, number_of_alive_neighbours: u8) -> bool
    {
        return current_cell == CELL_IS_POPULATED && (number_of_alive_neighbours < 2 || number_of_alive_neighbours > 3);
    }

    fn is_need_to_be_alived(current_cell: u8, number_of_alive_neighbours: u8) -> bool
    {
        return current_cell == CELL_IS_FREE && number_of_alive_neighbours == 3;
    }

    pub fn next_generation(&mut self) {
        self.alive_neighbours_wrapped = convolve_torus(&mut self.next_generation_wrapped, &RULE_KERNEL);

        self.next_generation_wrapped.zip_apply(&self.alive_neighbours_wrapped, |current_cell_state, number_of_alive_neighbours| {
            if Universe::is_need_to_be_alived(*current_cell_state, number_of_alive_neighbours)
            {
                *current_cell_state = CELL_IS_POPULATED;
            }
            else if Universe::is_need_to_be_killed(*current_cell_state, number_of_alive_neighbours)
            {
                *current_cell_state = CELL_IS_FREE;
            }
        });
    }
}

// fn populate_all_cells(universe) {
//     // Iterate over all cells
// }

// fn free_all_cells(universe) {
//     // Iterate over all cells
// }

// fn create_free_universe() -> Universe {
//     // Create universe
//     free_all_cells(universe)

//     universe
// }

// fn build_glider(upper_left_position, universe) {
//     // Put glider starting from upper left postion
// }

// fn generate_glider_pattern(upper_left_position)  {
//     return positions, states;
// }

// fn is_glider_detected(position, universe) -> bool {
//     // Check each of 9 cells

//     for (position, state) in generate_glider_pattern(position) {
//         if universe.get_cell_state(position) != state {
//             return false;
//         }
//     }

//     return true;
// }

#[cfg(test)]
mod test {
    use super::*;

    const GLIDER_PERIOD: usize = 4;
    const GLIDER_HEIGHT: usize = 3;
    const GLIDER_WIDTH: usize = 3;

    fn iterate_universe<F: FnMut(u8, u8)>(shape: (usize, usize), mut indexer: F) {
        let (universe_height, universe_width) = shape;
        for row_index in 0..universe_height {
            for column_index in 0..universe_width {
                indexer(row_index as u8, column_index as u8);
            }
        }
    }

    #[test]
    fn at_least_one_cell_is_populated() {

        let universe = Universe::new_random();
        let mut number_of_populated_cells: usize = 0;

        let indexer = |row_index: u8, column_index: u8| {
            if universe.get_cell_state(row_index, column_index) == CELL_IS_POPULATED {
                number_of_populated_cells += 1;
            }
        };

        iterate_universe(universe.shape(), indexer);
        assert!(number_of_populated_cells > 0);
    }

    #[test]
    fn at_least_one_cell_is_free() {

        let universe = Universe::new_random();
        let mut number_of_free_cells: usize = 0;

        let indexer = |row_index: u8, column_index: u8| {
            if universe.get_cell_state(row_index, column_index) == CELL_IS_FREE {
                number_of_free_cells += 1;
            }
        };

        iterate_universe(universe.shape(), indexer);
        assert!(number_of_free_cells > 0);
    }

    #[test]
    fn any_cell_can_be_populated() {
        let mut universe = Universe::new_random();
        let shape = universe.shape();

        let indexer = |row_index: u8, column_index: u8| {
            universe.populate_cell(row_index, column_index);

            let cell_state = universe.get_cell_state(row_index, column_index);
            assert_eq!(cell_state, CELL_IS_POPULATED);
        };

        iterate_universe(shape, indexer);
    }

    #[test]
    fn any_cell_can_be_freed() {
        let mut universe = Universe::new_random();
        let shape = universe.shape();

        let indexer = |row_index: u8, column_index: u8| {
            universe.free_cell(row_index, column_index);

            let cell_state = universe.get_cell_state(row_index, column_index);
            assert_eq!(cell_state, CELL_IS_FREE);
        };

        iterate_universe(shape, indexer);
    }

    // #[test]
    // fn glider_can_move_around_the_center() {
    //     let mut universe = create_free_universe();
    //     let (universe_height, universe_width) = universe.shape();

    //     let glider_position = (universe_height / 2, universe_width / 2);
    //     build_glider(glider_position, &mut universe);

    //     for _ in 0..GLIDER_PERIOD {
    //         universe.next_generation();
    //     }

    //     let expected_position = (0, 0);
    //     assert!(is_glider_detected(expected_position, universe));
    // }

    // #[test]
    // fn glider_can_cross_horizontal_borders() {
    //     let mut universe = create_free_universe();
    //     let (universe_height, universe_width) = universe.shape();

    //     let glider_position = (universe_height / 2, universe_width - GLIDER_WIDTH);
    //     build_glider(glider_position, &mut universe);

    //     for _ in 0..GLIDER_PERIOD {
    //         universe.next_generation();
    //     }

    //     let expected_position = (0, 0);
    //     assert!(is_glider_detected(expected_position, universe));
    // }

    // #[test]
    // fn glider_can_cross_vertical_borders() {
    //     let mut universe = create_free_universe();
    //     let (universe_height, universe_width) = universe.shape();

    //     let glider_position = (universe_height - GLIDER_HEIGHT, universe_width / 2);
    //     build_glider(glider_position, &mut universe);

    //     for _ in 0..GLIDER_PERIOD {
    //         universe.next_generation();
    //     }

    //     let expected_position = (0, 0);

    //     assert!(is_glider_detected(expected_position, universe));
    // }

    // #[test]
    // fn glider_can_cross_corners_from_bottom_right_to_upper_left() {
    //     // Create universe

    //     // Free universe

    //     // Put glider at the bottom-right corner

    //     // Update universe state as period of glider

    //     assert!(is_glider_detected(expected_position, universe));
    // }

    // #[test]
    // fn glider_can_cross_corners_from_upper_right_to_bottom_left() {
    //     // Create universe

    //     // Free universe

    //     // Put glider at the upper-right corner

    //     // Update universe state as period of glider

    //     assert!(is_glider_detected(expected_position, universe));
    // }
}
