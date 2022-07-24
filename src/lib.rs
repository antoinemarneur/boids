use nannou::prelude::*;

const WIDTH: f32 = 15.0;
const HEIGHT: f32 = 15.0;
const MARGIN: f32 = 100.0;
const SPEED_LIMIT: f32 = 10.0;
const SEPARATION: f32 = 40.0;
const DISTANCE: f32 = 100.0;


#[derive(PartialEq)]
pub struct Boid {
    position: Vec2,
    velocity: Vec2,
}

impl Boid {
    pub fn new(position: Vec2, velocity: Vec2) -> Boid {
        Boid {
            position,
            velocity,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.tri().color(BLACK)
            .w_h(WIDTH, HEIGHT)
            .xy(self.position)
            .rotate(self.velocity.angle());
    }

    pub fn update(&mut self, velocity: Vec2) {
        self.velocity += velocity;

        self.speed_limit();

        self.position += self.velocity;
    }

    // Regulate the speed of every boid to behave like real animals.
    fn speed_limit(&mut self) {
        let speed_limit = SPEED_LIMIT;

        let speed = (self.velocity.x * self.velocity.x + self.velocity.y * self.velocity.y).sqrt();
        if speed > speed_limit {
            self.velocity = (self.velocity / speed) * speed_limit;
        }
    }

    // Keep the boid in the window by stabilizing its velocity.
    // ± helps to stabilize it gradually. w/ an addition or soustraction
    // the transition is smooth to the opposite side with a short delay defined by the velocity.
    // >> let velocity: Vec2 = { x=-1.0, y=0.0 };
    // >> if position < LEFT_BORDER { velocity.x += 1.0 };
    // >> position = LEFT_BORDER // velocity.x = 0.0
    // >> position = LEFT_BORDER // velocity.x = 1.0
    // >> end
    // Having this transition helps for separation, alignment and cohesion near a border.
    pub fn check_border(&mut self, window: Rect<f32>) {
        let margin = MARGIN;
        let turn_factor = 1.0;

        if self.position.x < window.left() + margin {
            self.velocity.x += turn_factor;
        }
        
        if self.position.x > window.right() - margin {
            self.velocity.x -= turn_factor;
        }

        if self.position.y < window.bottom() + margin {
            self.velocity.y += turn_factor;
        }
        
        if self.position.y > window.top() - margin {
            self.velocity.y -= turn_factor;
        }
    }

    // Keep a small distance away from other boids.
    pub fn separation(&self, boids: &[Boid]) -> Vec2 {
        let mut velocity = Vec2::new(0.0, 0.0);
        let factor = 0.05; // Adjust the velocity of the action.

        for boid in boids.iter() {
            if boid != self {
                if distance(&self, &boid) < SEPARATION {
                    velocity += self.position - boid.position;
                }
            }
        }

        velocity = velocity * factor;

        velocity
    }

    // Match velocity with near boids.
    pub fn alignment(&self, boids: &[Boid]) -> Vec2 {
        let mut velocity = Vec2::new(0.0, 0.0);
        let mut neighbors = 0.0;
        let factor = 0.05; // Adjust the velocity of the action.

        for boid in boids.iter() {
            if boid != self {
                if distance(&self, &boid) < DISTANCE {
                    velocity.x += boid.velocity.x;
                    velocity.y += boid.velocity.y;

                    neighbors += 1.0;
                }
            }
        }

        if neighbors > 0.0 {
            velocity = ((velocity / neighbors) - self.velocity) * factor;
        }

        velocity
    }

    // Fly towards the center of mass of neihbouring boids.
    // Get the average position of all the boids defined as the center of mass.
    pub fn cohesion(&self, boids: &[Boid]) -> Vec2 {
        let mut velocity = Vec2::new(0.0, 0.0);
        let mut neighbors = 0.0;
        let factor = 0.005; // Adjust the velocity of the action.

        for boid in boids.iter() {
            if boid != self {
                if distance(&self, &boid) < DISTANCE {
                    velocity.x += boid.position.x;
                    velocity.y += boid.position.y;

                    neighbors += 1.0;
                }
            }
        }

        if neighbors > 0.0 {
            velocity = ((velocity / neighbors) - self.position) * factor;
        }

        velocity
    }
}

// Pythagoras theorem a^2 + b^2 = c^2
fn distance(boid1: &Boid, boid2: &Boid) -> f32 {
    ((boid1.position.x - boid2.position.x) * (boid1.position.x - boid2.position.x) + (boid1.position.y - boid2.position.y) * (boid1.position.y - boid2.position.y)).sqrt()
}
