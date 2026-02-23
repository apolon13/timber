use ggez::glam::{vec2, Vec2};

pub struct Activity {
    speed: f32,
    started: bool,
    pos: Vec2
}

impl Activity {
    pub fn new(speed: f32, started: bool, pos: Vec2) -> Self {
        Self{speed , started, pos}
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn started(& self) -> bool {
        self.started
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn sub_x(&mut self, modifier: f32) {
        self.pos = vec2(self.pos.x - modifier, self.pos.y);
    }

    pub fn reached_end_of_window(&self, limit: f32) -> bool {
        self.pos.x < limit
    }
}