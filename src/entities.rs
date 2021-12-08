use specs::{Builder, World, WorldExt};
use crate::components::*;
use crate::constants::*;

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
