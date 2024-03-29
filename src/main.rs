use nannou::color::rgba;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    x: f32,
    y: f32,
    size: f32,
    t: f32,
}

fn model(_app: &App) -> Model {
    Model {
        x: 0.0,
        y: 0.0,
        size: 20.0,
        t: 0.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.t += 0.01667;

    let mouse_pos = app.mouse.position();

    model.x = model.t.sin() * mouse_pos.x;
    model.y = (model.t * 1.5).sin() * mouse_pos.y;
    model.size = (model.t * 2.0).sin() * 10.0 + 20.0;
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    //draw.background().color(PLUM);
    //draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));

    draw.rect()
        .wh(app.window_rect().wh())
        .color(rgba(0.0, 0.0, 0.0, 0.03));

    draw.ellipse()
        .color(rgba(0.0, 0.0, 1.0, 0.9))
        .w(model.size)
        .h(model.size)
        .x_y(model.x, model.y);

    draw.to_frame(app, &frame).unwrap();
}
