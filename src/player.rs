use ggez::event::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Copy)]
pub struct Player {
    pos_x: f32,
    pos_y: f32,
    width: f32,
    height: f32,
    sneeze_range: f32,
    sneeze_max: f32,
    pub is_sneezing: bool,
}

impl Player {
    pub fn move_player(&mut self, (width, height): (f32, f32), pressed_keys: &HashSet<KeyCode>) {
        if pressed_keys.contains(&KeyCode::Left) {
            if self.pos_x + (self.width / 2.0) > 0.0 {
                self.pos_x -= 10.0
            } else {
                self.pos_x = width + self.width / 2.0
            }
        }
        if pressed_keys.contains(&KeyCode::Right) {
            if self.pos_x - (self.width / 2.0) < width {
                self.pos_x += 10.0
            } else {
                self.pos_x = -self.width / 2.0
            }
        }
        if pressed_keys.contains(&KeyCode::Down) {
            if self.pos_y - (self.height / 2.0) < height {
                self.pos_y += 10.0
            } else {
                self.pos_y = -self.height / 2.0
            }
        }
        if pressed_keys.contains(&KeyCode::Up) {
            if self.pos_y + (self.height / 2.0) > 0.0 {
                self.pos_y -= 10.0
            } else {
                self.pos_y = height + self.height / 2.0
            }
        }
    }

    pub fn get_sneeze_range(self) -> f32 {
        self.sneeze_range
    }

    pub fn sneeze(&mut self) {
        if self.is_sneezing {
            if self.sneeze_range <= self.sneeze_max {
                self.sneeze_range += 10.0;
                println!("sneee {}", self.sneeze_range);
            }
        } else {
            self.sneeze_range = 0.0;
        }
    }

    pub fn set_sneeze(&mut self, s: bool) {
        self.is_sneezing = s;
    }

    pub fn get_x(self) -> f32 {
        self.pos_x
    }

    pub fn get_y(self) -> f32 {
        self.pos_y
    }

    pub fn get_width(self) -> f32 {
        self.width
    }
}

pub fn default_player() -> Player {
    Player {
        pos_x: 100.0,
        pos_y: 100.0,
        width: 100.0,
        height: 100.0,
        sneeze_range: 20.0,
        sneeze_max: 100.0,
        is_sneezing: false,
    }
}
