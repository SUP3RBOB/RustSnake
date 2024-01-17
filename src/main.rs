mod player;

use macroquad::prelude::*;
use player::Player;

#[macroquad::main("RustSnake")]
async fn main() {
    let mut exited = false;

    let mut player = Player::new(0.0, 0.0, 32.0, 32.0, 32.0);
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

fn Draw(player: &Player) {
    clear_background(BLACK);
    player.Draw();
    draw_text("Hello World", 4.0, 4.0, 20f32, WHITE);
}

fn End() {

}