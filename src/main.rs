mod activity;
mod object;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::glam::vec2;
use ggez::graphics::{self, Canvas, Color, Image};
use ggez::{Context, GameResult};
use std::path::Path;
use crate::object::bee::Bee;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

struct GameState {
    background: Image,
    tree: Image,
    bee: Bee,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let background = Image::from_path(ctx, "/graphics/background.png")?;
        let tree = Image::from_path(ctx, "/graphics/tree.png")?;
        let bee = Bee::new(Image::from_path(ctx, "/graphics/bee.png")?);
        Ok(Self {
            background,
            tree,
            bee,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(30, 30, 46));
        canvas.draw(&self.background, graphics::DrawParam::default());
        canvas.draw(
            &self.tree,
            graphics::DrawParam::new().dest(vec2(810.0, 0.0)),
        );
        let bee_dest = self.bee.dest(ctx);
        canvas.draw(
            self.bee.image(),
            graphics::DrawParam::new().dest(bee_dest),
        );
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("game", "author")
        .window_setup(WindowSetup::default().title("Game"))
        .add_resource_path(Path::new("./resources"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
