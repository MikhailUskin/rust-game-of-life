use ggez::event::MouseButton;
use specs::{System, Write};
use crate::resources::*;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Write<'a, UniverseField>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut universe_field) = data;

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
                MouseButton::Left => universe_field.field.revive_cell(pressed_cell_y, pressed_cell_x),
                MouseButton::Right => universe_field.field.kill_cell(pressed_cell_y, pressed_cell_x),
                _ => ()
            }
        }
    }
}
