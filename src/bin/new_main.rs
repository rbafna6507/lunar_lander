mod utils;
mod game;

use std::vec;
use macroquad::{prelude::*};

use crate::utils::{Rocket, Vector2D};


const MOVEMENT_SPEED: f32 = 100.0;
const PLAYER_SPEED: f32 = 220.0;
const THRUST: f32 = 2.3;
const TURN_SPEED: f32 = 0.0003;
const MASS: f32 = 100.0;
const GRAVITY: f32 = 0.008;
const GROUND_HEIGHT: f32 = 100.0;
const ROCKET_WIDTH: f32 = 20.0;
const ROCKET_HEIGHT: f32 = 40.0;


fn window_conf() -> Conf {
    Conf {
        window_title: "My Game".to_owned(),
        window_width: 1480,   // Preferred width
        window_height: 1020,   // Preferred height
        ..Default::default()
    }
}

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

    // spawn items and item state
        // background (stars)
        // rocket (square, with two trapezoids 3 units from middle, two lines diagonal from bottom as landing gear)
        // floor
    clear_background(WHITE);

    // simulation loop
    loop {
        process_input(&mut rocket);

        update_rocket_state(&mut rocket);

        draw_rocket(&mut rocket);

        next_frame().await;
    }

}

fn process_input(rocket: &mut Rocket) {
    let delta_time = 1.0;

    if is_key_down(KeyCode::W) {
        rocket.acceleration.y -= delta_time * (THRUST * rocket.rotation.theta.cos()) / MASS;
        rocket.acceleration.x += delta_time * (THRUST * rocket.rotation.theta.sin()) / MASS;
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

fn draw_rocket(rocket: &mut utils::Rocket) {

    // Add Ground
    let ground_y: f32 = screen_height() - GROUND_HEIGHT;
    draw_rectangle(0.0, ground_y, screen_width(), GROUND_HEIGHT, GRAY);

    draw_text(&format!("Speed: {}", get_magnitude(rocket.velocity.x, rocket.velocity.y) * 9.5), 20.0, 50.0, 30.0, WHITE);

    let params: DrawRectangleParams = DrawRectangleParams { offset: ((0.5, 0.5)).into(), rotation: rocket.rotation.theta, color: (Color::new(125 as f32, 124 as f32, 124 as f32, 1.0)) };
    draw_rectangle_ex(rocket.position.x, rocket.position.y, ROCKET_WIDTH, ROCKET_HEIGHT, params);
}

