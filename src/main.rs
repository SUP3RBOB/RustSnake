mod player;

use macroquad::prelude::*;
use player::Player;

#[macroquad::main("RustSnake")]
async fn main() {
    let exited = false;

    let mut player = Player::new(0.0, 0.0, 32.0, 32.0, 1.0);
    let mut timer = 0f32;

    Start();

    while (!exited) {

        Update(&mut player);
        Draw(&player).await;
    }

    End();
}

fn Start() {

}

fn Update(player: &mut Player) {
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

    player.Move(1f32);
}

async fn Draw(player: &Player) {
    clear_background(BLACK);
    player.Draw();
    draw_text("Hello World", 4.0, 4.0, 20f32, WHITE);
    next_frame().await;
}

fn End() {

}