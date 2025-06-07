use macroquad::{
    color::{RED, WHITE},
    prelude::{Vec2, vec2},
    text::draw_text,
    texture::{Texture2D, draw_texture},
    window::{screen_height, screen_width},
};
use rand::{Rng, rng};

use crate::entities::Enemy;

pub const ACCELERATION: f32 = 0.5;
pub const MAX_SPEED: f32 = 8.0;
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
        WHITE,
    );
}

pub fn spawn_enemies(
    elapsed_time: f32,
    player_x: f32,
    player_y: f32,
    world_offset_x: f32,
    world_offset_y: f32,
    enemies: &mut Vec<Enemy>,
    enemy_texture: Texture2D,
) {
    let mut rng = rng();

    let extra_enemies = (elapsed_time / 10.0).floor() as usize;
    let num_enemies_to_spawn = 1 + extra_enemies.min(5);

    for _ in 0..num_enemies_to_spawn {
        let safe_distance = 100.0;
        let mut x;
        let mut y;

        loop {
            let player_world_x = player_x + world_offset_x;
            let player_world_y = player_y + world_offset_y;

            let screen_w = screen_width();
            let screen_h = screen_height();

            let spawn_margin = 100.0;
            let spawn_area_min_x = player_world_x - screen_w / 2.0 - spawn_margin;
            let spawn_area_max_x = player_world_x + screen_w / 2.0 + spawn_margin;
            let spawn_area_min_y = player_world_y - screen_h / 2.0 - spawn_margin;
            let spawn_area_max_y = player_world_y + screen_h / 2.0 + spawn_margin;

            loop {
                x = rng.random_range(spawn_area_min_x..spawn_area_max_x);
                y = rng.random_range(spawn_area_min_y..spawn_area_max_y);

                let in_view_x =
                    x >= player_world_x - screen_w / 2.0 && x <= player_world_x + screen_w / 2.0;
                let in_view_y =
                    y >= player_world_y - screen_h / 2.0 && y <= player_world_y + screen_h / 2.0;

                if !(in_view_x && in_view_y) {
                    break;
                }
            }

            let dx = x - player_world_x;
            let dy = y - player_world_y;
            if (dx * dx + dy * dy).sqrt() >= safe_distance {
                break;
            }
        }

        enemies.push(Enemy {
            x,
            y,
            width: 40.0,
            height: 40.0,
            speed: 100.0,
            texture: enemy_texture.clone(),
        });
    }
}
