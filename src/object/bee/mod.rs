use crate::activity::Activity;
use ggez::Context;
use ggez::glam::{Vec2, vec2};
use ggez::graphics::Image;
use rand::random_range;

pub struct Bee {
    activity: Activity,
    image: Image,
}

impl Bee {
    pub fn new(image: Image) -> Self {
        Self {
            activity: Activity::new(0.0, false, vec2(0.0, 0.0)),
            image,
        }
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn dest(&mut self, ctx: &mut Context) -> Vec2 {
        if !self.activity.started() {
            self.activity = Activity::new(
                random_range(200.0..=400.0),
                true,
                vec2(2000.0, random_range(500.0..=999.0)),
            );
        } else {
            self.activity
                .sub_x(self.activity.speed() * ctx.time.delta().as_secs_f32());
            if self.activity.reached_end_of_window(-(self.image.width() as f32)) {
                self.activity = Activity::new(0.0, false, vec2(0.0, 0.0));
            }
        }
        self.activity.pos()
    }
}
