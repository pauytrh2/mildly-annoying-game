use macroquad::{
    color::{DARKGRAY, RED, WHITE},
    prelude::{Vec2, vec2},
    text::draw_text,
    texture::{Texture2D, draw_texture},
    window::{screen_height, screen_width},
};

pub const ACCELERATION: f32 = 0.5;
pub const MAX_SPEED: f32 = 10.0;
pub const FRICTION: f32 = 0.2;
pub const ENEMY_SPAWN_INTERVAL: f32 = 2.0;
pub const PLAYER_WIDTH: f32 = 50.0;
pub const PLAYER_HEIGHT: f32 = 50.0;

pub enum GameState {
    Playing,
    GameOver,
}

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

pub fn draw_background(
    texture: &Texture2D,
    world_offset_x: f32,
    world_offset_y: f32,
    screen_width: f32,
    screen_height: f32,
) {
    let tex_width = texture.width();
    let tex_height = texture.height();

    let offset_x = -world_offset_x.rem_euclid(tex_width);
    let offset_y = -world_offset_y.rem_euclid(tex_height);

    let tiles_x = (screen_width / tex_width).ceil() as i32 + 1;
    let tiles_y = (screen_height / tex_height).ceil() as i32 + 1;

    for y in 0..tiles_y {
        for x in 0..tiles_x {
            draw_texture(
                &texture,
                offset_x + x as f32 * tex_width,
                offset_y + y as f32 * tex_height,
                WHITE,
            );
        }
    }
}

pub fn game_over_screen() {
    draw_text(
        "GAME OVER",
        screen_width() / 2.0 - 120.0,
        screen_height() / 2.0,
        48.0,
        RED,
    );
    draw_text(
        "Press any key or click to restart",
        screen_width() / 2.0 - 200.0,
        screen_height() / 2.0 + 50.0,
        24.0,
        DARKGRAY,
    );
}