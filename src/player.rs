use ggez::event::KeyCode;
use ggez::nalgebra as na;
use std::collections::HashSet;

use crate::moveable::EntityParams;
use crate::moveable::Moveable;
use crate::moveable::Zone;

#[derive(Clone, Copy)]
pub struct Player {
    ent_params: EntityParams,
    sneeze_range: f32,
    sneeze_max: f32,
    is_sneezing: bool,
    citizens_infected: i32,
}

impl Player {
    pub fn move_player(&mut self, (width, height): (f32, f32), pressed_keys: &HashSet<KeyCode>) {
        let r = self.ent_params.get_radius();
        match self.ent_params.where_is((width, height)) {
            Zone::LeftBorder => self.ent_params.set_cx(width + r),
            Zone::RightBorder => self.ent_params.set_cx(-r),
            Zone::BottomBorder => self.ent_params.set_cy(-r),
            Zone::UpBorder => self.ent_params.set_cy(height + r),
            Zone::Inside => {
                let c = self.get_position();
                let s = self.get_speed();

                self.ent_params.set_velocity(na::Vector2::new(0.0, 0.0));
                if pressed_keys.contains(&KeyCode::Left) {
                    self.ent_params.set_cx(c.x - s);
                };
                if pressed_keys.contains(&KeyCode::Right) {
                    self.ent_params.set_cx(c.x + s);
                }
                if pressed_keys.contains(&KeyCode::Up) {
                    self.ent_params.set_cy(c.y - s);
                }
                if pressed_keys.contains(&KeyCode::Down) {
                    self.ent_params.set_cy(c.y + s);
                }
                self.ent_params.move_step();
            }
        }
    }

    pub fn infect(&mut self) {
        self.citizens_infected += 1
    }

    pub fn get_sneeze_range(self) -> f32 {
        self.sneeze_range
    }

    pub fn get_infected(self) -> i32 {
        self.citizens_infected
    }

    pub fn check_if_sneezing(self) -> bool {
        self.is_sneezing
    }

    pub fn sneeze(&mut self) {
        if self.is_sneezing {
            if self.sneeze_range <= self.sneeze_max {
                self.sneeze_range += 10.0;
            } else {
                self.sneeze_range = 0.0;
            }
        }
    }

    pub fn set_sneeze(&mut self, s: bool) {
        self.is_sneezing = s;
    }
}

impl Moveable for Player {
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
            Zone::LeftBorder => self.ent_params.set_cx(width + r),
            Zone::RightBorder => self.ent_params.set_cx(-r),
            Zone::BottomBorder => self.ent_params.set_cy(-r),
            Zone::UpBorder => self.ent_params.set_cy(height + r),
            Zone::Inside => self.ent_params.move_step(),
        }
    }
}

pub fn default_player((width, height): (f32, f32)) -> Player {
    let params: EntityParams = EntityParams::new(
        na::Point2::new(width / 2.0, height / 2.0),
        20.0,
        na::Vector2::new(0.0, 0.0),
        5.0,
    );
    Player {
        ent_params: params,
        sneeze_range: 5.0,
        sneeze_max: 30.0,
        is_sneezing: false,
        citizens_infected: 0,
    }
}
