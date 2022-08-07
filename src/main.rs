use chrono::Utc;
use nannou::{
    noise::{Perlin, Seedable},
    prelude::*,
};

mod mover;
use mover::Mover;

mod liquid;
use liquid::Liquid;

struct Model {
    balls: Vec<Mover>,
    drag_field: Liquid,
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
        let position = vec2((random_f32() - 0.5) * bounds.w(), bounds.top() - 30.0);
        let mass = 5.0 + random_f32() * 15.0;
        let color = hsv(random_f32(), 0.6, 0.2);
        balls.push(Mover::new(position, mass, color));
    }

    let rect = Rect::from_w_h(bounds.w(), bounds.h() * 0.5).align_bottom_of(bounds);
    let drag_field = Liquid::new(rect.xy(), rect.wh(), 0.2);

    Model {
        balls,
        drag_field,
        _noise,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();

    for ball in &mut model.balls {
        // Wind force
        let wind = vec2(0.1, 0.0);
        ball.apply_force(wind);

        // Drag force
        if ball.is_inside(&model.drag_field) {
            let speed = ball.velocity.length();
            let drag_magnitude = model.drag_field.coefficient * speed * speed;
            let drag = (ball.velocity * -1.0).normalize_or_zero();
            let drag = drag * drag_magnitude;
            ball.apply_force(drag);
        }

        // Gravity force
        let gravity = vec2(0.0, -1.0 * ball.mass);
        ball.apply_force(gravity);

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

    model.drag_field.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
