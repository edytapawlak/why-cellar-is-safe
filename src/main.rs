use ggez::event::{self, EventHandler, MouseButton};
use ggez::input;
use ggez::nalgebra as na;
use ggez::{graphics, timer, Context, ContextBuilder, GameResult};
use std::collections::LinkedList;

mod citizen;
mod player;

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
    citizen: citizen::Citizen,
    citizens: LinkedList<citizen::Citizen>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            p: player::default_player(),
            citizen: citizen::random_citizen(SCREEN_SIZE),
            citizens: LinkedList::new(),
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

    fn filter_citizens(&mut self) {
        self.citizens = self
            .citizens
            .iter()
            .filter(|cit| ! (cit.is_outside(SCREEN_SIZE)))
            .map(|&x| x)
            .collect();
    }
    fn change_citizens_angle(&mut self) {
      
      for cit in self.citizens.iter_mut() {
        cit.change_angle();
        
      }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Change citizen angle every second
     
        while timer::check_update_time(ctx, 1) {
            self.citizens
                .push_front(citizen::random_citizen(SCREEN_SIZE));
            self.change_citizens_angle();
        }

        self.p
            .move_player(SCREEN_SIZE, input::keyboard::pressed_keys(ctx));
        self.p.sneeze();

        self.filter_citizens();

        for cit in self.citizens.iter_mut() {
            cit.move_citizen();
        }
        self.citizen.move_citizen();
        Ok(())
    }

    /* DRAWING */
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        // Draw player
        self.draw_circle(
            ctx,
            self.p.get_x(),
            self.p.get_y(),
            self.p.get_width() / 2.0,
            graphics::BLACK,
        )?;
        // Draw citizen
        self.draw_circle(
            ctx,
            self.citizen.get_x(),
            self.citizen.get_y(),
            self.citizen.get_radius(),
            graphics::Color {
                r: 0.2,
                g: 0.0,
                b: 0.0,
                a: 0.9,
            },
        )?;

        for cit in self.citizens.iter() {
            self.draw_circle(
                ctx,
                cit.get_x(),
                cit.get_y(),
                cit.get_radius(),
                graphics::Color {
                    r: 0.2,
                    g: 0.0,
                    b: 0.0,
                    a: 0.9,
                },
            )?;
        }

        if self.p.is_sneezing {
            // Draw sneezing
            self.draw_circle(
                ctx,
                self.p.get_x(),
                self.p.get_y(),
                self.p.get_width() + self.p.get_sneeze_range(),
                graphics::Color {
                    r: 0.2,
                    g: 0.2,
                    b: 0.2,
                    a: 0.3,
                },
            )?;
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
