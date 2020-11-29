use nannou::color::named::{BLACK, WHITE};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[derive(Copy, Clone, PartialEq)]
struct Circle {
    point: Vector2<f32>,
    r: f32,
    growing: bool,
}

impl Circle {
    fn new(x: f32, y: f32, r: f32) -> Self {
        Circle {
            point: vec2(x, y),
            r,
            growing: true,
        }
    }

    fn overlaps(self: Circle, other: &Circle, increment: f32) -> bool {
        self.point.distance(other.point) <= self.r + other.r + increment
    }
}

struct Model {
    circles: Vec<Circle>,
}

fn model(_app: &App) -> Model {
    Model { circles: vec![] }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let area = app.window_rect();
    for _i in 0..clamp(model.circles.len() / 5, 1, 100) {
        'inner: for j in 0..100 {
            if j == 99 {
                app.set_loop_mode(LoopMode::loop_once());
                app.main_window().capture_frame("./screenshot.png");
                println!("DONE");
            }
            let x = random_range(area.left(), area.right());
            let y = random_range(area.top(), area.bottom());
            let new_size = match model.circles.len() {
                0..=5 => 25.0,
                6..=10 => 20.0,
                11..=20 => 10.0,
                21..=50 => 5.0,
                _ => 2.0,
            };
            let new = Circle::new(x, y, new_size);
            if model
                .circles
                .iter()
                .any(|other| new.overlaps(other, new_size / 2.0))
            {
                continue;
            }
            model.circles.push(new);
            break 'inner;
        }
    }
    for i in 1..=model.circles.len() {
        let (first, second) = model.circles[..].split_at_mut(i);
        if let Some(mut c) = first.last_mut() {
            let increment = clamp(c.r / 10.0, 1.0, 10.0);
            second.iter_mut().for_each(|other| {
                if c.overlaps(other, increment * 2.0) {
                    c.growing = false;
                    other.growing = false;
                };
            });
            if c.growing {
                c.r += increment;
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for c in &model.circles {
        let diameter = c.r * 2.0;
        draw.ellipse()
            .x_y(c.point.x, c.point.y)
            .width(diameter)
            .height(diameter)
            .no_fill()
            .stroke_weight(2.0)
            .stroke(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}
