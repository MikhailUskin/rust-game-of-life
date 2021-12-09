use ggez::{conf, Context, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use specs::{RunNow, World, WorldExt};
use std::path;

mod components;
mod constants;
mod entities;
mod resources;
mod systems;
mod rules;

use crate::resources::*;
use crate::components::*;
use crate::constants::*;
use crate::entities::*;
use crate::systems::*;
use crate::rules::*;

// This struct will hold all our game state
struct GameState {
    world: World,
}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl EventHandler for GameState {
    fn update(&mut self, _context : &mut Context) -> GameResult<()> {

        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        // Render game entities
        {
            let mut rs = RenderingSystem { context };
            rs.run_now(&self.world);
        }

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);

        let mut input_queue = self.world.write_resource::<InputQueue>();

        // Filter mouse presses
        match button {
            MouseButton::Left => input_queue.mouse_button_events.push(MouseButton::Left),
            MouseButton::Right => input_queue.mouse_button_events.push(MouseButton::Right),
            _ => return,
        }

        input_queue.pressed_cell_positions.push(Position {
            x: (x / TILE_WIDTH) as u8,
            y: (y / TILE_WIDTH) as u8,
        });
    }
}

// Initialize the level
pub fn initialize_level(world: &mut World) {
    let initial_generation = world.read_resource::<Universe>().seed_initial_generation();
    for (cell_y, row) in initial_generation.iter().enumerate(){
        for (cell_x, is_alive) in row.iter().enumerate(){
            // Create the position at which to create something on the map
            let position = Position {
                x: cell_x as u8,
                y: cell_y as u8,
            };

            // Figure out what object we should create
            if *is_alive {
                create_alive_cell(world, position);
            }
            else {
                create_dead_cell(world, position);
            }
        }
    }

    world.insert(initial_generation);
}

fn generate_game_state() -> GameState {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    return GameState {
        world
    };
}

pub fn main() -> GameResult {
    // Create the game state
    let game = generate_game_state();

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_game_of_life", "game_of_life")
        .window_setup(conf::WindowSetup::default().title("Conway's Game Of Life!"))
        .window_mode(conf::WindowMode::default().dimensions((UNIVERSE_WIDTH as f32)*TILE_WIDTH, (UNIVERSE_HEIGHT as f32)*TILE_WIDTH))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;

    // Run the main event loop
    event::run(context, event_loop, game)
}
