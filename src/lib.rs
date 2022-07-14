use nannou::prelude::*;

#[derive(PartialEq)]
pub struct Boid {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Boid {
    pub fn new(x: f32, y: f32, velocity: f32) -> Boid {
        Boid {
            x,
            y,
            dx: velocity,
            dy: velocity,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.tri().color(BLACK)
            .w_h(15.0, 15.0)
            .x_y(self.x, self.y)
            .rotate(vec2(self.dx, self.dy).angle());
    }

    pub fn update(&mut self, x: f32, y: f32) {
        self.dx += x;
        self.dy += y;

        self.speedLimit();

        self.x += self.dx;
        self.y += self.dy;
    }

    // Regulate the speed of every boid to behave boids like real animals.
    fn speedLimit(&mut self) {
        let speedLimit = 10.0;

        let speed = (self.dx * self.dx + self.dy * self.dy).sqrt();
        if (speed > speedLimit) {
            self.dx = (self.dx / speed) * speedLimit;
            self.dy = (self.dy / speed) * speedLimit;
        }
    }

    // Keep the boid in the window by stabilizing its velocity.
    // ± helps to stabilize it gradually. w/ an addition or soustraction
    // the transition is smooth to the opposite side with a short delay defined by the velocity.
    // let dx = -1;
    // if x < 200 { dx += 1 };
    // >> x = 200 // dx = 0
    // >> x = 200 // dx = 1
    // Having this transition help for separation, alignment and cohesion near a border.
    pub fn check_border(&mut self, window: Rect<f32>) {
        let margin = 100.0;

        if self.x < window.left() + margin {
            self.dx += 1.0;
        }
        
        if self.x > window.right() - margin {
            self.dx -= 1.0;
        }

        if self.y < window.bottom() + margin {
            self.dy += 1.0;
        }
        
        if self.y > window.top() - margin {
            self.dy -= 1.0;
        }
    }

    // Keep a small distance away from other boids.
    pub fn separation(&self, boids: &[Boid]) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        let factor = 0.05; // Adjust the velocity of the action.

        for boid in boids.iter() {
            if boid != self {
                if distance(&self, &boid) < 40.0 {
                    x += self.x - boid.x;
                    y += self.y - boid.y;
                }
            }
        }

        x = x * factor;
        y = y * factor;

        (x, y)
    }

    // Match velocity with near boids.
    pub fn alignment(&self, boids: &[Boid]) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut neighbors = 0.0;
        let factor = 0.05; // Adjust the velocity of the action.

        for boid in boids.iter() {
            if boid != self {
                if distance(&self, &boid) < 100.0 {
                    x += boid.dx;
                    y += boid.dy;

                    neighbors += 1.0;
                }
            }
        }

        if neighbors > 0.0 {
            x = ((x / neighbors) - self.dx) * factor;
            y = ((y / neighbors) - self.dy) * factor;
        }

        (x, y)
    }

    // Fly towards the center of mass of neihbouring boids.
    // Get the average position of all the boids defined as the center of mass.
    pub fn cohesion(&self, boids: &[Boid]) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut neighbors = 0.0;
        let factor = 0.005; // Adjust the velocity of the action.

        for boid in boids.iter() {
            if boid != self {
                if distance(&self, &boid) < 100.0 {
                    x += boid.x;
                    y += boid.y;

                    neighbors += 1.0;
                }
            }
        }

        if neighbors > 0.0 {
            x = ((x / neighbors) - self.x ) * factor;
            y = ((y / neighbors) - self.y ) * factor;
        }

        (x, y)
    }
}

// Pythagoras theorem a^2 + b^2 = c^2
fn distance(boid1: &Boid, boid2: &Boid) -> f32 {
    ((boid1.x - boid2.x) * (boid1.x - boid2.x) + (boid1.y - boid2.y) * (boid1.y - boid2.y)).sqrt()
}