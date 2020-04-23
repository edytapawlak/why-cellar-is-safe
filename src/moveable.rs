use ggez::nalgebra as na;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub trait Moveable {
    fn get_position(&self) -> na::Point2<f32>;
    fn get_radius(self) -> f32;
    fn get_speed(self) -> f32;
    fn move_being(&mut self, screen_width: f32, screen_height: f32);
}

#[derive(PartialEq)]
pub enum Zone {
    LeftBorder,
    RightBorder,
    UpBorder,
    BottomBorder,
    Inside,
}
impl Distribution<Zone> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Zone {
        match (rng.gen_range(0.0, 4.0) as f32).floor() as i32 {
            0 => Zone::LeftBorder,
            1 => Zone::RightBorder,
            2 => Zone::UpBorder,
            3 => Zone::BottomBorder,
            _ => Zone::Inside,
        }
    }
}

#[derive(Copy, Clone)]
pub struct EntityParams {
    center: na::Point2<f32>,
    radius: f32,
    velocity: na::Vector2<f32>,
    speed: f32,
}

impl EntityParams {
    pub fn get_center(self) -> na::Point2<f32> {
        self.center
    }

    pub fn get_radius(self) -> f32 {
        self.radius
    }

    pub fn get_speed(self) -> f32 {
        self.speed
    }

    pub fn set_velocity(&mut self, v: na::Vector2<f32>) {
        self.velocity = v;
    }

    pub fn come_back(&mut self) {
        self.velocity = -self.velocity;
    }

    pub fn set_cx(&mut self, x: f32) {
        self.center.x = x;
    }

    pub fn set_cy(&mut self, y: f32) {
        self.center.y = y;
    }

    pub fn set_speed(&mut self, s: f32) {
        self.speed = s;
    }

    pub fn where_is(self, width: f32, height: f32) -> Zone {
        let center = self.center;
        let r = self.radius;

        if center.x + r < 0.0 {
            return Zone::LeftBorder;
        }
        if center.x - r > width {
            return Zone::RightBorder;
        }
        if center.y - r > height {
            return Zone::BottomBorder;
        }
        if center.y + r < 0.0 {
            return Zone::UpBorder;
        }
        Zone::Inside
    }

    pub fn move_step(&mut self) {
        self.center += self.speed * self.velocity
    }

    pub fn stop(&mut self) {
        self.speed = 0.0;
    }

    pub fn random((width, height): (f32, f32)) -> EntityParams {
        let mut rng = rand::thread_rng();
        let angle: f64 = rng.gen_range(0.0, 360.0);
        EntityParams {
            center: na::Point2::new(rng.gen_range(0.0, width), rng.gen_range(0.0, height)),
            radius: 20.0,
            velocity: na::Vector2::new(angle.sin() as f32, angle.cos() as f32),
            speed: 5.0,
        }
    }

    pub fn new(c: na::Point2<f32>, r: f32, v: na::Vector2<f32>, s: f32) -> EntityParams {
        EntityParams {
            center: c,
            radius: r,
            velocity: v,
            speed: s,
        }
    }
}
