use macroquad::{prelude::*, rand::{self, srand}};

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

    pub fn Position(&self) -> (f32, f32) {
        return (self.x, self.y);
    }

    pub fn JumpToRandomPosition(&mut self) {
        let xb = screen_width() as i32 / 32;
        let yb = screen_height() as i32 / 32;


        srand(get_time() as u64);
        let newX = rand::RandomRange::gen_range(0, xb) * 32;
        let newY = rand::RandomRange::gen_range(0, yb) * 32;

        self.x = newX as f32;
        self.y = newY as f32;
    }

    pub fn Draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, GREEN);

        
    }
}