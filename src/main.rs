use core::time;

use macroquad::prelude::*;
use macroquad::input::get_keys_down;


const MOVEMENT_SPEED: f32 = 2000.0;
const PLAYER_SPEED: f32 = 220.0;
const THRUST: f32 = 2.0;
const TURN_SPEED: f32 = 2.0;
const MASS: f32 = 100.0;
const GRAVITY: f32 = 0.008;


struct Rocket {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,
    theta: f32,
    vtheta: f32,
    atheta: f32
}

// TODO: figure out pivot on a point for thrusters
// TODO: gravity
// TODO: need a brief menu / keyboard control to reset the game
// TODO: would be cool to measure how good of a landing we had (eg. angle, speed)
// TODO: spawn in the rocket with a random downward speed and random orientation (between -90 and 90 degrees)
// TODO: autopilot

#[macroquad::main("rocket")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;


    
    let mut rocket: Rocket = Rocket { x: (x), y: (y), vx: (0.0), vy: (0.0), ax: (0.0), ay: (0.0), theta: (0.0), vtheta: (0.0), atheta: (0.0)};

    // spawn items and item state
        // background (stars)
        // rocket (square, with two trapezoids 3 units from middle, two lines diagonal from bottom as landing gear)
        // floor


    loop {

        clear_background(WHITE);

        new_process_input(&mut rocket);

        add_gravity(&mut rocket);

        update_state(&mut rocket);

        draw_rocket(&mut rocket);

        next_frame().await;
    }
}



fn new_process_input(rocket: &mut Rocket) {
    let delta_time = 1.0;

    if is_key_down(KeyCode::W) {
        rocket.ay -= delta_time * (THRUST * rocket.theta.cos()) / MASS;
        rocket.ax += delta_time * (THRUST * rocket.theta.sin()) / MASS;
    }
    if is_key_down(KeyCode::A) {
        rocket.atheta -= delta_time * TURN_SPEED * 0.0001;
    }
    if is_key_down(KeyCode::S) {
        rocket.ay += delta_time * (THRUST * rocket.theta.cos()) / MASS;
        rocket.ax -= delta_time * (THRUST * rocket.theta.sin() / MASS);
    }
    if is_key_down(KeyCode::D) {
        rocket.atheta += delta_time * TURN_SPEED * 0.0001;
    }
}

fn add_gravity(rocket: &mut Rocket) {
    let dt: f32 = 1.0;
    rocket.ay += dt * GRAVITY;
}

fn update_state(rocket: &mut Rocket) {
    let dt = 1.0;

    rocket.vx += rocket.ax * dt;
    rocket.vy += rocket.ay * dt;
    rocket.vtheta += rocket.atheta * dt;

    rocket.x += rocket.vx * dt;
    rocket.y += rocket.vy * dt;
    rocket.theta += rocket.vtheta * dt;

    rocket.ax = 0.0;
    rocket.ay = 0.0;
    rocket.atheta = 0.0;

}



fn draw_rocket(rocket: &mut Rocket) {
    let params: DrawRectangleParams = DrawRectangleParams { offset: ((0.5, 0.5)).into(), rotation: rocket.theta, color: (BLUE) };
    draw_rectangle_ex(rocket.x, rocket.y, 20.0, 40.0, params);
}