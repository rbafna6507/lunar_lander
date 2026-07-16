mod utils;
mod game;

use std::vec;
use macroquad::{prelude::*};
use macroquad_particles::{BlendMode::{self, Additive}, ColorCurve, Emitter, EmitterConfig};


use crate::utils::{Rocket, Vector2D};


const MOVEMENT_SPEED: f32 = 75.0;
const THRUST: f32 = 2.3;
const TURN_SPEED: f32 = 0.0003;
const MASS: f32 = 100.0;
const GRAVITY: f32 = 0.008;
const GROUND_HEIGHT: f32 = 100.0;
const ROCKET_WIDTH: f32 = 30.0;
const ROCKET_HEIGHT: f32 = 60.0;


fn window_conf() -> Conf {
    Conf {
        window_title: "My Game".to_owned(),
        window_width: 1480,   // Preferred width
        window_height: 1020,   // Preferred height
        ..Default::default()
    }
}


// TODO: scoring a landing
    // TODO: 0-1 speed is quite good, as close to 0 degrees as possible is good
    // TODO: score is a function of speed and theta

// TODO: PID system that balances speed + orientation to land rocket automatically

enum GameState {
    Start,
    Playing,
    GameOver(String),
}


enum Landing {
    Success {score: u32},
    Crash {score: u32}
}


fn spawn_rocket() -> utils::Rocket {
    utils::Rocket::new(
        utils::Vector2D::new(screen_width() / 2.0, screen_height() / 2.0),
        utils::Vector2D::new(0.0, 0.0),
        utils::Vector2D::new(0.0, 0.0),
        utils::RotationalState::new(0.0, 0.0, 0.0),
    )
}

fn draw_centered_text(text: &str, y: f32, font_size: f32, color: Color) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    draw_text(text, (screen_width() - dims.width) / 2.0, y, font_size, color);
}

fn draw_start_screen() {
    draw_centered_text("ROCKET LANDER", screen_height() / 2.0 - 40.0, 70.0, WHITE);
    draw_centered_text("press SPACE to start", screen_height() / 2.0 + 30.0, 34.0, GRAY);
}

fn draw_game_over_screen(message: &str) {
    draw_centered_text(message, screen_height() / 2.0 - 30.0, 56.0, WHITE);
    draw_centered_text("SPACE to restart        Q to quit", screen_height() / 2.0 + 40.0, 32.0, GRAY);
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut rocket = spawn_rocket();

    let exhaust_texture = load_texture("assets/exhaust.png").await.unwrap();
    let mut rocket_texture = load_texture("assets/rocket.png").await.unwrap();

    rocket_texture.set_filter(FilterMode::Nearest);
    exhaust_texture.set_filter(FilterMode::Linear);

    // 2. build the emitter config
    let engine_config = EmitterConfig {
        texture: Some(exhaust_texture.clone()),
        blend_mode: BlendMode::Additive,
        colors_curve: ColorCurve {
            start: YELLOW,
            mid:   Color::new(1.0, 0.5, 0.5, 0.5),
            end:   Color::new(1.0, 1.0, 1.0, 0.0),
        },
        lifetime: 0.6,
        lifetime_randomness: 0.4,
        amount: 10,
        size: 2.0,
        initial_direction_spread: 1.5,
        initial_velocity: 200.0,
        initial_velocity_randomness: 0.9,
        emitting: false,
        local_coords: false,
        ..Default::default()
    };

    let side_attitude_config = EmitterConfig {
        texture: Some(exhaust_texture),
        blend_mode: BlendMode::Additive,
        colors_curve: ColorCurve {
            start: WHITE,
            mid:   WHITE,
            end:   GRAY,
        },
        lifetime: 0.1,
        lifetime_randomness: 0.2,
        amount: 5,
        size: 2.0,
        initial_direction_spread: 0.5,
        initial_velocity: 200.0,
        initial_velocity_randomness: 0.3,
        emitting: false,
        local_coords: false,
        ..Default::default()
    };

    let mut engine_emitter = Emitter::new(engine_config);
    let mut attitude_emitter = Emitter::new(side_attitude_config);

    let mut state = GameState::Start;

    // main game loop
    loop {
        clear_background(BLACK);

        let mut next_state: Option<GameState> = None;

        match &state {
            GameState::Start => {
                draw_start_screen();
                if is_key_pressed(KeyCode::Space) {
                    rocket = spawn_rocket();
                    next_state = Some(GameState::Playing);
                }
            }
            GameState::Playing => {
                process_input(&mut rocket, &mut engine_emitter, &mut attitude_emitter);
                engine_emitter.draw(Vec2::ZERO);
                attitude_emitter.draw(Vec2::ZERO);

                update_rocket_state(&mut rocket);

                draw_rocket(&mut rocket, &mut rocket_texture);

                if let Some(message) = handle_collisions(&rocket) {
                    next_state = Some(GameState::GameOver(message));
                }
            }
            GameState::GameOver(message) => {
                draw_game_over_screen(message);
                if is_key_pressed(KeyCode::Space) {
                    rocket = spawn_rocket();
                    next_state = Some(GameState::Playing);
                }
                if is_key_pressed(KeyCode::Q) {
                    break;
                }
            }
        }

        if let Some(s) = next_state {
            state = s;
        }

        next_frame().await;
    }
}

