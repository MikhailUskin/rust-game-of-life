use ggez::{conf, timer, Context, GameResult};
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

const UNIVERSE_UPDATE_ENABLED_STATE: MouseButton = MouseButton::Other(1111);

trait UniverseMouseUpdater {
    fn is_universe_update_enabled(&self) -> bool { 
        false 
    }

    fn enable_universe_update(&mut self) {}

    fn disable_universe_update(&mut self, _button: MouseButton) {}

    fn update_generation(&mut self) {}

    fn capture_mouse_pressed_position(&self, _x: f32, _y:f32) {}
}

// This struct will hold all our game state
struct GameState {
    world: World,
    pressed_button: MouseButton,
}

impl UniverseMouseUpdater for GameState {
    fn is_universe_update_enabled(&self) -> bool { 
        self.pressed_button == UNIVERSE_UPDATE_ENABLED_STATE
    }

    fn enable_universe_update(&mut self) {
        self.pressed_button = UNIVERSE_UPDATE_ENABLED_STATE;
    }

    fn disable_universe_update(&mut self, button: MouseButton) {
        self.pressed_button = button;
    }

    fn update_generation(&mut self) {
        if !self.is_universe_update_enabled()
        {
            return;
        }

        let mut universe_field = self.world.write_resource::<UniverseField>();
        universe_field.field.next_generation();
    }

    fn capture_mouse_pressed_position(&self, x: f32, y:f32) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.mouse_button_events.push(self.pressed_button);
        input_queue.pressed_cell_positions.push(Position {
            x: (x / TILE_WIDTH) as u8,
            y: (y / TILE_WIDTH) as u8,
        });
    }
}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl EventHandler for GameState {
    fn update(&mut self, context : &mut Context) -> GameResult<()> {

        while timer::check_update_time(context, DESIRED_FPS) {
            self.update_generation();
        }

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

        ggez::timer::yield_now();

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        // Filter mouse presses
        match button {
            MouseButton::Left => self.disable_universe_update(MouseButton::Left),
            MouseButton::Right => self.disable_universe_update(MouseButton::Right),
            _ => return,
        }

        self.capture_mouse_pressed_position(x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        // Filter mouse presses
        match button {
            MouseButton::Left => self.enable_universe_update(),
            MouseButton::Right => self.enable_universe_update(),
            _ => return,
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if self.is_universe_update_enabled()
        {
            return;
        }

        self.capture_mouse_pressed_position(x, y);
    }
}

// Initialize the level
pub fn initialize_level(world: &mut World) {
    let (universe_height, universe_width) = world.read_resource::<UniverseField>().field.shape();
    for row in 0..universe_height{
        for column in 0..universe_width{
            // Create the position at which to create something on the map
            let position = Position {
                x: column as u8,
                y: row as u8,
            };

            create_cell(world, position);
        }
    }
}

fn generate_game_state() -> GameState {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    return GameState {
        world,
        pressed_button: UNIVERSE_UPDATE_ENABLED_STATE
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
