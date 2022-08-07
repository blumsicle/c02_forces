use nannou::prelude::*;

use crate::mover::Mover;

pub struct Attractor {
    pub position: Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Attractor {
    const G: f32 = 10.0;

    pub fn new(position: Vec2, mass: f32) -> Self {
        Self {
            position,
            mass,
            radius: mass * 3.0,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .color(hsv(0.01, 0.8, 0.6))
            .stroke(BLACK)
            .stroke_weight(2.0)
            .radius(self.radius)
            .xy(self.position);
    }

    pub fn attract(&self, mover: &Mover) -> Vec2 {
        let direction = self.position - mover.position;
        let distance = direction.length().clamp(20.0, 100.0);
        let norm = direction.normalize_or_zero();
        let strength = (Self::G * self.mass * mover.mass) / (distance * distance);
        let force = norm * strength;

        force
    }
}
