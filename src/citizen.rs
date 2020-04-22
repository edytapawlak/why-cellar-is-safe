use ggez::nalgebra as na;
use rand::Rng;
use ggez::graphics;

use crate::moveable::EntityParams;
use crate::moveable::Moveable;
use crate::moveable::Zone;
use crate::infectable::InfectionParams;
use crate::infectable::Infectable;

#[derive(Copy, Clone)]
pub struct Citizen {
    ent_params: EntityParams,
    is_infected: bool,
    inf_params : InfectionParams,
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

    //pub fn become_infected(&mut self) {
    //    self.is_infected = true;
    //}

    pub fn get_color(self, hcolor: graphics::Color, dcolor : graphics::Color) -> graphics::Color {
      let h_to_imm = self.inf_params.helth_to_imm();
      let calc = |h: f32, d: f32| -> f32 { h - (h_to_imm *(h-d))};
      let dr = calc(hcolor.r, dcolor.r);
      let dg = calc(hcolor.g, dcolor.g);
      let db = calc(hcolor.b, dcolor.b);
      let da = calc(hcolor.a, dcolor.a);
      graphics::Color::new(dr, dg, db, da)
    }
}

impl Moveable for Citizen {
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
                self.inf_params.recover();
            }
            Zone::RightBorder => {
                self.ent_params.set_cx(-r);
                self.inf_params.recover();
            }
            Zone::BottomBorder => {
                self.ent_params.set_cy(-r);
                self.inf_params.recover();
            }
            Zone::UpBorder => {
                self.ent_params.set_cy(height + r);
                self.inf_params.recover();
            }
            Zone::Inside => self.ent_params.move_step(),
        }
    }
}

impl Infectable for Citizen {

  fn needs_doctor(self) -> bool {
    self.inf_params.needs_doctor()
  } 

  fn get_immunity(self) -> i32 {
    self.inf_params.get_immunity()
  }
  fn become_infected(&mut self) {
    self.inf_params.infect();
  }
  fn call_emergency(&mut self) {
      self.ent_params.stop();
    }

  fn cure(&mut self) {
    self.inf_params.cure();
  }
}

pub fn random_citizen((width, height): (f32, f32)) -> Citizen {
    Citizen {
        ent_params: EntityParams::random((width, height)),
        is_infected: false,
        inf_params: InfectionParams::default(),
    }
}
