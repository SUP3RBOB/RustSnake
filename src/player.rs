use macroquad::color::{ORANGE, RED, WHITE};
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text;

pub struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    x_dir: i32,
    y_dir: i32,
    speed: f32,
    tails: Vec<(f32, f32)>,
    highscore: i32,
}

impl Player {
    pub fn new(x_pos: f32, y_pos: f32, w: f32, h: f32, spd: f32) -> Player {
        return Player {
            x: x_pos,
            y: y_pos,
            width: w,
            height: h,
            x_dir: 1,
            y_dir: 0,
            speed: spd,
            tails: Vec::new(),
            highscore: 0,
        }
    }

    pub fn get_direction(&self) -> (i32, i32) {
        return (self.x_dir, self.y_dir);
    }

    pub fn set_direction(&mut self, x: i32, y: i32) {
        self.x_dir = x;
        self.y_dir = y;
    }

    pub fn move_player(&mut self) {
        let prev_pos = (self.x, self.y);

        self.x += (self.x_dir as f32) * self.speed;
        self.y += (self.y_dir as f32) * self.speed;

        if (self.tails.len() <= 0) {
            return;
        }

        let tail_count = self.tails.len();

        let mut i = tail_count - 1;
        while (i > 0) {
            self.tails[i] = self.tails[i - 1];
            i -= 1;
        }

        self.tails[0] = prev_pos;
    }

    pub fn set_position(&mut self, x_pos: f32, y_pos: f32) {
        self.x = x_pos;
        self.y = y_pos;
    }

    pub fn position(&self) -> (f32, f32) {
        return (self.x, self.y);
    }

    pub fn add_to_tail(&mut self) {
        let mut tail_x: f32;
        let mut tail_y: f32;
        let tail_count = self.tails.len();

        if (tail_count <= 0) {
            tail_x = self.x + -(self.x_dir * 32) as f32;
            tail_y = self.y + -(self.y_dir * 32) as f32;
        } else {
            tail_x = self.tails[tail_count - 1].0 + -(self.x_dir * 32) as f32;
            tail_y = self.tails[tail_count - 1].1 + -(self.y_dir * 32) as f32;
        }

        self.tails.push((tail_x, tail_y));
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, RED);
        draw_text(format!("Score: {}", self.tails.len()).as_str(), 15.0, 20.0, 20.0, WHITE);
        draw_text(format!("Highscore: {}", self.highscore).as_str(), 16.0, 40.0, 20.0, WHITE);

        if (self.tails.len() <= 0) {
            return;
        }

        for tail in self.tails.iter() {
            draw_rectangle(tail.0, tail.1, self.width, self.height, ORANGE);
        }
    }

    pub fn is_out_of_bounds(&self) -> bool {
        if (self.x < 0.0 || self.x >= 512.0 || self.y < 0.0 || self.y >= 512.0) {
            return true;
        }

        return false;
    }

    pub fn is_touching_tail(&self) -> bool {
        for tail in self.tails.iter() {
            if (self.x, self.y) == (*tail) {
                return true;
            }
        }

        return false;
    }

    pub fn clear_tails(&mut self) {
        self.tails.clear();
    }

    pub fn get_highscore(&self) -> i32 {
        return self.highscore;
    }

    pub fn set_highscore(&mut self, score: i32) {
        self.highscore = score;
    }

    pub fn get_tail_count(&self) -> i32 {
        return self.tails.len() as i32;
    }
}