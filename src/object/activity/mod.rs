use crate::activity::{Activity, DirectionMovement};
use ggez::Context;
use ggez::glam::Vec2;
use ggez::graphics::Image;
use rand::random_range;
use std::ops::Range;

pub struct ActivityObject {
    activity: Option<Activity>,
    image: Image,
    pos_range: Range<f32>,
    direction: DirectionMovement,
}

impl ActivityObject {
    pub fn new(image: Image, pos_range: Range<f32>, direction: DirectionMovement) -> Self {
        Self {
            activity: None,
            image,
            pos_range,
            direction,
        }
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn dest(&mut self, ctx: &mut Context) -> Vec2 {
        match &self.activity {
            Some(activity) => {
                let changed = activity.move_to_direction(ctx.time.delta().as_secs_f32());
                let pos = changed.pos();
                if activity.reached_end() {
                    self.activity = None;
                } else {
                    self.activity = Some(changed);
                }
                pos
            }
            None => {
                let activity = Activity::new(
                    random_range(200.0..=400.0),
                    random_range(self.pos_range.clone()),
                    self.direction,
                    self.image.width() as f32,
                );
                let pos = activity.pos();
                self.activity = Option::from(activity);
                pos
            }
        }
    }
}
