use specs::{Component, VecStorage, World, WorldExt};

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub path: String,
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
}
