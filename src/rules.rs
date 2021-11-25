use rand::{distributions::Uniform, Rng};

type Generation = Vec<Vec<bool>>;

//type rule_kernel = Vec<Vec<bool>>;

//const rule_kernel: [[], ];
const is_alive: bool = true;
const is_dead: bool = false;

pub struct Universe {
    alive_neighbours: Vec<Vec<u8>>,
}

impl Universe {
    fn make_alive_neighbours(width: usize, height: usize) -> Vec<Vec<u8>> {
        return vec![vec![0 as u8; width]; height];
    }

    pub fn new(width: usize, height: usize) -> Self {
        Universe {
            alive_neighbours: Universe::make_alive_neighbours(width, height)
        }
    }

    pub fn seed_initial_generation(&self) -> Generation {
        let mut random_generator = rand::thread_rng();
        let uniform_range = Uniform::new(0, 10);

        let universe_width = self.get_universe_width();
        let universe_height = self.get_universe_height();

        let mut initial_generation: Generation = Generation::new();
        for m in 0..universe_height
        {
            initial_generation.push(vec![]);
            for n in 0..universe_width
            {
                let is_cell_alive = random_generator.sample(&uniform_range) >= 5;
                initial_generation[m].push(is_cell_alive);
            } 
        }

        return initial_generation;
    }

    fn get_universe_height(&self) -> usize {
        return self.alive_neighbours.len();
    }

    fn get_universe_width(&self) -> usize {
        assert!(self.alive_neighbours.len() > 0);

        return self.alive_neighbours[0].len();
    }

    // is_need_to_be_killed(current_cell, number_of_alive_neighbouts) -> bool
    // {
    //     return current_cell_is_alive && (number_of_cell_neighbours < 2 || number_of_cell_neighbours > 3));
    // }

    // is_need_to_be_alived(current_cell, number_of_alive_neighbouts) -> bool
    // {
    //     return current_cell_is_dead && number_of_cell_neighbours == 3;
    // }



    // pub update_generation(mut& previous_generation) {
        // assert_eq!() - check width
        // assert_eq!() - check height

        // convolve_with_rule_kernel(previous_generation)
        //
        // for (each_cell)
        // {
        //     if (is_need_to_be_alived(current_cell, get_number_of_alive_neighbours(position)))
        //     {
        //         make_current_cell_alive
        //     } 
        //     else if (is_need_to_be_killed(current_cell, getnumber_of_alive_neighbours(position)))
        //     {
        //         make_current_cell_dead
        //     }
        // }
    // }
}
