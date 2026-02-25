use ggez::glam::{Vec2, vec2};

#[derive(Copy, Clone)]
pub enum DirectionMovement {
    Left,
    Right,
}

const LEFT_POSITION: f32 = 1920.0;
const RIGHT_POSITION: f32 = 0.0;

pub struct HorizontalActivity {
    speed: f32,
    y_pos: f32,
    x_pos: f32,
    direction: DirectionMovement,
    object_w: f32,
}

impl HorizontalActivity {
    pub fn new(speed: f32, y_pos: f32, direction: DirectionMovement, object_w: f32) -> Self {
        Self {
            speed,
            y_pos,
            direction,
            object_w,
            x_pos: match direction {
                DirectionMovement::Left => LEFT_POSITION + object_w,
                DirectionMovement::Right => RIGHT_POSITION - object_w,
            },
        }
    }

    pub fn pos(&self) -> Vec2 {
        vec2(self.x_pos, self.y_pos)
    }

    pub fn tick(&mut self, dt: f32) {
        match self.direction {
            DirectionMovement::Left => self.x_pos -= self.speed * dt,
            DirectionMovement::Right => self.x_pos += self.speed * dt,
        }
    }

    pub fn reached_end(&self) -> bool {
        match self.direction {
            DirectionMovement::Left => self.x_pos < RIGHT_POSITION - self.object_w,
            DirectionMovement::Right => self.x_pos > LEFT_POSITION + self.object_w,
        }
    }
}
