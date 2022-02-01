use glam::Vec2;
use ggez::Context;
use ggez::graphics::{self, DrawParam, Image, spritebatch::SpriteBatch};
use specs::{join::Join, ReadStorage, Read, System};
use std::collections::HashMap;
use crate::constants::*;
use crate::components::*;
use crate::resources::*;
use crate::universe::*;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Position>, 
        ReadStorage<'a, Renderable>,
        Read<'a, UniverseField>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables, universe_field) = data;

        // Clearing the screen (this gives us the background colour)
        graphics::clear(self.context, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        // Get all the renderables with their positions
        let rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        let mut rendering_batches: HashMap<String, Vec<DrawParam>> = HashMap::new();

        // Iterate each of the renderables, determine which image path should be rendered
        // at which drawparams, and then add that to the rendering_batches.
        for (position, renderable) in rendering_data.iter() {
            if universe_field.field.get_cell_state(position.y, position.x) == CELL_IS_FREE {
                continue;
            }

            let x = (position.x as f32) * TILE_WIDTH;
            let y = (position.y as f32) * TILE_WIDTH;

            // Add to rendering batches
            let draw_param = DrawParam::new().dest(Vec2::new(x, y));
            rendering_batches
                .entry(renderable.path.clone())
                .or_default()
                .push(draw_param);
        }

        // Iterate spritebatches ordered by z and actually render each of them
        for (image_path, draw_params) in rendering_batches.iter()
        {
            let image = Image::new(self.context, image_path).expect("expected image");
            let mut sprite_batch = SpriteBatch::new(image);

            for draw_param in draw_params.iter() {
                sprite_batch.add(*draw_param);
            }

            graphics::draw(self.context, &sprite_batch, graphics::DrawParam::new())
                .expect("expected render");
        }

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}
