use nannou::prelude::*;

pub struct Mover {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub color: Hsv,
}

impl Mover {
    pub fn new(position: Vec2, mass: f32, color: Hsv) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass,
            radius: mass * 3.0,
            color,
        }
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration = Vec2::ZERO;
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .color(self.color)
            .stroke(BLACK)
            .stroke_weight(2.0)
            .radius(self.radius)
            .xy(self.position);
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }

    pub fn check_edges(&mut self, bounds: &Rect) {
        if self.position.x + self.radius < bounds.left() {
            self.position.x = bounds.right() + self.radius;
        } else if self.position.x - self.radius > bounds.right() {
            self.position.x = bounds.left() - self.radius;
        }

        if self.position.y - self.radius < bounds.bottom() {
            self.position.y = bounds.bottom() + self.radius;
            self.velocity.y *= -1.0;
        }
        if self.position.y + self.radius > bounds.top() {
            self.position.y = bounds.top() - self.radius;
            self.velocity.y *= -1.0;
        }
    }
}
