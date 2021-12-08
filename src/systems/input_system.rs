use ggez::event::MouseButton;
use specs::{join::Join, ReadStorage, System, Write, WriteStorage};
use crate::constants::*;
use crate::components::*;
use crate::resources::*;
use crate::rules::*;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Write<'a, Generation>,
        WriteStorage<'a, Renderable>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut generation, mut renderables, positions) = data;

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

            for (rendrable, renderable_cell_position) in (&mut renderables, &positions).join() {
                if renderable_cell_position.x != pressed_cell_x {
                    continue;
                }

                if renderable_cell_position.y != pressed_cell_y {
                    continue;
                }

                match button {
                    MouseButton::Left => rendrable.path = String::from(ALIVE_CELL_TILE_PATH),
                    MouseButton::Right => rendrable.path = String::from(DEAD_CELL_TILE_PATH),
                    _ => ()
                }
            }
        }
    }
}
