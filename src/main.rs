use core::time;

use macroquad::prelude::*;
use macroquad::input::get_keys_down;


const MOVEMENT_SPEED: f32 = 2000.0;
const PLAYER_SPEED: f32 = 200.0;
const MASS: f32 = 100.0;


struct Rocket {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,
    theta: f32
}

// TODO: figure out pivot on a point for thrusters
// TODO: 

#[macroquad::main("rocket")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;


    
    let mut rocket: Rocket = Rocket { x: (x), y: (y), vx: (0.0), vy: (0.0), ax: (0.0), ay: (0.0), theta: (0.0)};

    // spawn items and item state
        // background (stars)
        // rocket (square, with two trapezoids 3 units from middle, two lines diagonal from bottom as landing gear)
        // floor


    loop {

        clear_background(WHITE);
    

        // take input
            // get keys pressed

        // process input - eg. generate new state - acceleration, velocity, position
            // apply gravity
            // if w -> add a force in the current orientation of the rocket
            // if a -> add a force (like a lever) 3 units from the middle of the ship, towards the left
            // if d -> add a force (like a lever) 3 units from the middle of the ship, towrds the right
        
        // apply new state:
            // 

        // execute changes - apply new state to the items


        // validate "win condition" - if rocket is still for more than 3 seconds
            // in radius of desired landing position
            // in range of acceptable landing orientation

        // process_input(&mut rocket);

        new_process_input(&mut rocket);

        update_state(&mut rocket);

        draw_rocket(&mut rocket);

        next_frame().await;
    }
}



fn process_input(rocket: &mut Rocket) {

    if is_key_down(KeyCode::W) {
        rocket.y -= PLAYER_SPEED * rocket.theta.cos();
        rocket.x += PLAYER_SPEED * rocket.theta.sin();
    }
    if is_key_down(KeyCode::A) {
        rocket.theta -= PLAYER_SPEED * 0.01;
    }
    if is_key_down(KeyCode::S) {
        rocket.y += PLAYER_SPEED * rocket.theta.cos();
        rocket.x -= PLAYER_SPEED * rocket.theta.sin();
    }
    if is_key_down(KeyCode::D) {
        rocket.theta += PLAYER_SPEED * 0.01;
    }
}


fn new_process_input(rocket: &mut Rocket) {
    let delta_time = get_frame_time();

    if is_key_down(KeyCode::W) {
        rocket.ay -= delta_time * (PLAYER_SPEED * rocket.theta.cos()) / MASS;
        rocket.ax += delta_time * (PLAYER_SPEED * rocket.theta.sin()) / MASS;
    }
    if is_key_down(KeyCode::A) {
        rocket.theta -= delta_time * PLAYER_SPEED * 0.01;
    }
    if is_key_down(KeyCode::S) {
        rocket.ay += delta_time * (PLAYER_SPEED * rocket.theta.cos()) / MASS;
        rocket.ax -= delta_time * (PLAYER_SPEED * rocket.theta.sin() / MASS);
    }
    if is_key_down(KeyCode::D) {
        rocket.theta += delta_time * PLAYER_SPEED * 0.01;
    }
}

fn update_state(rocket: &mut Rocket) {
    let dt = 1.0;
    // let delta_time = 1.0;

    rocket.vx = rocket.vx + rocket.ax * dt;
    rocket.vy = rocket.vy + rocket.ay * dt;

    rocket.x = rocket.vx * dt;
    rocket.y = rocket.vy * dt;

}



// so the idea is:
// we say given an orientation, add the player speed from a movement
    // decompose that into x and y components using sin and cos
    // given a movement F
    // the force in x is Fcos(theta)
    // the force in y is Fsin(theta)


fn draw_rocket(rocket: &mut Rocket) {
    let params: DrawRectangleParams = DrawRectangleParams { offset: ((0.5, 0.5)).into(), rotation: rocket.theta, color: (BLUE) };
    draw_rectangle_ex(rocket.x, rocket.y, 20.0, 40.0, params);
}