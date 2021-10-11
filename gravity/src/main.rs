mod balls;
mod ball;
mod physics;

use nannou::prelude::*;
use balls::Balls;
use ball::Ball;
use physics::apply_gravity;

pub trait Drawable {
    fn draw(&self, draw_context: &Draw);
}

fn main() {
    nannou::app(init_model).update(update).run();
}

struct Model {
    _window: window::Id,
    balls: Balls,
}

impl Model {
    fn reset(&mut self) {
        self.balls = init_balls();         
    }
}

fn init_model(app: &App) -> Model {
    let balls = init_balls();
    let _window = app.new_window().view(view).build().unwrap();
    
    Model { balls, _window }
}

fn init_balls() -> Balls {
    Balls::new_static()
    // let mut balls = Vec::new();
    // for x in -5..5 {
    //     for y in -5..5 {
    //         balls.push(Ball::new(Point2::new(50.0 * x as f32, 50.0 * y as f32), 10.0, Point2::new(0.0, 0.0), RED));
    //     }
    // }
    // Balls::new(balls)
}

fn update(app: &App, model: &mut Model, _update: Update) {
    apply_gravity(model.balls.balls_mut());
    
    model.balls.update(app.window_rect());

    if app.keys.down.contains(&Key::R){
        model.reset();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let Model{ balls, .. } = model;

    balls.draw(&draw);

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
