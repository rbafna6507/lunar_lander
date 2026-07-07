mod utils;
mod game;

use std::vec;
use macroquad::{prelude::*};

use crate::utils::Vector2D;


const MOVEMENT_SPEED: f32 = 100.0;
const PLAYER_SPEED: f32 = 220.0;
const THRUST: f32 = 2.0;
const TURN_SPEED: f32 = 3.0;
const MASS: f32 = 100.0;
const GRAVITY: f32 = 0.008;
const GROUND_HEIGHT: f32 = 100.0;


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
    let ground_y = screen_height() - GROUND_HEIGHT;
    let mut particles: Vec<Particle> = vec![];

    loop {

        // clear_background(WHITE);

        draw_rectangle(0.0, ground_y, screen_width(), GROUND_HEIGHT, GRAY);

        process_input(&mut rocket, &mut particles);

        add_gravity(&mut rocket);

        update_state(&mut rocket);

        draw_text(&format!("Speed: {}", get_magnitude(rocket.vx, rocket.vy)), 20.0, 50.0, 30.0, WHITE);

        draw_particles(&mut particles);

        draw_trajectory(&rocket);

        draw_rocket(&mut rocket);


        // if touching the ground and too fast, exit and say you crashed
        if (is_crashed(&mut rocket) && get_magnitude(rocket.vx, rocket.vy) >= 0.9) {
            break;
        }
        // if touching the ground and at acceptable speed, let linger for two seconds, take the velocity at after that time and score it

         next_frame().await;

    }

}