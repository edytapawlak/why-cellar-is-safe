use ggez::event::{self, EventHandler, MouseButton};
use ggez::input;
use ggez::nalgebra as na;
use ggez::{graphics, timer, Context, ContextBuilder, GameResult};
use rand::Rng;

mod citizen;
mod player;

const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
const CITIZENT_QUANTITY: i32 = 50;

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Why cellar is safe", "E")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct MyGame {
    p: player::Player,
    citizens: Vec<citizen::Citizen>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let mut l = Vec::new();
        for _ in 0..CITIZENT_QUANTITY {
            l.push(citizen::random_citizen(SCREEN_SIZE));
        }
        MyGame {
            p: player::default_player(),
            citizens: l,
        }
    }

    pub fn draw_circle(
        &self,
        ctx: &mut Context,
        pos_x: f32,
        pos_y: f32,
        radius: f32,
        color: graphics::Color,
    ) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(pos_x, pos_y),
            radius,
            2.0,
            color,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())
    }

    fn is_victim(cit: citizen::Citizen, pl: player::Player) -> bool {
        let player_cent = na::Point2::new(pl.get_x(), pl.get_y());
        let citi_cent = na::Point2::new(cit.get_x(), cit.get_y());
        let dist = na::distance(&player_cent, &citi_cent);
        dist < (pl.get_radius() + pl.get_sneeze_range()) && !cit.get_is_infected()
    }

    fn infection(&mut self) {
        for cit in self.citizens.iter_mut() {
            if MyGame::is_victim(*cit, self.p) {
                cit.infect();
                self.p.infect();
            }
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Change citizen angle every second

        while timer::check_update_time(ctx, 1) {
            let mut rng = rand::thread_rng();
            let r = rng.gen_range(0, 10);
            self.citizens[r].change_angle();
        }

        self.infection();
        self.p
            .move_player(SCREEN_SIZE, input::keyboard::pressed_keys(ctx));
        self.p.sneeze();

        for cit in self.citizens.iter_mut() {
            cit.move_citizen(SCREEN_SIZE);
        }
        Ok(())
    }

    /* DRAWING */
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(
            ctx,
            graphics::Color {
                r: 0.404,
                g: 0.561,
                b: 0.220,
                a: 1.0,
            },
        );

        // Draw player
        self.draw_circle(
            ctx,
            self.p.get_x(),
            self.p.get_y(),
            self.p.get_radius(),
            graphics::Color {
                r: 0.8,
                g: 0.624,
                b: 0.353,
                a: 1.0,
            },
        )?;
        // Draw citizens
        for cit in self.citizens.iter() {
            let col = if cit.get_is_infected() {
                graphics::Color {
                    r: 0.514,
                    g: 0.004,
                    b: 0.145,
                    a: 1.0,
                }
            } else {
                graphics::Color {
                    r: 0.0,
                    g: 0.2,
                    b: 0.0,
                    a: 0.9,
                }
            };
            self.draw_circle(ctx, cit.get_x(), cit.get_y(), cit.get_radius(), col)?;
        }

        if self.p.check_if_sneezing() {
            // Draw sneezing
            self.draw_circle(
                ctx,
                self.p.get_x(),
                self.p.get_y(),
                self.p.get_radius() + self.p.get_sneeze_range(),
                graphics::Color {
                    r: 0.2,
                    g: 0.2,
                    b: 0.2,
                    a: 0.3,
                },
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
