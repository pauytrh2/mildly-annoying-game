use ::rand::{Rng, rng};
use macroquad::prelude::*;

struct Enemy {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    speed: f32,
}

struct Bullet {
    x: f32,
    y: f32,
    radius: f32,
    speed: f32,
    color: Color,
    direction_x: f32,
    direction_y: f32,
}

enum GameState {
    Playing,
    GameOver,
}

#[macroquad::main("Game")]
async fn main() {
    const PLAYER_WIDTH: f32 = 50.0;
    const PLAYER_HEIGHT: f32 = 50.0;

    let mut game_state = GameState::Playing;

    let mut world_offset_x: f32 = 0.0;
    let mut world_offset_y: f32 = 0.0;

    let mut vel_x: f32 = 0.0;
    let mut vel_y: f32 = 0.0;

    const ACCELERATION: f32 = 0.5;
    const MAX_SPEED: f32 = 10.0;
    const FRICTION: f32 = 0.2;

    let mut enemies: Vec<Enemy> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut arrow_angle: f32 = 0.0;
    let mut kills: u32 = 0;

    let mut spawn_timer: f32 = 0.0;
    const ENEMY_SPAWN_INTERVAL: f32 = 2.0;

    loop {
        let dt = get_frame_time();
        clear_background(LIGHTGRAY);

        let player_x: f32 = screen_width() / 2.0;
        let player_y: f32 = screen_height() / 2.0;

        match game_state {
            GameState::Playing => {
                spawn_timer += dt;

                let mouse_pos = mouse_position();
                let mouse_world_x = mouse_pos.0 + world_offset_x;
                let mouse_world_y = mouse_pos.1 + world_offset_y;

                let dir_x = mouse_world_x - (player_x + world_offset_x);
                let dir_y = mouse_world_y - (player_y + world_offset_y);
                let dist = (dir_x * dir_x + dir_y * dir_y).sqrt();

                if dist > 1.0 {
                    let norm_dir_x = dir_x / dist;
                    let norm_dir_y = dir_y / dist;

                    vel_x += norm_dir_x * ACCELERATION;
                    vel_y += norm_dir_y * ACCELERATION;
                }

                vel_x = vel_x.clamp(-MAX_SPEED, MAX_SPEED);
                vel_y = vel_y.clamp(-MAX_SPEED, MAX_SPEED);

                if dist <= 1.0 {
                    vel_x = (vel_x - vel_x.signum() * FRICTION).clamp(-MAX_SPEED, MAX_SPEED);
                    if vel_x.abs() < FRICTION {
                        vel_x = 0.0;
                    }
                    vel_y = (vel_y - vel_y.signum() * FRICTION).clamp(-MAX_SPEED, MAX_SPEED);
                    if vel_y.abs() < FRICTION {
                        vel_y = 0.0;
                    }
                }

                world_offset_x += vel_x;
                world_offset_y += vel_y;

                if spawn_timer >= ENEMY_SPAWN_INTERVAL {
                    spawn_timer = 0.0;
                    let mut rng = rng();

                    let safe_distance = 100.0;
                    let mut x;
                    let mut y;

                    loop {
                        let angle = rng.random_range(0.0..std::f32::consts::TAU);
                        let distance = rng.random_range(safe_distance..300.0);
                        let player_world_x = player_x + world_offset_x;
                        let player_world_y = player_y + world_offset_y;

                        x = player_world_x + distance * angle.cos();
                        y = player_world_y + distance * angle.sin();

                        let dx = x - player_world_x;
                        let dy = y - player_world_y;
                        if (dx * dx + dy * dy).sqrt() >= safe_distance {
                            break;
                        }
                    }

                    enemies.push(Enemy {
                        x,
                        y,
                        width: 30.0,
                        height: 30.0,
                        color: DARKPURPLE,
                        speed: 100.0,
                    });
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    let dir_x = mouse_world_x - (player_x + world_offset_x);
                    let dir_y = mouse_world_y - (player_y + world_offset_y);
                    let length = (dir_x * dir_x + dir_y * dir_y).sqrt();
                    let norm_dir_x = dir_x / length;
                    let norm_dir_y = dir_y / length;

                    bullets.push(Bullet {
                        x: player_x + world_offset_x,
                        y: player_y + world_offset_y,
                        radius: 5.0,
                        speed: 500.0,
                        color: BLACK,
                        direction_x: norm_dir_x,
                        direction_y: norm_dir_y,
                    });
                }

                for bullet in &mut bullets {
                    bullet.x += bullet.direction_x * bullet.speed * dt;
                    bullet.y += bullet.direction_y * bullet.speed * dt;
                }

                bullets.retain(|b| {
                    b.x >= world_offset_x
                        && b.x <= world_offset_x + screen_width()
                        && b.y >= world_offset_y
                        && b.y <= world_offset_y + screen_height()
                });

                bullets.retain(|bullet| {
                    let mut hit_enemy = false;
                    enemies.retain(|enemy| {
                        let enemy_rect = Rect::new(enemy.x, enemy.y, enemy.width, enemy.height);
                        let bullet_point = Vec2::new(bullet.x, bullet.y);
                        if enemy_rect.contains(bullet_point) {
                            hit_enemy = true;
                            kills += 1;
                            false
                        } else {
                            true
                        }
                    });
                    !hit_enemy
                });

                let player_rect = Rect::new(
                    player_x - PLAYER_WIDTH / 2.0,
                    player_y - PLAYER_HEIGHT / 2.0,
                    PLAYER_WIDTH,
                    PLAYER_HEIGHT,
                );

                for enemy in &enemies {
                    let enemy_rect = Rect::new(
                        enemy.x - world_offset_x,
                        enemy.y - world_offset_y,
                        enemy.width,
                        enemy.height,
                    );
                    if player_rect.overlaps(&enemy_rect) {
                        game_state = GameState::GameOver;
                    }
                }

                let player_world_x = player_x + world_offset_x;
                let player_world_y = player_y + world_offset_y;

                for enemy in &mut enemies {
                    let dx = player_world_x - enemy.x;
                    let dy = player_world_y - enemy.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance > 1.0 {
                        let dir_x = dx / distance;
                        let dir_y = dy / distance;

                        enemy.x += dir_x * enemy.speed * dt;
                        enemy.y += dir_y * enemy.speed * dt;
                    }
                }

                for enemy in &enemies {
                    draw_rectangle(
                        enemy.x - world_offset_x,
                        enemy.y - world_offset_y,
                        enemy.width,
                        enemy.height,
                        enemy.color,
                    );
                }

                for bullet in &bullets {
                    draw_circle(
                        bullet.x - world_offset_x,
                        bullet.y - world_offset_y,
                        bullet.radius,
                        bullet.color,
                    );
                }

                draw_rectangle(
                    player_x - PLAYER_WIDTH / 2.0,
                    player_y - PLAYER_HEIGHT / 2.0,
                    PLAYER_WIDTH,
                    PLAYER_HEIGHT,
                    RED,
                );

                draw_text(
                    "Move toward mouse pointer, Shoot with Left Mouse Button",
                    20.0,
                    40.0,
                    24.0,
                    DARKGRAY,
                );
                draw_text(
                    &format!("Enemies killed: {}", kills),
                    20.0,
                    70.0,
                    24.0,
                    DARKGRAY,
                );

                if let Some(closest_enemy) = enemies.iter().min_by(|a, b| {
                    let da = (a.x - (player_x + world_offset_x))
                        .hypot(a.y - (player_y + world_offset_y));
                    let db = (b.x - (player_x + world_offset_x))
                        .hypot(b.y - (player_y + world_offset_y));
                    da.partial_cmp(&db).unwrap()
                }) {
                    let to_enemy_x = closest_enemy.x - (player_x + world_offset_x);
                    let to_enemy_y = closest_enemy.y - (player_y + world_offset_y);

                    let target_angle = to_enemy_y.atan2(to_enemy_x);

                    arrow_angle = lerp_angle(arrow_angle, target_angle, 0.1);

                    let arrow_pos_x = player_x;
                    let arrow_pos_y = player_y - PLAYER_HEIGHT / 2.0 - 20.0;

                    let arrow_size = 15.0;

                    let p1 = Vec2::new(0.0, -arrow_size / 2.0);
                    let p2 = Vec2::new(arrow_size, 0.0);
                    let p3 = Vec2::new(0.0, arrow_size / 2.0);

                    let rp1 = rotate_point(p1, arrow_angle) + vec2(arrow_pos_x, arrow_pos_y);
                    let rp2 = rotate_point(p2, arrow_angle) + vec2(arrow_pos_x, arrow_pos_y);
                    let rp3 = rotate_point(p3, arrow_angle) + vec2(arrow_pos_x, arrow_pos_y);
                    draw_triangle(rp1, rp2, rp3, BLACK);
                } else {
                    let square_pos_x = player_x - 10.0;
                    let square_pos_y = player_y - PLAYER_HEIGHT / 2.0 - 30.0;
                    let square_size = 20.0;

                    draw_rectangle(square_pos_x, square_pos_y, square_size, square_size, BLACK);
                }

                let move_dir_angle = dir_y.atan2(dir_x);

                let tri_pos_x = 40.0;
                let tri_pos_y = screen_height() - 40.0;

                let tri_size = 20.0;

                let t1 = Vec2::new(0.0, -tri_size / 2.0);
                let t2 = Vec2::new(tri_size, 0.0);
                let t3 = Vec2::new(0.0, tri_size / 2.0);

                let rt1 = rotate_point(t1, move_dir_angle) + vec2(tri_pos_x, tri_pos_y);
                let rt2 = rotate_point(t2, move_dir_angle) + vec2(tri_pos_x, tri_pos_y);
                let rt3 = rotate_point(t3, move_dir_angle) + vec2(tri_pos_x, tri_pos_y);

                draw_triangle(rt1, rt2, rt3, BLUE);
            }

            GameState::GameOver => {
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

                if is_key_pressed(KeyCode::Space)
                    || is_mouse_button_pressed(MouseButton::Left)
                    || is_key_pressed(KeyCode::Enter)
                    || is_key_pressed(KeyCode::Escape)
                {
                    world_offset_x = 0.0;
                    world_offset_y = 0.0;
                    vel_x = 0.0;
                    vel_y = 0.0;
                    enemies.clear();
                    bullets.clear();
                    spawn_timer = 0.0;
                    arrow_angle = 0.0;
                    kills = 0;
                    game_state = GameState::Playing;
                }
            }
        }

        next_frame().await;
    }
}

fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let diff =
        (b - a + std::f32::consts::PI * 3.0) % (std::f32::consts::PI * 2.0) - std::f32::consts::PI;
    a + diff * t
}

fn rotate_point(p: Vec2, angle: f32) -> Vec2 {
    Vec2::new(
        p.x * angle.cos() - p.y * angle.sin(),
        p.x * angle.sin() + p.y * angle.cos(),
    )
}