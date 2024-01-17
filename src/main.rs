mod player;
mod apple;

use macroquad::prelude::*;
use player::Player;

fn WindowConfig() -> Conf {
    let mut c = Conf {
        window_title: "RustSnake".to_owned(),
        fullscreen: false,
        ..Default::default()
    };

    c.window_width = 512;
    c.window_height = 512;
    return c;
}

#[macroquad::main(WindowConfig)]
async fn main() {
    let mut exited = false;

    let mut player = Player::new(0.0, 0.0, 32.0, 32.0, 32.0);
    let mut apple = Apple::new(128.0, 128.0, 32.0, 32.0);

    let mut gameSpeed = 0.2;
    let mut lastUpdate = get_time();

    Start();

    while (!exited) {
        if (get_time() - lastUpdate >= gameSpeed) {
            Update(&mut player, &mut exited);
            Draw(&player);
            lastUpdate = get_time();
            next_frame().await;
        }
    }

    End();
}

fn Start() {

}

fn Update(player: &mut Player, exit: &mut bool) {
    if (is_key_pressed(KeyCode::Escape)) {
        (*exit) = true;
    }

    if (is_key_pressed(KeyCode::Right)) {
        player.SetDirection(1, 0);
    }

    if (is_key_pressed(KeyCode::Up)) {
        player.SetDirection(0, -1);
    }

    if (is_key_pressed(KeyCode::Left)) {
        player.SetDirection(-1, 0);
    }

    if (is_key_pressed(KeyCode::Down)) {
        player.SetDirection(0, 1);
    }

    player.Move();
}

fn Draw(player: &Player, apple: &Apple) {
    clear_background(BLACK);
    player.Draw();
    apple.Draw();
}

fn End() {

}