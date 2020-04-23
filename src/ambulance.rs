use ggez::nalgebra as na;
use ggez::{graphics, Context, GameResult};
use rand;
use rand::Rng;

use crate::moveable::EntityParams;
use crate::moveable::Moveable;
use crate::moveable::Zone;

#[derive(Copy, Clone)]
pub struct Ambulance {
    ent_params: EntityParams,
    pub destination: na::Point2<f32>,
    is_free: bool,
    patient: Option<usize>,
}

impl Ambulance {
    pub fn finish_act(&mut self) {
        self.is_free = true;
        self.patient = None;
    }

    pub fn is_free(self) -> bool {
        self.is_free
    }

    pub fn set_destination(
        &mut self,
        cid: usize,
        dest: na::Point2<f32>,
        width: f32,
        height: f32)
     {
        self.destination = dest;
        self.patient = Some(cid);
        self.is_free = false;
        let mut rng = rand::thread_rng();
        let zone: Zone = rand::random();
        let r = self.get_radius();
        let start = match zone {
            Zone::LeftBorder => na::Point2::new(-r, rng.gen_range(0.0, height)),
            Zone::RightBorder => na::Point2::new(width + r, rng.gen_range(0.0, height)),
            Zone::UpBorder => na::Point2::new(rng.gen_range(0.0, width), -r),
            Zone::BottomBorder => na::Point2::new(rng.gen_range(0.0, width), height + r),
            _ => na::Point2::new(r, r),
        };
        self.ent_params.set_cx(start.x);
        self.ent_params.set_cy(start.y);
        self.ent_params
            .set_velocity((self.destination - start).normalize());
    }

    pub fn draw_ambulance(&mut self, ctx: &mut Context) -> GameResult {
        let image = graphics::Image::new(ctx, "/ambulance2.png")?;
        let drawparams = graphics::DrawParam::new()
            .dest(self.get_position())
            .rotation(0.0)
            .offset(na::Point2::new(0.5, 0.5));

        graphics::draw(ctx, &image, drawparams)
    }

    pub fn get_patient_id(self) -> Option<usize> {
        self.patient
    }
}

impl Moveable for Ambulance {
    fn get_position(&self) -> na::Point2<f32> {
        self.ent_params.get_center()
    }

    fn get_radius(self) -> f32 {
        self.ent_params.get_radius()
    }

    fn get_speed(self) -> f32 {
        self.ent_params.get_speed()
    }

    fn move_being(&mut self, width: f32, height: f32) {
        match self.ent_params.where_is(width, height) {
            Zone::Inside => {
                self.ent_params.move_step();
                if na::distance(&self.destination, &self.get_position())
                    <= self.ent_params.get_radius()
                {
                    self.ent_params.come_back();
                }
            }
            _ => {
                if !self.is_free() {
                    self.finish_act();
                }
            }
        }
    }
}

pub fn new(width: f32, height: f32, dest: na::Point2<f32>) -> Ambulance {
    let mut rng = rand::thread_rng();
    let zone: Zone = rand::random();
    let r = 10.0; 
    let start = match zone {
        Zone::LeftBorder => na::Point2::new(-r, rng.gen_range(0.0, height)),
        Zone::RightBorder => na::Point2::new(width + r, rng.gen_range(0.0, height)),
        Zone::UpBorder => na::Point2::new(rng.gen_range(0.0, width), -r),
        Zone::BottomBorder => na::Point2::new(rng.gen_range(0.0, width), height + r),
        _ => na::Point2::new(r, r),
    };
    let vect: na::Vector2<f32> = (dest - start).normalize();

    let ent_params = EntityParams::new(start, r, vect, 10.0);
    Ambulance {
        ent_params: ent_params,
        destination: dest,
        is_free: false,
        patient: None,
    }
}