fn process_input(rocket: &mut Rocket, engine_emitter: &mut Emitter, side_emitter: &mut Emitter) {
    let delta_time = 1.0;

    if is_key_down(KeyCode::W) {
        rocket.acceleration.y -= delta_time * (THRUST * rocket.rotation.theta.cos()) / MASS;
        rocket.acceleration.x += delta_time * (THRUST * rocket.rotation.theta.sin()) / MASS;

        engine_emitter.config.initial_direction = Vec2 { x: -rocket.rotation.theta.sin(), y: rocket.rotation.theta.cos() };
        engine_emitter.emit(Vec2 { x: rocket.position.x - rocket.rotation.theta.sin() * ROCKET_HEIGHT/2.0, y: rocket.position.y + rocket.rotation.theta.cos() * ROCKET_HEIGHT/2.0 }, 100);
    }

    if is_key_down(KeyCode::A) {
        rocket.rotation.atheta -= delta_time * TURN_SPEED;

        let side_thrust_position = Vec2 { 
            x: rocket.position.x + (ROCKET_WIDTH/3.0 * rocket.rotation.theta.cos() + ROCKET_HEIGHT/4.0 * rocket.rotation.theta.sin()), 
            y: rocket.position.y + (ROCKET_WIDTH/3.0 * rocket.rotation.theta.sin() - ROCKET_HEIGHT/4.0 * rocket.rotation.theta.cos())
        };
        side_emitter.config.initial_direction = Vec2 { x: rocket.rotation.theta.cos(), y: rocket.rotation.theta.sin() };
        side_emitter.emit(side_thrust_position, 10);
    }

    if is_key_down(KeyCode::D) {
        rocket.rotation.atheta += delta_time * TURN_SPEED;

        let side_thrust_position = Vec2 { 
            x: rocket.position.x - (ROCKET_WIDTH/3.0 * rocket.rotation.theta.cos() - ROCKET_HEIGHT/4.0 * rocket.rotation.theta.sin()), 
            y: rocket.position.y - (ROCKET_WIDTH/3.0 * rocket.rotation.theta.sin() + ROCKET_HEIGHT/4.0 * rocket.rotation.theta.cos())
        };
        side_emitter.config.initial_direction = Vec2 { x: -rocket.rotation.theta.cos(), y: -rocket.rotation.theta.sin() };
        side_emitter.emit(side_thrust_position, 10);
    }

    if is_key_down(KeyCode::S) {
        rocket.acceleration.y += delta_time * (THRUST * rocket.rotation.theta.cos()) / MASS;
        rocket.acceleration.x -= delta_time * (THRUST * rocket.rotation.theta.sin()) / MASS;
    }

    rocket.acceleration.y += GRAVITY;
}

