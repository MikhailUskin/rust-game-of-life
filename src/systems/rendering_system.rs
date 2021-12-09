use glam::Vec2;
use ggez::Context;
use ggez::graphics::{self, DrawParam, Image};
use specs::{join::Join, ReadStorage, Read, System};
use crate::constants::*;
use crate::components::*;
use crate::rules::*;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Position>, 
        ReadStorage<'a, Renderable>,
        Read<'a, Generation>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables, generation) = data;

        // Clearing the screen (this gives us the background colour)
        graphics::clear(self.context, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        // Get all the renderables with their positions
        let rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            if generation[position.y as usize][position.x as usize] == CELL_IS_DEAD {
                continue;
            }

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
