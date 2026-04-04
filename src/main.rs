use core::time;

use macroquad::prelude::*;
use macroquad::input::get_keys_down;


const MOVEMENT_SPEED: f32 = 100.0;
const PLAYER_SPEED: f32 = 220.0;
const THRUST: f32 = 2.0;
const TURN_SPEED: f32 = 2.0;
const MASS: f32 = 100.0;
const GRAVITY: f32 = 0.008;
const GROUND_HEIGHT: f32 = 100.0;


struct Rocket {
    w: f32,
    h: f32,

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
// TODO: need a brief menu / keyboard control to reset the game
// TODO: would be cool to measure how good of a landing we had (eg. angle, speed)
// TODO: spawn in the rocket with a random downward speed and random orientation (between -90 and 90 degrees)
// TODO: autopilot


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
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;


    
    let mut rocket: Rocket = Rocket { 
        x: (x), 
        y: (y), 
        vx: (0.0), 
        vy: (0.0), 
        ax: (0.0), 
        ay: (0.0), 
        theta: (0.0), 
        vtheta: (0.0), 
        atheta: (0.0), 
        w: (20.0), 
        h: (40.0)
    };

    // spawn items and item state
        // background (stars)
        // rocket (square, with two trapezoids 3 units from middle, two lines diagonal from bottom as landing gear)
        // floor
    clear_background(WHITE);
    let ground_y = screen_height() - GROUND_HEIGHT;


    loop {

        // clear_background(WHITE);

        draw_rectangle(0.0, ground_y, screen_width(), GROUND_HEIGHT, GRAY);

        new_process_input(&mut rocket);

        add_gravity(&mut rocket);

        update_state(&mut rocket);


        // if touching the ground and too fast, exit and say you crashed
        if (is_crashed(&mut rocket) ) {
            return ;
        }
        // if touching the ground and at acceptable speed, let linger for two seconds, take the velocity at that time and score it

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
    let dt = get_frame_time() * MOVEMENT_SPEED;

    rocket.vx += rocket.ax * dt;
    rocket.vy += rocket.ay * dt;
    rocket.vtheta += rocket.atheta * dt;

    rocket.x += rocket.vx * dt;
    rocket.y += rocket.vy * dt;
    rocket.theta += rocket.vtheta * dt;

    rocket.x = rocket.x % screen_width();
    rocket.y = rocket.y % screen_height();

    rocket.ax = 0.0;
    rocket.ay = 0.0;
    rocket.atheta = 0.0;

}

fn is_crashed(rocket: &mut Rocket) -> bool {
    let r: Rect = Rect { x: rocket.x, y: rocket.y - 20.0, w: (rocket.w), h: (rocket.h) };
    let g: Rect = Rect { x: 0.0, y:screen_height() - GROUND_HEIGHT, w:screen_width(), h: GROUND_HEIGHT };

    r.overlaps(&g)
}



fn draw_rocket(rocket: &mut Rocket) {
    let params: DrawRectangleParams = DrawRectangleParams { offset: ((0.5, 0.5)).into(), rotation: rocket.theta, color: (BLUE) };
    draw_rectangle_ex(rocket.x, rocket.y, rocket.w, rocket.h, params);
}