use macroquad::color::RED;
use macroquad::shapes::draw_rectangle;

pub struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    xDir: i32,
    yDir: i32,
    speed: f32,
    tails: Vec<(f32, f32)>,
}

impl Player {
    pub fn new(xPos: f32, yPos: f32, w: f32, h: f32, spd: f32) -> Player {
        return Player {
            x: xPos,
            y: yPos,
            width: w,
            height: h,
            xDir: 1,
            yDir: 0,
            speed: spd,
            tails: Vec::new(),
        }
    }

    pub fn SetDirection(&mut self, x: i32, y: i32) {
        self.xDir = x;
        self.yDir = y;
    }

    pub fn Move(&mut self) {
        self.x += (self.xDir as f32) * self.speed;
        self.y += (self.yDir as f32) * self.speed;
    }

    pub fn Position(&self) -> (f32, f32) {
        return (self.x, self.y);
    }

    pub fn Draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, RED);
    }
}