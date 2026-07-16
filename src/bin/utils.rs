use macroquad_particles::Emitter;

use macroquad::{prelude::*};
use macroquad_particles::{BlendMode::{self}, ColorCurve, EmitterConfig};



#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {

    pub fn new(x: f32, y: f32) -> Self { 
        Self { x, y } 
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RotationalState {
    pub theta: f32,
    pub vtheta: f32,
    pub atheta: f32
}

impl RotationalState {

    pub fn new(theta: f32, vtheta: f32, atheta: f32) -> RotationalState{
        Self {theta, vtheta, atheta}
    }

}

pub struct Rocket {
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub acceleration: Vector2D,
    pub rotation: RotationalState
}

impl Rocket {
    pub fn new(position: Vector2D, velocity: Vector2D, acceleration: Vector2D, rotation: RotationalState) -> Rocket {
        Self {position, velocity, acceleration, rotation}
    }
}

pub async fn create_booster_emitters() -> (Emitter, Emitter, Texture2D) {
    let exhaust_texture = load_texture("assets/exhaust.png").await.unwrap();
    let rocket_texture = load_texture("assets/rocket.png").await.unwrap();

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

    (engine_emitter, attitude_emitter, rocket_texture)
}

fn main() {
    unimplemented!()
}

