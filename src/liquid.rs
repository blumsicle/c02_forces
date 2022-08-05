use nannou::prelude::*;

pub struct Liquid {
    pub rect: Rect,
    pub coefficient: f32,
}

impl Liquid {
    pub fn new(position: Vec2, size: Vec2, coefficient: f32) -> Self {
        Self {
            rect: Rect::from_xy_wh(position, size),
            coefficient,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.rect()
            .xy(self.rect.xy())
            .wh(self.rect.wh())
            .color(hsva(0.6, 0.5, 0.7, 0.2));
    }
}
