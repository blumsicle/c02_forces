use nannou::prelude::*;

pub struct Mover {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub color: Hsv,
}

impl Mover {
    pub fn new(position: Vec2, color: Hsv) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass: 1.0,
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
            .radius(8.0)
            .xy(self.position);
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }

    pub fn check_edges(&mut self, bounds: &Rect) {
        if self.position.x < bounds.left() {
            self.position.x = bounds.right();
        } else if self.position.x > bounds.right() {
            self.position.x = bounds.left();
        }

        if self.position.y < bounds.bottom() {
            self.position.y = bounds.top();
        } else if self.position.y > bounds.top() {
            self.position.y = bounds.bottom();
        }
    }
}
