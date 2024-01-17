use macroquad::{prelude::*, rand::{self, srand, RandomRange}};

pub struct Apple {
    x: f32,
    y: f32,
    w: f32,
    h: f32
}

impl Apple {
    pub fn new(x_pos: f32, y_pos: f32, width: f32, height: f32) -> Apple {
        return Apple {
            x: x_pos,
            y: y_pos,
            w: width,
            h: height,
        };
    }

    pub fn position(&self) -> (f32, f32) {
        return (self.x, self.y);
    }

    pub fn jump_to_random_position(&mut self) {
        let xb = screen_width() as i32 / 32;
        let yb = screen_height() as i32 / 32;


        srand(get_time() as u64);
        let new_x = RandomRange::gen_range(0, xb) * 32;
        let new_y = RandomRange::gen_range(0, yb) * 32;

        self.x = new_x as f32;
        self.y = new_y as f32;
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, GREEN);
    }
}