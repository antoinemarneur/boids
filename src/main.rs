use nannou::prelude::*;
use boids::Boid;

const BOIDS: usize = 20;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    boids : Vec<Boid>,
}

fn model(app: &App) -> Model {
    let mut boids = Vec::with_capacity(BOIDS);

    for _n in 0..BOIDS {
        boids.push(Boid::new((random_f32() - 0.5) * 800.0, (random_f32() - 0.5) * 600.0, 5.0));
    }

    app.new_window().size(1000, 700).view(view).build().unwrap();

    Model { boids }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let boundary = app.window_rect();

    for i in 0..BOIDS {
        let sep = model.boids[i].separation(&model.boids);
        let align = model.boids[i].alignment(&model.boids);
        let cohesion = model.boids[i].cohesion(&model.boids);
        
        model.boids[i].check_border(boundary);
        model.boids[i].update(sep.0 + align.0 + cohesion.0, sep.1 + align.1 + cohesion.1);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(GREY);

    for boid in model.boids.iter() {
        boid.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
