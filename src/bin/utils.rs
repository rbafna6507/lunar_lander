

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

fn main() {
    unimplemented!()
}

