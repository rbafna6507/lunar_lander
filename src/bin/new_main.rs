mod utils;
mod game;

use std::vec;
use macroquad::{prelude::*};
use macroquad_particles::{BlendMode::{self, Additive}, ColorCurve, Emitter, EmitterConfig};


use crate::utils::{Rocket, Vector2D};


const MOVEMENT_SPEED: f32 = 100.0;
const PLAYER_SPEED: f32 = 220.0;
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


// TODO: could do force (thrust) as a variable of rocket state (eg. mass)
    // TODO: extra credit lol
// TODO: collisions (for landing)
    // TODO: if speed < 3 and collides with ground, land successful
    // TODO: else: failure
// TODO: particles
    // TODO: thrust particles
    // TODO: angular thrust particles
// TODO: scoring a landing
    // TODO: 0-1 speed is quite good, as close to 0 degrees as possible is good
    // TODO: score is a function of speed and theta

// TODO: draw trajectory line

// TODO: PID system that balances speed + orientation to land rocket automatically

#[macroquad::main(window_conf)]
async fn main() {

    let x = screen_width() / 2.0;
    let y = screen_height() / 2.0;

    let mut rocket: utils::Rocket = utils::Rocket::new(
        utils::Vector2D::new(x, y),
        utils::Vector2D::new(0.0, 0.0),
        utils::Vector2D::new(0.0, 0.0),
        utils::RotationalState::new(0.0, 0.0, 0.0)
    );

    let exhaust_texture = load_texture("assets/exhaust.png").await.unwrap();
    let mut rocket_texture = load_texture("assets/rocket.png").await.unwrap();

    rocket_texture.set_filter(FilterMode::Nearest);
    exhaust_texture.set_filter(FilterMode::Linear);

    // 2. build the emitter config
    let engine_config = EmitterConfig {
        texture: Some(exhaust_texture),
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

    let mut engine_emitter = Emitter::new(engine_config);

    // spawn items and item state
        // background (stars)
        // rocket (square, with two trapezoids 3 units from middle, two lines diagonal from bottom as landing gear)
        // floor
    clear_background(WHITE);

    // main simulation loop
    loop {
        process_input(&mut rocket, &mut engine_emitter);
        engine_emitter.draw(Vec2::ZERO);

        update_rocket_state(&mut rocket);

        draw_rocket(&mut rocket, &mut rocket_texture);

        next_frame().await;

    }

}

fn process_input(rocket: &mut Rocket, engine_emitter: &mut Emitter) {
    let delta_time = 1.0;

    if is_key_down(KeyCode::W) {
        rocket.acceleration.y -= delta_time * (THRUST * rocket.rotation.theta.cos()) / MASS;
        rocket.acceleration.x += delta_time * (THRUST * rocket.rotation.theta.sin()) / MASS;

        engine_emitter.config.initial_direction = Vec2 { x: -rocket.rotation.theta.sin(), y: rocket.rotation.theta.cos() };
        engine_emitter.emit(Vec2 { x: rocket.position.x - rocket.rotation.theta.sin() * ROCKET_HEIGHT/2.0, y: rocket.position.y + rocket.rotation.theta.cos() * ROCKET_HEIGHT/2.0 }, 100);
    }

    if is_key_down(KeyCode::A) {
        rocket.rotation.atheta -= delta_time * TURN_SPEED;
    }

    if is_key_down(KeyCode::D) {
        rocket.rotation.atheta += delta_time * TURN_SPEED;
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

fn draw_particles(rocket: &mut Rocket) {
    unimplemented!()
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
        draw_line(points[i].x, points[i].y, points[i+1].x, points[i+1].y, 1.0, WHITE);
    }

}

