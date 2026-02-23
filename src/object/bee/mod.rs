use crate::activity::Activity;
use ggez::Context;
use ggez::glam::{Vec2, vec2};
use ggez::graphics::Image;
use rand::random_range;

pub struct Bee {
    activity: Option<Activity>,
    image: Image,
}

impl Bee {
    pub fn new(image: Image) -> Self {
        Self {
            activity: None,
            image,
        }
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn dest(&mut self, ctx: &mut Context) -> Vec2 {
        match &self.activity {
            Some(activity) => {
                let changed = activity.move_to_left(ctx.time.delta().as_secs_f32());
                let pos = changed.pos();
                if activity.reached_end_of_window(-(self.image.width() as f32)) {
                    self.activity = None;
                } else {
                    self.activity = Some(changed);
                }
                pos
            }
            None => {
                let pos = vec2(2000.0, random_range(500.0..=999.0));
                self.activity = Option::from(Activity::new(random_range(200.0..=400.0), pos));
                pos
            }
        }
    }
}
