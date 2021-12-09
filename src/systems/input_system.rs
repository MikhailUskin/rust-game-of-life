use ggez::event::MouseButton;
use specs::{System, Write};
use crate::resources::*;
use crate::rules::*;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Write<'a, Generation>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut generation) = data;

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
        }
    }
}
