use ggez::{ContextBuilder, GameResult};
use crate::game::Game;

mod game;

const GAME_ID: &'static str = "project a";
const GAME_AUTHORS: &'static str = "PoZicOn & Obamium3157";

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new(GAME_ID, GAME_AUTHORS)
        .add_resource_path("resources")
        .build()
        .unwrap();

    let game = Game::new(&mut ctx);

    ggez::event::run(ctx, event_loop, game)
}
