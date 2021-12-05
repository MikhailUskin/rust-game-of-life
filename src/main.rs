use glam::Vec2;
use ggez::{conf, Context, GameResult,
        event::{self, EventHandler, MouseButton}, 
        graphics::{self, DrawParam, Image}};
use specs::{
        join::Join, Builder, Component, ReadStorage, RunNow, System, VecStorage, World, WorldExt, Write, WriteStorage,};
use std::path;

mod rules;
use rules::*;

const TILE_WIDTH: f32 = 10.0;
const UNIVERSE_WIDTH: usize = 50;
const UNIVERSE_HEIGHT: usize = 50;

const ALIVE_CELL_TILE_PATH: &str = "/images/alive_cell.png";
const DEAD_CELL_TILE_PATH: &str = "/images/dead_cell.png";

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

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position {
    x: u8,
    y: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    path: String,
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
}

pub fn create_cell<'a>(world: &mut World, position: Position, path: String) {
    world
        .create_entity()
        .with(Position { 
            ..position 
        })
        .with(Renderable {
            path: path,
        })
        .build();
}

pub fn create_alive_cell(world: &mut World, position: Position){
    create_cell(world, position, String::from(ALIVE_CELL_TILE_PATH));
}

pub fn create_dead_cell(world: &mut World, position: Position){
    create_cell(world, position, String::from(DEAD_CELL_TILE_PATH));
}

pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}

// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Position>, 
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        // Clearing the screen (this gives us the background colour)
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image = Image::new(self.context, renderable.path.clone()).expect("expected image");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}

// Initialize the level
pub fn initialize_level(world: &mut World) {
    let initial_generation = rules::Universe::new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT).seed_initial_generation();

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

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub mouse_button_events: Vec<MouseButton>,
    pub pressed_cell_positions: Vec<Position>,
}

// Registering resources
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Write<'a, Generation>,
        WriteStorage<'a, Renderable>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut generation, mut renderables, positions) = data;

        while 
            input_queue.mouse_button_events.len() > 0 && 
            input_queue.pressed_cell_positions.len() > 0 {

            let button_optional = input_queue.mouse_button_events.pop();
            if !button_optional.is_some()
            {
                continue;
            }

            let pressed_position_optional = input_queue.pressed_cell_positions.pop();
            if !pressed_position_optional.is_some()
            {
                continue;
            }

            let button = button_optional.unwrap();
            let pressed_position = pressed_position_optional.unwrap();

            let pressed_cell_x = pressed_position.x;
            let pressed_cell_y = pressed_position.y;

            match button {
                MouseButton::Left => generation[pressed_cell_y as usize][pressed_cell_x as usize] = CELL_IS_ALIVE,
                MouseButton::Right => generation[pressed_cell_y as usize][pressed_cell_x as usize] = CELL_IS_DEAD,
                _ => ()
            }

            for (rendrable, renderable_cell_position) in (&mut renderables, &positions).join() {
                if renderable_cell_position.x != pressed_cell_x {
                    continue;
                }

                if renderable_cell_position.y != pressed_cell_y {
                    continue;
                }

                match button {
                    MouseButton::Left => rendrable.path = String::from(ALIVE_CELL_TILE_PATH),
                    MouseButton::Right => rendrable.path = String::from(DEAD_CELL_TILE_PATH),
                    _ => ()
                }
            }
        }
    }
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
