use specs::World;
use ggez::event::MouseButton;
use crate::components::*;

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
