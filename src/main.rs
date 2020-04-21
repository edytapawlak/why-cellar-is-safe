use ggez::event::{self, EventHandler, MouseButton};
use ggez::input;
use ggez::nalgebra as na;
use ggez::{graphics, Context, ContextBuilder, GameResult};

mod player;
mod citizen;

const SCREEN_SIZE: (f32, f32) = (600.0, 800.0);

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
    citizen : citizen::Citizen,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            p: player::default_player(),
            citizen: citizen::random_citizen(SCREEN_SIZE),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.p
            .move_player(SCREEN_SIZE, input::keyboard::pressed_keys(ctx));
        self.p.sneeze();
        self.citizen.move_citizen();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        let player_body = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.p.get_x(), self.p.get_y()),
            self.p.get_width() / 2.0,
            2.0,
            graphics::BLACK,
        )?;
        let citizen_body = graphics::Mesh::new_circle(
          ctx,
          graphics::DrawMode::fill(),
          na::Point2::new(self.citizen.get_x(), self.citizen.get_y()),
          self.citizen.get_radius(),
          2.0,
          graphics::Color {
            r: 0.2,
            g: 0.0,
            b: 0.0,
            a: 0.9,
        },
      )?;
        let sneeze = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.p.get_x(), self.p.get_y()),
            self.p.get_width() + self.p.get_sneeze_range(),
            2.0,
            graphics::Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 0.3,
            },
        )?;

        graphics::draw(ctx, &player_body, graphics::DrawParam::default())?;
        graphics::draw(ctx, &citizen_body, graphics::DrawParam::default())?;
        if self.p.is_sneezing {
            graphics::draw(ctx, &sneeze, graphics::DrawParam::default())?;
        }
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
        self.citizen = citizen::random_citizen(SCREEN_SIZE);
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
