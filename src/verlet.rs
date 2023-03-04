
use crate::vector::Vec2;

pub struct VerletObject {
    pub position_current: Vec2,
    pub position_old: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
}

impl VerletObject {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position_current: Vec2 {x, y},
            position_old: Vec2 {x, y},
            acceleration: Vec2::zero(),
            radius: 50.0,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocity = self.position_current - self.position_old;
        self.position_old = self.position_current;
        self.position_current = self.position_current + velocity + self.acceleration * dt * dt;
        self.acceleration = Vec2::zero();
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acceleration += acc;
    }
}