extern crate rand;
extern crate rand_distr;
use rand::Rng;
use rand_distr::{Distribution, Normal, StandardNormal};

#[derive(Clone, Copy)]
pub struct Citizen {
  pos_x: f32,
  pos_y: f32,
  radius: f32,
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
    let mut rng = rand::thread_rng();
    let f : f32 = rng.sample(StandardNormal);
    self.pos_x += f * 10.0;
    self.pos_y += f * 10.0;
  }
}



pub fn random_citizen((width, height) : (f32, f32)) -> Citizen{
  let mut rng = rand::thread_rng();
  Citizen {
    pos_x : rng.gen_range(0.0, width),
    pos_y : rng.gen_range(0.0, height),
    radius : 50.0,
  }
}