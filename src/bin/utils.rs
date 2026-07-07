

#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    x: f32,
    y: f32,
}

impl Vector2D {

    pub fn new(x: f32, y: f32) -> Self { 
        Self { x, y } 
    }

    pub fn x(&self) -> f32 { 
        self.x 
    }
    pub fn y(&self) -> f32 { 
        self.y 
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RotationalState {
    theta: f32,
    vtheta: f32,
    atheta: f32
}

impl RotationalState {

    pub fn new(theta: f32, vtheta: f32, atheta: f32) -> RotationalState{
        Self {theta, vtheta, atheta}
    }

    pub fn theta(&self) -> f32 {
        self.theta
    }

    pub fn vtheta(&self) -> f32 {
        self.vtheta
    }

    pub fn atheta(&self) -> f32 {
        self.atheta
    }

}

pub struct Rocket {
    position: Vector2D,
    velocity: Vector2D,
    acceleration: Vector2D,
    theta: RotationalState
}

impl Rocket {
    pub fn new(position: Vector2D, velocity: Vector2D, acceleration: Vector2D, theta: RotationalState) -> Rocket {
        Self {position, velocity, acceleration, theta}
    }

    pub fn position(&self) -> Vector2D {
        self.position
    }

    pub fn velocity(&self) -> Vector2D {
        self.velocity
    }

    pub fn acceleration(&self) -> Vector2D {
        self.acceleration
    }

    pub fn theta(&self) -> RotationalState {
        self.theta
    }
}

fn main() {
    unimplemented!()
}

