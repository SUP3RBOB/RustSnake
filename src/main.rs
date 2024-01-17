mod player;
mod apple;

use macroquad::prelude::*;
use player::Player;
use apple::Apple;

fn window_config() -> Conf {
    return Conf {
        window_title: "RustSnake".to_owned(),
        fullscreen: false,
        window_width: 512,
        window_height: 512,
        ..Default::default()
    };
}

#[macroquad::main(window_config)]
async fn main() {
    let mut exited = false;

    let mut player = Player::new(64.0, 64.0, 32.0, 32.0, 32.0);
    let mut apple = Apple::new(128.0, 128.0, 32.0, 32.0);
    apple.jump_to_random_position();

    let gameSpeed = 0.15;
    let mut lastUpdate = get_time();

    while (!exited) {
        if (get_time() - lastUpdate >= gameSpeed) {
            update(&mut player, &mut apple, &mut exited);
            draw(&player, &apple);
            lastUpdate = get_time();
            next_frame().await;
        }
    }
}

fn update(player: &mut Player, apple: &mut Apple, exit: &mut bool) {
    if (is_key_pressed(KeyCode::Escape)) {
        (*exit) = true;
    }

    if (is_key_pressed(KeyCode::Right)) {
        player.set_direction(1, 0);
    }

    if (is_key_pressed(KeyCode::Up)) {
        player.set_direction(0, -1);
    }

    if (is_key_pressed(KeyCode::Left)) {
        player.set_direction(-1, 0);
    }

    if (is_key_pressed(KeyCode::Down)) {
        player.set_direction(0, 1);
    }

    player.move_player();

    if (player.position() == apple.position()) {
        player.add_to_tail();
        apple.jump_to_random_position();
    }

    if (player.is_out_of_bounds() || player.is_touching_tail()) {
        player.clear_tails();
        player.set_position(64.0, 64.0);
        player.set_direction(1, 0);
        apple.jump_to_random_position();
    }
}

fn draw(player: &Player, apple: &Apple) {
    clear_background(BLACK);
    player.draw();
    apple.draw();
}