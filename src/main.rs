use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window, x: random_f32() , y: random_f32(), dx: 3.0, dy: 3.0 }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.x += model.dx;
    model.y += model.dy;

    let boundary = app.window_rect();

    if model.x < boundary.left() || model.x > boundary.right() {
        model.dx *= -1.0;
    }

    if model.y < boundary.bottom() || model.y > boundary.top() {
        model.dy *= -1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(GREY);
    draw.tri().color(STEELBLUE).w(30.0).h(30.0).rotate(vec2(model.dx, model.dy).angle()).x_y(model.x, model.y);
    draw.to_frame(app, &frame).unwrap();
}
