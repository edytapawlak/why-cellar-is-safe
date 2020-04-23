use ggez::nalgebra as na;
use ggez::{graphics, Context, GameResult};
use rand::Rng;

use crate::infectable::Infectable;
use crate::infectable::InfectionParams;
use crate::moveable::EntityParams;
use crate::moveable::Moveable;
use crate::moveable::Zone;

#[derive(Copy, Clone)]
pub struct Citizen {
    ent_params: EntityParams,
    inf_params: InfectionParams,
    id: usize,
}

impl Citizen {
    pub fn draw_citizen(
        self,
        ctx: &mut Context,
        hcolor: graphics::Color,
        dcolor: graphics::Color,
    ) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.get_position(),
            self.get_radius(),
            2.0,
            self.get_color(hcolor, dcolor),
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())
    }

    pub fn change_angle(&mut self) {
        let mut rng = rand::thread_rng();
        let angle: f64 = rng.gen_range(0.0, 360.0);
        let vx = angle.sin() as f32;
        let vy = angle.cos() as f32;
        self.ent_params.set_velocity(na::Vector2::new(vx, vy));
    }

    pub fn get_color(self, hcolor: graphics::Color, dcolor: graphics::Color) -> graphics::Color {
        let h_to_imm = self.inf_params.helth_to_imm();
        let calc = |h: f32, d: f32| -> f32 { h - (h_to_imm * (h - d)) };
        let dr = calc(hcolor.r, dcolor.r);
        let dg = calc(hcolor.g, dcolor.g);
        let db = calc(hcolor.b, dcolor.b);
        let da = calc(hcolor.a, dcolor.a);
        graphics::Color::new(dr, dg, db, da)
    }

    pub fn go_hospital(&mut self, p: na::Point2<f32>) {
        self.ent_params.set_cx(p.x);
        self.ent_params.set_cy(p.y);
    }

    pub fn recover(&mut self) {
        self.inf_params.recover();
    }

    pub fn get_id(self) -> usize {
        self.id
    }

    pub fn stop(&mut self){
      self.ent_params.stop();
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

    fn move_being(&mut self, width: f32, height: f32) {
        let r = self.ent_params.get_radius();
        match self.ent_params.where_is(width, height) {
            Zone::LeftBorder => {
                self.ent_params.set_cx(width + r);
                self.recover();
            }
            Zone::RightBorder => {
                self.ent_params.set_cx(-r);
                self.recover();
            }
            Zone::BottomBorder => {
                self.ent_params.set_cy(-r);
                self.recover();
            }
            Zone::UpBorder => {
                self.ent_params.set_cy(height + r);
                self.recover();
            }
            Zone::Inside => self.ent_params.move_step(),
        }
    }
}

impl Infectable for Citizen {
    fn needs_doctor(&mut self) -> bool {
        self.inf_params.needs_doctor()
    }

    fn get_immunity(self) -> i32 {
        self.inf_params.get_immunity()
    }
    fn become_infected(&mut self) {
        self.inf_params.infect();
    }

    fn cure(&mut self) {
        self.inf_params.cure();
        if self.inf_params.is_healthy() {
            self.ent_params.set_speed(5.0);
        }
    }
}

pub fn random_citizen(width: f32, height: f32, id: usize) -> Citizen {
    Citizen {
        id,
        ent_params: EntityParams::random((width, height)),
        inf_params: InfectionParams::default(),
    }
}
