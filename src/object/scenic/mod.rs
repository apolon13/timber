use crate::motion::{DirectionMovement, HorizontalActivity};
use ggez::glam::Vec2;
use ggez::graphics::Image;
use rand::random_range;
use std::ops::Range;

pub struct ScenicObject {
    activity: Option<HorizontalActivity>,
    image: Image,
    pos_range: Range<f32>,
    direction: DirectionMovement,
}

impl ScenicObject {
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

    pub fn fly(&mut self, dt: f32) {
        if let Some(activity) = &mut self.activity {
            activity.tick(dt);
            if activity.reached_end() {
                self.activity = None;
            }
        }
        if self.activity.is_none() {
            self.activity = Some(HorizontalActivity::new(
                random_range(200.0..=400.0),
                random_range(self.pos_range.clone()),
                self.direction,
                self.image.width() as f32,
            ));
        }
    }

    pub fn position(&self) -> Vec2 {
        self.activity.as_ref().map_or(Vec2::ZERO, |a| a.pos())
    }
}
