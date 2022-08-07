use chrono::Utc;
use nannou::{
    noise::{Perlin, Seedable},
    prelude::*,
};

mod attractor;
use attractor::Attractor;

mod mover;
use mover::Mover;

struct Model {
    attractor: Attractor,
    follow_mouse: bool,
    follow_offset: Option<Vec2>,
    balls: Vec<Mover>,
    _noise: Perlin,
}

fn model(app: &App) -> Model {
    let _ = app
        .new_window()
        .title("noc_c02_forces")
        .size(800, 800)
        .view(view)
        .event(event)
        .build()
        .unwrap();

    let seed = Utc::now().timestamp() as u32;
    let _noise = Perlin::new().set_seed(seed);

    let bounds = app.window_rect();

    let attractor = Attractor::new(Vec2::ZERO, 30.0);

    let mut balls = Vec::new();
    for _ in 0..1 {
        let position = vec2(
            (random_f32() - 0.5) * bounds.w(),
            (random_f32() - 0.5) * bounds.h(),
        );
        let mass = 5.0 + random_f32() * 15.0;
        let color = hsv(random_f32(), 0.6, 0.2);
        balls.push(Mover::new(position, mass, color));
    }

    Model {
        attractor,
        follow_mouse: false,
        follow_offset: None,
        balls,
        _noise,
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(button) => {
            if button == MouseButton::Left {
                let mouse = vec2(app.mouse.x, app.mouse.y);
                let offset = model.attractor.position - mouse;
                if offset.length() < model.attractor.radius {
                    model.follow_mouse = true;
                    model.follow_offset = Some(offset);
                }
            }
        }

        MouseReleased(button) => {
            if button == MouseButton::Left {
                model.follow_mouse = false;
                model.follow_offset = None;
            }
        }
        _ => (),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.follow_mouse {
        if let Some(offset) = model.follow_offset {
            let mouse = vec2(app.mouse.x, app.mouse.y);
            model.attractor.position = mouse + offset;
        }
    }

    for ball in &mut model.balls {
        // Gravitational pull force
        let force = model.attractor.attract(ball);
        ball.apply_force(force);

        ball.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(hsv(0.6, 0.8, 0.05));

    model.attractor.draw(&draw);

    for ball in &model.balls {
        ball.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
