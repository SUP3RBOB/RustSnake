use macroquad::{prelude::*, rand};

pub struct Apple {
    x: f32,
    y: f32,
    w: f32,
    h: f32
}

impl Apple {
    pub fn new(xPos: f32, yPos: f32, width: f32, height: f32) -> Apple{
        return Apple {
            x: xPos,
            y: yPos,
            w: width,
            h: height,
        };
    }

    pub fn JumpToRandomPosition(&mut self) {
        let xb = screen_width() / 32f32;
        let yb = screen_height() / 32f32;

        let newX = rand::RandomRange::gen_range(0.0, xb) * 32f32;
        let newY = rand::RandomRange::gen_range(0.0, yb) * 32f32;

        self.x = newX;
        self.y = newY;
    }

    pub fn Draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, GREEN);
    }
}