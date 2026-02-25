mod motion;
mod object;
mod game;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self};
use ggez::{GameResult};
use std::path::Path;
use ggez::graphics::FontData;
use crate::game::State;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("timber", "pankov")
        .window_setup(WindowSetup::default().title("Timber"))
        .add_resource_path(Path::new("./resources"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    
    ctx.gfx.add_font("komikap", FontData::from_path(&ctx, "/fonts/KOMIKAP_.ttf")?);
    let state = State::new(&ctx)?;
    event::run(ctx, event_loop, state)
}
