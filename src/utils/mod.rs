use macroquad::prelude::{Vec2, vec2};

pub fn rotate_point(p: Vec2, angle: f32) -> Vec2 {
    let sin = angle.sin();
    let cos = angle.cos();
    vec2(p.x * cos - p.y * sin, p.x * sin + p.y * cos)
}

pub fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let max = std::f32::consts::TAU;
    let delta = ((b - a + max / 2.0) % max) - max / 2.0;
    a + delta * t
}

pub enum GameState {
    Playing,
    GameOver,
}