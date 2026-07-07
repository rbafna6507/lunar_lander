mod new_main;

use std::vec;

use macroquad::{prelude::*};


const MOVEMENT_SPEED: f32 = 100.0;
const PLAYER_SPEED: f32 = 220.0;
const THRUST: f32 = 2.0;
const TURN_SPEED: f32 = 3.0;
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


struct Particle {
    pos: Vec2,
    vel: Vec2,
    theta: f32,
    alpha: f32,
    size: f32,
}


// TODO: need to fix the 
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
    let x = screen_width() / 2.0;
    let y = screen_height() / 2.0;


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


fn process_input(rocket: &mut Rocket, particles: &mut Vec<Particle>) {
    let delta_time = 1.0;

    if is_key_down(KeyCode::W) {
        rocket.ay -= delta_time * (THRUST * rocket.theta.cos()) / MASS;
        rocket.ax += delta_time * (THRUST * rocket.theta.sin()) / MASS;

        
        // for _ in 0..5 {
        //     particles.push(Particle {
        //         pos: vec2(rocket.x - rocket.theta.sin() * rocket.h/2.0 + rand::gen_range(-0.8, 0.8), rocket.y + rocket.theta.cos() * rocket.h/2.0),
        //         vel: vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(2.0, 4.0)),
        //         theta: rocket.theta,
        //         alpha: 1.0,
        //         size: rand::gen_range(2.0, 5.0),
        //     });
        // }

        for _ in 0..5 {

            let mut p: Particle = Particle {pos: vec2(rocket.x - rocket.theta.sin() * rocket.h/2.0 + rand::gen_range(-0.8, 0.8), rocket.y + rocket.theta.cos() * rocket.h/2.0),
                vel: vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(2.0, 4.0)),
                theta: rocket.theta,
                alpha: 1.0,
                size: rand::gen_range(2.0, 5.0),};

            let unit_vector_angle: Vec2 = vec2(-p.theta.sin(), p.theta.cos());
            let magnitude: f32 = get_magnitude(p.vel.x, p.vel.y);

            // let mut new_vel: Vec2 = unit_vector_angle * magnitude;
            let mut new_vel: Vec2 = unit_vector_angle * vec2(rand::gen_range(-2.0, 0.0), rand::gen_range(1.5, 2.0));

            p.vel = new_vel;

            // let unit_vector_angle: Vec2 = vec2(-p.theta.sin(), p.theta.cos());
            // let magnitude: f32 = get_magnitude(p.vel.x, p.vel.y);

            // // let mut new_vel: Vec2 = unit_vector_angle * magnitude;
            // let mut new_vel: Vec2 = unit_vector_angle * vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(1.5, 2.0));

            // p.vel = new_vel;

            particles.push(p);

        }
    }
    if is_key_down(KeyCode::A) {
        rocket.atheta -= delta_time * TURN_SPEED * 0.0001;

        // use a rotation matrix to get x and y position of the particle when rotated
        // add the current x and y position of the rocket to get correct position

        // let mut rotated_x = -10.0 * rocket.theta.cos() +  10.0 * rocket.theta.sin();
        // let mut rotated_y: f32 = -10.0 * rocket.theta.sin() + -10.0 * rocket.theta.cos();

        // for _ in 0..5 {
        //     particles.push(Particle {
        //         pos: vec2(rotated_x + rocket.x, rotated_y + rocket.y),
        //         vel: vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(2.0, 4.0)),
        //         theta: rocket.theta,
        //         alpha: 1.0,
        //         size: rand::gen_range(2.0, 5.0),
        //     });
        // }

    }
    if is_key_down(KeyCode::S) {
        rocket.ay += delta_time * (THRUST * rocket.theta.cos()) / MASS;
        rocket.ax -= delta_time * (THRUST * rocket.theta.sin() / MASS);
    }
    if is_key_down(KeyCode::D) {
        rocket.atheta += delta_time * TURN_SPEED * 0.0001;


        // use a rotation matrix to get x and y position of the particle when rotated
        // add the current x and y position of the rocket to get correct position

        // let mut rotated_x = 10.0 * rocket.theta.cos() +  10.0 * rocket.theta.sin();
        // let mut rotated_y: f32 = 10.0 * rocket.theta.sin() + -10.0 * rocket.theta.cos();

        // for _ in 0..5 {
        //     particles.push(Particle {
        //         pos: vec2(rotated_x + rocket.x, rotated_y + rocket.y),
        //         vel: vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(2.0, 4.0)),
        //         theta: rocket.theta,
        //         alpha: 1.0,
        //         size: rand::gen_range(2.0, 5.0),
        //     });
        // }
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
    let params: DrawRectangleParams = DrawRectangleParams { offset: ((0.5, 0.5)).into(), rotation: rocket.theta, color: (Color::new(125 as f32, 124 as f32, 124 as f32, 1.0)) };
    draw_rectangle_ex(rocket.x, rocket.y, rocket.w, rocket.h, params);
}


fn draw_particles(particles: &mut Vec<Particle>) {
    particles.retain_mut(|p| {

            let unit_vector_angle: Vec2 = vec2(-p.theta.sin(), p.theta.cos());
            let magnitude: f32 = get_magnitude(p.vel.x, p.vel.y);

            // let mut new_vel: Vec2 = unit_vector_angle * magnitude;
            let mut new_vel: Vec2 = unit_vector_angle * vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(1.5, 2.0));

            p.pos += p.vel;
            p.alpha -= 0.02;
            p.size -= 0.05;
            
            if p.alpha > 0.0 && p.size > 0.0 {
                draw_circle(p.pos.x, p.pos.y, p.size, Color::new(247.0/255.0, 0.0, 0.0, p.alpha));
                true
            } else {
                false
            }
        });
}


fn draw_trajectory(rocket: &Rocket) {
    let mut points: Vec<Vec2> = vec![];

    let bound: f32 = screen_height() - GROUND_HEIGHT;

    let mut proj_y: f32 = rocket.y;
    let mut proj_x: f32 = rocket.x;

    let mut proj_vx: f32 = rocket.vx;
    let mut proj_vy: f32 = rocket.vy;

    let mut proj_ax: f32 = rocket.ax;
    let mut proj_ay: f32 = rocket.ax;

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


fn display_state(rocket: &Rocket) {
    unimplemented!()

    // should display rocket state next to the rocket
    // green if under velocity criteria
    // red if over velocity/orientation criteria
}


fn get_magnitude(x: f32, y: f32) -> f32 {
    let x_squared: f32 = x.powi(2);
    let y_squared: f32 = y.powi(2);

    let val: f32 = x_squared + y_squared;
    val.sqrt()
}









