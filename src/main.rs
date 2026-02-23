mod activity;
mod object;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::glam::vec2;
use ggez::graphics::{self, Canvas, Color, Image};
use ggez::{Context, GameResult};
use std::path::Path;
use crate::object::activity::ActivityObject;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

struct GameState {
    background: Image,
    tree: Image,
    bee: ActivityObject,
    clouds: Vec<ActivityObject>,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            background: Image::from_path(ctx, "/graphics/background.png")?,
            tree: Image::from_path(ctx, "/graphics/tree.png")?,
            bee: ActivityObject::new_with_started_pos(Image::from_path(ctx, "/graphics/bee.png")?, 500.0..999.0),
            clouds: vec![
                ActivityObject::new_with_started_pos(Image::from_path(ctx, "/graphics/cloud.png")?, 0.0..150.0),
                ActivityObject::new_with_started_pos(Image::from_path(ctx, "/graphics/cloud.png")?, 150.0..250.0),
                ActivityObject::new_with_started_pos(Image::from_path(ctx, "/graphics/cloud.png")?, 250.0..350.0),
            ],
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
        for cloud in self.clouds.iter_mut() {
            let cloud_dest = cloud.dest(ctx);
            canvas.draw(
                cloud.image(),
                graphics::DrawParam::new().dest(cloud_dest),
            );
        }
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
