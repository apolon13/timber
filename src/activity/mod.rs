use ggez::glam::{vec2, Vec2};

pub struct Activity {
    speed: f32,
    pos: Vec2
}

impl Activity {
    pub fn new(speed: f32, pos: Vec2) -> Self {
        Self{speed, pos}
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn move_to_left(&self, modifier: f32) -> Activity {
        Activity{speed: self.speed, pos: vec2(self.pos.x - (self.speed * modifier), self.pos.y)}
    }

    pub fn reached_end(&self, limit: f32) -> bool {
        self.pos.x < limit
    }
}