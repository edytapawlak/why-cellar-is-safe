use ggez::event::{self, EventHandler, MouseButton};
use ggez::input;
use ggez::nalgebra as na;
use ggez::{graphics, timer, Context, ContextBuilder, GameResult};
use infectable::Infectable;
use moveable::Moveable;
use rand::Rng;

mod ambulance;
mod citizen;
mod gamesettings;
mod infectable;
mod moveable;
mod player;

fn main() {
    // Make settings, context and the game.
    let settings = gamesettings::GameSettings::default();
    let (mut ctx, mut event_loop) = ContextBuilder::new("Why cellar is safe", "E")
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(settings.get_screen_width(), settings.get_screen_height()),
        )
        .build()
        .expect("aieee, could not create ggez context!");

    let mut my_game = MyGame::new(settings);

    // Run.
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct MyGame {
    settings: gamesettings::GameSettings,
    p: player::Player,
    citizens: Vec<citizen::Citizen>,
    ambulance: ambulance::Ambulance,
}

impl MyGame {
    pub fn new(settings: gamesettings::GameSettings) -> MyGame {
        // List of random citizens.
        let mut l = Vec::new();
        for i in 0..(settings.get_citizens_quan()) {
            l.push(citizen::random_citizen(
                settings.get_screen_width(),
                settings.get_screen_height(),
                i as usize,
            ));
        }

        MyGame {
            settings,
            p: player::init(settings.get_screen_width(), settings.get_screen_height()),
            citizens: l,
            ambulance: ambulance::new(settings.get_screen_width(), settings.get_screen_height(), na::Point2::new(400.0, 00.0)),
        }
    }

    fn is_victim(cit: citizen::Citizen, pl: player::Player) -> bool {
        let player_cent = pl.get_position();
        let citi_cent = cit.get_position();
        let dist = na::distance(&player_cent, &citi_cent);
        dist < (pl.get_radius() + pl.get_sneeze_range())
    }

    fn infection(&mut self) {
        for cit in self.citizens.iter_mut() {
            if MyGame::is_victim(*cit, self.p) {
                cit.become_infected();
                if cit.needs_doctor() && self.ambulance.is_free() {
                    cit.stop();
                    self.p.infect();
                    self.ambulance.set_destination(
                        cit.get_id(),
                        cit.get_position(),
                        self.settings.get_screen_width(),
                        self.settings.get_screen_height()
                    );
                }
            } else {
                cit.cure();
            }
        }
    }
}
impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let swidth = self.settings.get_screen_width();
        let sheight = self.settings.get_screen_height();

        // Choose citizen randomly and change his angle.
        while timer::check_update_time(ctx, 1) {
            let mut rng = rand::thread_rng();
            let r = rng.gen_range(0, 10);
            self.citizens[r].change_angle();
        }

        self.infection();
        match self.ambulance.get_patient_id() {
            Some(id) => {
                if na::distance(
                    &self.ambulance.get_position(),
                    &self.citizens[id].get_position(),
                ) < 2.0 * self.ambulance.get_radius()
                {
                    self.citizens[id].go_hospital(self.ambulance.get_position());
                }
            }
            None => (),
        }

        self.p
            .move_player( swidth, sheight, input::keyboard::pressed_keys(ctx));

        self.p.sneeze();

        for cit in self.citizens.iter_mut() {
            cit.move_being(swidth, swidth);
        }

        self.ambulance.move_being(swidth, sheight);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, self.settings.get_bg_col());

        // Player drawing.
        self.p.draw_player(ctx, self.settings.get_player_col())?;
        // Citizens drawing.
        for cit in self.citizens.iter() {
            cit.draw_citizen(
                ctx,
                self.settings.get_disease_color(),
                self.settings.get_health_col(),
            )?;
        }

        if self.p.check_if_sneezing() {
            // Draw sneeze range.
            self.p
                .draw_sneezing(ctx, self.settings.get_sneeze_color())?;
        }

        // Ambulance drawing.
        self.ambulance.draw_ambulance(ctx)?;

        // Draw stats.
        let score = graphics::Text::new((
            format!("Infected: {} ", self.p.get_infected().to_string()),
            graphics::Font::default(),
            24.0,
        ));
        graphics::draw(ctx, &score, graphics::DrawParam::default())?;

        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.p.set_sneeze(true);
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.p.set_sneeze(false);
    }
}
