use ggez::{graphics, Context, GameResult};
use ggez::event::EventHandler;
use ggez::glam::vec2;
use ggez::graphics::{Canvas, Color, Drawable, FontData, Image, PxScale, Text, TextLayout};
use ggez::input::keyboard;
use crate::activity::DirectionMovement;
use crate::object::activity::{ActivityObject};

pub struct State {
    background: Image,
    tree: Image,
    bee: ActivityObject,
    clouds: Vec<ActivityObject>,
    paused: bool,
    scores: i32,
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        ctx.gfx.add_font("komikap", FontData::from_path(ctx, "/fonts/KOMIKAP_.ttf")?);
        Ok(Self {
            scores: 0,
            paused: true,
            background: Image::from_path(ctx, "/graphics/background.png")?,
            tree: Image::from_path(ctx, "/graphics/tree.png")?,
            bee: ActivityObject::new(Image::from_path(ctx, "/graphics/bee.png")?, 500.0..999.0, DirectionMovement::Left),
            clouds: vec![
                ActivityObject::new(Image::from_path(ctx, "/graphics/cloud.png")?, 0.0..150.0, DirectionMovement::Left),
                ActivityObject::new(Image::from_path(ctx, "/graphics/cloud.png")?, 150.0..250.0, DirectionMovement::Right),
                ActivityObject::new(Image::from_path(ctx, "/graphics/cloud.png")?, 250.0..350.0, DirectionMovement::Left),
            ],
        })
    }

    fn draw_objects(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        if self.paused {
            let (screen_w, screen_h) = ctx.gfx.drawable_size();
            Text::new("Press Spase to be continue")
                .set_font("komikap")
                .set_layout(TextLayout::center())
                .set_scale(PxScale::from(120.0))
                .draw(canvas, graphics::DrawParam::new().dest(vec2(screen_w / 2.0, screen_h / 2.0)));
            return Ok(())
        }
        Text::new(format!("Scores: {}", self.scores))
            .set_font("komikap")
            .set_scale(PxScale::from(40.0))
            .draw(canvas, graphics::DrawParam::new().dest(vec2(20.0, 20.0)));

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
        Ok(())
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Space) {
            self.paused = !self.paused;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(30, 30, 46));
        canvas.draw(&self.background, graphics::DrawParam::default());
        canvas.draw(
            &self.tree,
            graphics::DrawParam::new().dest(vec2(810.0, 0.0)),
        );
        self.draw_objects(ctx, &mut canvas)?;
        Ok(canvas.finish(ctx)?)
    }
}
