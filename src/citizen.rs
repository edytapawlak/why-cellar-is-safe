use ggez::nalgebra as na;
use rand::Rng;

use crate::movingbeing::EntityParams;
use crate::movingbeing::MovingBeing;
use crate::movingbeing::Zone;

#[derive(Copy, Clone)]
pub struct Citizen {
    ent_params: EntityParams,
    is_infected: bool,
}

impl Citizen {
    pub fn get_is_infected(&self) -> bool {
        self.is_infected
    }

    pub fn change_angle(&mut self) {
        let mut rng = rand::thread_rng();
        let angle: f64 = rng.gen_range(0.0, 360.0);
        let vx = angle.sin() as f32;
        let vy = angle.cos() as f32;
        self.ent_params.set_velocity(na::Vector2::new(vx, vy));
    }

    pub fn become_infected(&mut self) {
        self.is_infected = true;
    }
}

impl MovingBeing for Citizen {
    fn get_position(&self) -> na::Point2<f32> {
        self.ent_params.get_center()
    }

    fn get_radius(self) -> f32 {
        self.ent_params.get_radius()
    }

    fn get_speed(self) -> f32 {
        self.ent_params.get_speed()
    }

    fn move_being(&mut self, (width, height): (f32, f32)) {
        let r = self.ent_params.get_radius();
        match self.ent_params.where_is((width, height)) {
            Zone::LeftBorder => {
                self.ent_params.set_cx(width + r);
                self.is_infected = false;
            }
            Zone::RightBorder => {
                self.ent_params.set_cx(-r);
                self.is_infected = false;
            }
            Zone::BottomBorder => {
                self.ent_params.set_cy(-r);
                self.is_infected = false
            }
            Zone::UpBorder => {
                self.ent_params.set_cy(height + r);
                self.is_infected = false
            }
            Zone::Inside => self.ent_params.move_step(),
        }
    }
}

pub fn random_citizen((width, height): (f32, f32)) -> Citizen {
    Citizen {
        ent_params: EntityParams::random((width, height)),
        is_infected: false,
    }
}
