use specs::{Builder, World, WorldExt};
use crate::components::*;
use crate::constants::*;

pub fn create_cell<'a>(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { 
            ..position 
        })
        .with(Renderable {
            path: String::from(ALIVE_CELL_TILE_PATH),
        })
        .build();
}
