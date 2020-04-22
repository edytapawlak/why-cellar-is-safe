use ggez::event::{self, EventHandler, MouseButton};
use ggez::input;
use ggez::nalgebra as na;
use ggez::{graphics, timer, Context, ContextBuilder, GameResult};
use movingbeing::MovingBeing;
use rand::Rng;

mod citizen;
mod gamesettings;
mod movingbeing;
mod player;

fn main() {
    /* Make settings, context and the game */
    let settings = gamesettings::GameSettings::default();
    let (mut ctx, mut event_loop) = ContextBuilder::new("Why cellar is safe", "E")
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(settings.get_screen_width(), settings.get_screen_height()),
        )
        .build()
        .expect("aieee, could not create ggez context!");

    let mut my_game = MyGame::new(&mut ctx, settings);

    /* Run */
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct MyGame {
    settings: gamesettings::GameSettings,
    p: player::Player,
    citizens: Vec<citizen::Citizen>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context, settings: gamesettings::GameSettings) -> MyGame {
        /* List of random citizens */
        let mut l = Vec::new();
        for _ in 0..(settings.get_citizens_quan()) {
            l.push(citizen::random_citizen(settings.get_screen_size()));
        }
        MyGame {
            settings,
            p: player::default_player(settings.get_screen_size()),
            citizens: l,
        }
    }

    pub fn draw_circle(
        &self,
        ctx: &mut Context,
        pos: na::Point2<f32>,
        radius: f32,
        color: graphics::Color,
    ) -> GameResult<()> {
        let circle =
            graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), pos, radius, 2.0, color)?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())
    }

    fn is_victim(cit: citizen::Citizen, pl: player::Player) -> bool {
        let player_cent = pl.get_position();
        let citi_cent = cit.get_position();
        let dist = na::distance(&player_cent, &citi_cent);
        dist < (pl.get_radius() + pl.get_sneeze_range()) && !cit.get_is_infected()
    }

    fn infection(&mut self) {
        for cit in self.citizens.iter_mut() {
            if MyGame::is_victim(*cit, self.p) {
                cit.become_infected();
                self.p.infect();
            }
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        /* Choose citizen randomly and change his angle  */
        while timer::check_update_time(ctx, 1) {
            let mut rng = rand::thread_rng();
            let r = rng.gen_range(0, 10);
            self.citizens[r].change_angle();
        }

        self.infection();
        self.p.move_player(
            self.settings.get_screen_size(),
            input::keyboard::pressed_keys(ctx),
        );
        self.p.sneeze();

        for cit in self.citizens.iter_mut() {
            cit.move_being(self.settings.get_screen_size());
        }
        Ok(())
    }

    /* DRAWING */
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, self.settings.get_bg_col());

        /* Player drawing */
        self.draw_circle(
            ctx,
            self.p.get_position(),
            self.p.get_radius(),
            self.settings.get_player_col(),
        )?;
        /* Citizens drawing */
        for cit in self.citizens.iter() {
            let col = if cit.get_is_infected() {
                self.settings.get_disease_color()
            } else {
                self.settings.get_health_col()
            };
            self.draw_circle(ctx, cit.get_position(), cit.get_radius(), col)?;
        }

        if self.p.check_if_sneezing() {
            /* Draw sneeze range */
            self.draw_circle(
                ctx,
                self.p.get_position(),
                self.p.get_radius() + self.p.get_sneeze_range(),
                self.settings.get_sneeze_color(),
            )?;
        }

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