fn update_rocket_state(rocket: &mut Rocket) {
    let dt = get_frame_time() * MOVEMENT_SPEED;

    // update rocket velocity state
    rocket.velocity.x += rocket.acceleration.x * dt;
    rocket.velocity.y += rocket.acceleration.y * dt;
    
    // update rocket position
    rocket.position.x += rocket.velocity.x * dt;
    rocket.position.y += rocket.velocity.y * dt;

    // update rocket rotational velocity and angle
    rocket.rotation.vtheta += rocket.rotation.atheta * dt;
    rocket.rotation.theta += rocket.rotation.vtheta * dt;

    // Allow rocket x axis wrap-around
    rocket.position.x = rocket.position.x % screen_width();

    // reset acceleration values
    rocket.acceleration.x = 0.0;
    rocket.acceleration.y = 0.0;
    rocket.rotation.atheta = 0.0;
}

fn get_magnitude(x: f32, y:f32) -> f32 {
    (x.powf(2.0) + y.powf(2.0)).sqrt()
}



fn handle_collisions(rocket: &Rocket) -> Option<String> {
    // TODO:
        // need to keep track of left foot
        // need to keep track of right foot
        // need to keep track of tip of rocket

    let x = rocket.rotation.theta.cos() * ROCKET_WIDTH/2.0;
    let y = rocket.rotation.theta.sin() * ROCKET_HEIGHT / 2.0;



    

    // if any of those points are at or below the ground, collision
        // if speed is > 8 - failed landing
            // calculate score
            // display loss screen

        // if speed is <= 8 - successful landing
        // and abs(theta) <= 20
            // calculate score
            // display you landed but got a ___ score

    // TEMP: delete this once your 3-point detection above is wired in.
    // It only exists so the Start/GameOver screens are testable end-to-end;
    // your real detection should return Some(message) on a landing/crash.
    let ground_y = screen_height() - GROUND_HEIGHT;
    if rocket.position.y + ROCKET_HEIGHT / 2.0 >= ground_y {
        return Some("not quite a pretty landing".to_string());
    }

    None

}

fn pid(rocket: &mut Rocket) {
    
}

fn draw_rocket(rocket: &mut Rocket, rocket_texture: &mut Texture2D) {

    // Add Ground
    let ground_y: f32 = screen_height() - GROUND_HEIGHT;
    draw_rectangle(0.0, ground_y, screen_width(), GROUND_HEIGHT, GRAY);

    draw_text(&format!("Speed: {}", get_magnitude(rocket.velocity.x, rocket.velocity.y) * 9.5), 20.0, 50.0, 30.0, WHITE);

    let params = DrawTextureParams { dest_size: Some(vec2(ROCKET_WIDTH, ROCKET_HEIGHT)), rotation: rocket.rotation.theta, ..Default::default() };
    draw_texture_ex(&rocket_texture, rocket.position.x - ROCKET_WIDTH / 2.0, rocket.position.y - ROCKET_HEIGHT / 2.0, WHITE, params);

    draw_trajectory(rocket);
}

fn draw_trajectory(rocket: &Rocket) {
    let mut points: Vec<Vec2> = vec![];
    let bound: f32 = screen_height() - GROUND_HEIGHT;

    let mut proj_ax: f32 = rocket.acceleration.x;
    let mut proj_ay: f32 = rocket.acceleration.y;

    let mut proj_vx: f32 = rocket.velocity.x;
    let mut proj_vy: f32 = rocket.velocity.y;

    let mut proj_y: f32 = rocket.position.y;
    let mut proj_x: f32 = rocket.position.x;

    points.push(vec2(proj_x, proj_y));

    while proj_y < bound {
        // apply forces (just graity)
        proj_ay += GRAVITY;

        // update projected velocities
        proj_vx += proj_ax;
        proj_vy += proj_ay;

        // update projected x/y positions
        proj_x += proj_vx;
        proj_y += proj_vy;

        // reset acceleration for next projection
        proj_ay = 0.0;
        proj_ax = 0.0;

        // store them
        points.push(vec2(proj_x, proj_y));
    }

    // plot the vector of vec2s
    for i in 0..points.len()-1 {
        draw_line(points[i].x, points[i].y, points[i+1].x, points[i+1].y, 1.0, GRAY);
    }

}

