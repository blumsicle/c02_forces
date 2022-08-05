use chrono::Utc;
use nannou::{
    noise::{Perlin, Seedable},
    prelude::*,
};

mod mover;
use mover::Mover;

struct Model {
    balls: Vec<Mover>,
    _noise: Perlin,
}

fn model(app: &App) -> Model {
    let _ = app
        .new_window()
        .title("noc_c02_forces")
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let seed = Utc::now().timestamp() as u32;
    let _noise = Perlin::new().set_seed(seed);

    let bounds = app.window_rect();
    let mut balls = Vec::new();
    for _ in 0..10 {
        let position = (random::<Vec2>() - 0.5) * bounds.wh();
        // let position = vec2((random_f32() - 0.5) * bounds.w(), 200.0);
        let mass = 5.0 + random_f32() * 15.0;
        let color = hsv(random_f32(), 0.6, 0.2);
        balls.push(Mover::new(position, mass, color));
    }

    Model { balls, _noise }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();

    for ball in &mut model.balls {
        // Wind force
        let wind = vec2(0.1, 0.0);
        ball.apply_force(wind);

        // Friction force
        let c = 0.1;
        let friction = (ball.velocity * -1.0).normalize_or_zero();
        let friction = friction * c;
        ball.apply_force(friction);

        // Gravity force
        let gravity = -1.0 * ball.mass;
        ball.apply_force(vec2(0.0, gravity));

        ball.update();
        ball.check_edges(&bounds);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(hsv(0.6, 0.8, 0.05));

    for ball in &model.balls {
        ball.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
