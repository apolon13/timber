use ggez::{graphics, Context, GameResult};
use ggez::event::EventHandler;
use ggez::glam::vec2;
use ggez::graphics::{Canvas, Color, Image};
use ggez::input::keyboard;
use crate::object::activity::ActivityObject;

pub struct State {
    background: Image,
    tree: Image,
    bee: ActivityObject,
    clouds: Vec<ActivityObject>,
    paused: bool,
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            paused: false,
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

    fn draw_objects(&mut self, ctx: &mut Context) -> GameResult {
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

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::W) {
            self.paused = !self.paused;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.paused == true {
            return Ok(());
        }
        Ok(self.draw_objects(ctx)?)
    }
}
