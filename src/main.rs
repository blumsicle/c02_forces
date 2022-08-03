use chrono::Utc;
use nannou::{
    noise::{NoiseFn, Perlin, Seedable},
    prelude::*,
};

mod mover;
use mover::Mover;

struct Model {
    balls: Vec<Mover>,
    noise: Perlin,
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
    let noise = Perlin::new().set_seed(seed);

    let bounds = app.window_rect();
    let mut balls = Vec::new();
    for _ in 0..100 {
        let position = (random::<Vec2>() - 0.5) * bounds.wh();
        let mass = 5.0 + random_f32() * 15.0;
        let color = hsv(random_f32(), 0.6, 0.6);
        balls.push(Mover::new(position, mass, color));
    }

    Model { balls, noise }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();
    let t = app.elapsed_frames();

    for (i, ball) in model.balls.iter_mut().enumerate() {
        let t = (t + i as u64) as f64 * 0.01;
        let wind = model.noise.get([0.0, 0.0, t]) as f32 * 1.0;
        ball.apply_force(vec2(wind, 0.0));

        let gravity = -1.0 * ball.mass;
        ball.apply_force(vec2(0.0, gravity));

        ball.update();
        ball.check_edges(&bounds);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(hsv(0.5, 0.4, 0.4));

    for ball in &model.balls {
        ball.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
