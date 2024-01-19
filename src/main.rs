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
        window_resizable: false,
        ..Default::default()
    };
}

#[macroquad::main(window_config)]
async fn main() {
    let mut exited = false;

    let mut player = Player::new(64.0, 64.0, 32.0, 32.0, 32.0);
    let mut apple = Apple::new(128.0, 128.0, 32.0, 32.0);
    apple.jump_to_random_position();

    let game_speed = 0.15;
    let mut last_update = get_time();

    while (!exited) {
        get_inputs(&mut player, &mut exited);

        if (get_time() - last_update >= game_speed) {
            update(&mut player, &mut apple);
            draw(&player, &apple);
            last_update = get_time();
            next_frame().await;
        }
    }
}

fn get_inputs(player: &mut Player, exit: &mut bool) {
    if (is_key_pressed(KeyCode::Escape)) {
        (*exit) = true;
    }

    if (is_key_pressed(KeyCode::Right)) {
        if (player.get_direction() != (-1, 0)) {
            player.set_direction(1, 0);
        }
    }

    if (is_key_pressed(KeyCode::Up)) {
        if (player.get_direction() != (0, 1)) {
            player.set_direction(0, -1);
        }
    }

    if (is_key_pressed(KeyCode::Left)) {
        if (player.get_direction() != (1, 0)) {
            player.set_direction(-1, 0);
        }
    }

    if (is_key_pressed(KeyCode::Down)) {
        if (player.get_direction() != (0, -1)) {
            player.set_direction(0, 1);
        }
    }
}

fn update(player: &mut Player, apple: &mut Apple) {
    player.move_player();

    if (player.position() == apple.position()) {
        player.add_to_tail();
        apple.jump_to_random_position();
    }

    if (player.is_out_of_bounds() || player.is_touching_tail()) {
        let tail_count = player.get_tail_count();
        if (tail_count > player.get_highscore()) {
            player.set_highscore(tail_count);
        }

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