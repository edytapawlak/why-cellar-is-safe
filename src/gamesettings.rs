use ggez::graphics;

#[derive(Copy, Clone)]
pub struct GameSettings {
    screen_width: f32,
    screen_height: f32,
    citizen_quantity: i32,
    bg_color: graphics::Color,
    player_color: graphics::Color,
    health_color: graphics::Color,
    disease_color: graphics::Color,
    sneeze_color: graphics::Color,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            screen_width: 800.0,
            screen_height: 600.0,
            citizen_quantity: 50,
            bg_color: graphics::Color {
                r: 0.404,
                g: 0.561,
                b: 0.220,
                a: 1.0,
            },
            player_color: graphics::Color {
                r: 0.8,
                g: 0.624,
                b: 0.353,
                a: 1.0,
            },
            health_color: graphics::Color {
                r: 0.0,
                g: 0.2,
                b: 0.0,
                a: 1.0,
            },
            disease_color: graphics::Color {
                r: 0.514,
                g: 0.004,
                b: 0.145,
                a: 1.0,
            },
            sneeze_color: graphics::Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 0.3,
            },
        }
    }
}
impl GameSettings {
    pub fn get_player_col(self) -> graphics::Color {
        self.player_color
    }

    pub fn get_bg_col(self) -> graphics::Color {
        self.bg_color
    }

    pub fn get_disease_color(self) -> graphics::Color {
        self.disease_color
    }

    pub fn get_health_col(self) -> graphics::Color {
        self.health_color
    }

    pub fn get_citizens_quan(self) -> i32 {
        self.citizen_quantity
    }

    pub fn get_sneeze_color(self) -> graphics::Color {
        self.sneeze_color
    }

    pub fn get_screen_width(self) -> f32 {
        self.screen_width
    }
    pub fn get_screen_height(self) -> f32 {
        self.screen_height
    }
}
