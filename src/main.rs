use raylib::prelude::*;
mod game;
use crate::game::Game;

const SCREEN_WIDTH:i32 = 1920;
const SCREEN_HEIGHT:i32 = 1080;
const CELL_SIDE_LENGTH:i32 = 120;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Game of Life")
        .fullscreen()
        .build();

    let mut game = Game::new(&rl, CELL_SIDE_LENGTH);

    while !rl.window_should_close() {
        // Logic
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            game.toggle_run();
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            game.set_cell_alive_at_mouse_position(rl.get_mouse_position());
        }

        game.apply_rule_logic();

        // Drawing
        let mut draw_handle = rl.begin_drawing(&thread);
        game.draw_grid(&mut draw_handle);
    }
}

