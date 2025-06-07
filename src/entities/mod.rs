use macroquad::{
    color::Color,
    shapes::{draw_circle, draw_rectangle},
};

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub speed: f32,
    pub color: Color,
    pub direction_x: f32,
    pub direction_y: f32,
}

impl Bullet {
    pub fn update(&mut self, dt: f32) {
        self.x += self.direction_x * self.speed * dt;
        self.y += self.direction_y * self.speed * dt;
    }

    pub fn draw(&self, offset_x: f32, offset_y: f32) {
        draw_circle(
            self.x - offset_x,
            self.y - offset_y,
            self.radius,
            self.color,
        );
    }
}

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub speed: f32,
}

impl Enemy {
    pub fn update(&mut self, target_x: f32, target_y: f32, dt: f32) {
        let dx = target_x - self.x;
        let dy = target_y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > 1.0 {
            self.x += dx / dist * self.speed * dt;
            self.y += dy / dist * self.speed * dt;
        }
    }

    pub fn draw(&self, offset_x: f32, offset_y: f32) {
        draw_rectangle(
            self.x - offset_x,
            self.y - offset_y,
            self.width,
            self.height,
            self.color,
        );
    }
}