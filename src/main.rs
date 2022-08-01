use chrono::Utc;
use nannou::{
    noise::{NoiseFn, Perlin, Seedable},
    prelude::*,
};

mod mover;
use mover::Mover;

struct Model {
    balloon: Mover,
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

    Model {
        balloon: Mover::new(Vec2::ZERO, 10.0, hsv(0.0, 0.6, 0.6)),
        noise,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();
    let t = app.elapsed_frames() as f64 * 0.01;

    let wind = model.noise.get([0.0, 0.0, t]) as f32 * 1.0;

    model.balloon.apply_force(vec2(0.0, 0.5));
    model.balloon.apply_force(vec2(wind, 0.0));
    model.balloon.update();
    model.balloon.check_edges(&bounds);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(hsv(0.5, 0.4, 0.4));

    model.balloon.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
