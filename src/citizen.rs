extern crate rand;
extern crate rand_distr;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Citizen {
    pos_x: f32,
    pos_y: f32,
    radius: f32,
    velocity_x: f32,
    velocity_y: f32,
    angle: f64,
    resistance_points: f32,
}

impl Citizen {
    pub fn get_x(self) -> f32 {
        self.pos_x
    }

    pub fn get_y(self) -> f32 {
        self.pos_y
    }

    pub fn get_radius(self) -> f32 {
        self.radius
    }

    pub fn move_citizen(&mut self) {
        let vx = self.angle.sin();
        let vy = self.angle.cos();

        let distance_x = self.velocity_x * (vx as f32);
        let distance_y = self.velocity_y * (vy as f32);
        self.pos_x += distance_x;
        self.pos_y += distance_y;
    }

    pub fn change_angle(&mut self) {
        let mut rng = rand::thread_rng();
        self.angle = rng.gen_range(0.0, 360.0);
    }
}

pub fn random_citizen((width, height): (f32, f32)) -> Citizen {
    let mut rng = rand::thread_rng();
    Citizen {
        pos_x: rng.gen_range(0.0, width),
        pos_y: rng.gen_range(0.0, height),
        radius: 50.0,
        velocity_x: 5.0,
        velocity_y: 5.0,
        angle: 0.0,
        resistance_points: 0.0,
    }
}
