use glam::Vec2;
use ggez::{conf, event::{self, EventHandler}, Context, GameResult,
    graphics::{self, DrawParam, Image}};
use specs::{
    join::Join, Builder, Component, ReadStorage, RunNow, System, VecStorage, World, WorldExt};
use std::path;

mod rules;

const TILE_WIDTH: f32 = 10.0;
const UNIVERSE_WIDTH: usize = 50;
const UNIVERSE_HEIGHT: usize = 50;

// This struct will hold all our game state
struct Game {
    world: World,
}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl EventHandler for Game {
    fn update(&mut self, _context : &mut Context) -> GameResult<()> {
        // TODO: update game logic here
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
}

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    path: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct DeadCell {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct AliveCell {}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<DeadCell>();
    world.register::<AliveCell>();
}

pub fn create_dead_cell(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/dead_cell.png".to_string(),
        })
        .with(DeadCell {})
        .build();
}

pub fn create_alive_cell(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/alive_cell.png".to_string(),
        })
        .with(AliveCell {})
        .build();
}

pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}

// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        // Clearing the screen (this gives us the background colour)
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

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

    for (y, row) in initial_generation.iter().enumerate(){
        for (x, cell) in row.iter().enumerate(){
            // Create the position at which to create something on the map
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0,
            };

            // Figure out what object we should create
            if *cell {
                create_alive_cell(world, position);
            }
            else {
                create_dead_cell(world, position);
            }
        }
    }
}

pub fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_game_of_life", "game_of_life")
        .window_setup(conf::WindowSetup::default().title("Conway's Game Of Life!"))
        .window_mode(conf::WindowMode::default().dimensions((UNIVERSE_WIDTH as f32)*TILE_WIDTH, (UNIVERSE_HEIGHT as f32)*TILE_WIDTH))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;

    // Create the game state
    let game = Game { world };

    // Run the main event loop
    event::run(context, event_loop, game)
}
