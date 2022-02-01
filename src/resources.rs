use specs::World;
use ggez::event::MouseButton;
use crate::components::*;
use crate::universe::*;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub mouse_button_events: Vec<MouseButton>,
    pub pressed_cell_positions: Vec<Position>,
}

pub struct UniverseField {
    pub field: Universe,
}

impl Default for UniverseField {
    fn default() -> Self {
        Self { field: Universe::new_random() }
    }
}

// Registering resources
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(UniverseField::default());
}
