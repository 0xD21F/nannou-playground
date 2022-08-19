use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing {
    vectors: Vec<Vector2>,
}

impl Thing {
    pub fn new(p: Vec<Vector2>) -> Self {
        let vectors = p;
        Thing { vectors }
    }
}

struct Model {
    things: Vec<Thing>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    let things = vec![
        Thing::new(vec![vec2(0.0, 0.0), vec2(0.0, 1024.0)]),
        Thing::new(vec![vec2(0.0, 0.0), vec2(0.0, -1024.0)]),
    ];

    Model { things }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut next = vec![];
    for thing in model.things.iter() {
        let next_thing = Thing::new(vec![thing.vectors[0], thing.vectors[1].rotate(0.02)]);
        next.push(next_thing);
    }

    model.things = next;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let frames = app.elapsed_frames();
    let frame_shift = frames + 300;
    let time = app.time;
    let win = app.window_rect();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    let color = Hsv::new(
        (frames % 360).to_f32().unwrap(),
        0.9,
        0.9,
    );

    // Modified from nannou example draw_polyline
    let n_points = 1000;
    let weight = 10.0;

    let hz = 4.0;

    let vertices = (0..n_points)
        // A sine wave mapped to the range of the window.
        .map(|i| {
            let x = map_range(i, 0, n_points - 1, win.left(), win.right());
            let fract = i as f32 / n_points as f32;
            let amp = (time + fract * hz * TAU).sin();
            let y = map_range(amp, -1.0, 1.0, win.bottom() * 0.5, win.top() * 0.5);
            pt2(x, y)
        })
        .enumerate()
        // Colour each vertex uniquely based on its index.
        .map(|(i, p)| {
            let fract = i as f32 / n_points as f32;
            let r = (time + fract) % 1.0;
            let g = (time + 1.0 - fract) % 1.0;
            let b = (time + 0.5 + fract) % 1.0;
            let rgba = srgba(r, g, b, 1.0);
            (p, rgba)
        });

    for thing in model.things.iter() {
        draw.polyline()
            .weight(20.0)
            .points(thing.vectors.iter().cloned())
            .color(color);
    }

    // Draw the polyline as a stroked path.
    draw.polyline()
        .weight(weight)
        .join_round()
        .points_colored(vertices);

    for n in (1..=50).rev() {
        let ellipse_color = Hsv::new((frame_shift % 360 + n * 2).to_f32().unwrap(), 0.9, 0.9);
        draw.ellipse()
            .color(ellipse_color)
            .w(n as f32 * 10.)
            .h(n as f32 * 10.);
    }

    draw.to_frame(app, &frame).unwrap();
}
