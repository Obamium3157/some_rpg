use ggez::{Context, GameError};
use ggez::event::EventHandler;

const SCREEN_WIDTH: f32 = 640.;
const SCREEN_HEIGHT: f32 = 480.;

pub struct Game { }

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
        ctx.gfx.set_window_title("Project A");
        ctx.gfx.set_drawable_size(SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();

        Game { }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
}