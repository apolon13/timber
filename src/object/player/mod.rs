use crate::object::tree::Tree;
use ggez::glam::vec2;
use ggez::graphics;
use ggez::graphics::{Canvas, Image, Rect};

const PLAYER_Y: f32 = 700.0;

const GRAVE_Y: f32 = 750.0;
const PLAYER_LEFT_X: f32 = 780.0;
const PLAYER_RIGHT_X: f32 = 1140.0;

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerSide {
    Left,
    Right,
}

pub struct Player {
    body: Image,
    side: PlayerSide,
    x_pos: f32,
    scale_x: f32,
    dead: bool,
    grave: Image
}

impl Player {
    pub fn new(body: Image, grave: Image) -> Self {
        Player {
            grave,
            x_pos: PLAYER_LEFT_X,
            scale_x: -1.0,
            body,
            side: PlayerSide::Left,
            dead: false,
        }
    }

    pub fn move_to(&mut self, side: PlayerSide) {
        if self.dead {
            return;
        }
        self.side = side;
    }

    pub fn dodge(&mut self, tree: &Tree) {
        let (x, scale_x) = match self.side {
            PlayerSide::Left => (PLAYER_LEFT_X, -1.0),
            PlayerSide::Right => (PLAYER_RIGHT_X, 1.0),
        };
        self.x_pos = x;
        self.scale_x = scale_x;

        let rect = match self.side {
            PlayerSide::Left => Rect::new(x - 150.0, PLAYER_Y, 150.0, 192.0),
            PlayerSide::Right => Rect::new(x, PLAYER_Y, 150.0, 192.0),
        };
        if tree.hit_by_a_branch(&rect) {
            self.dead = true;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        if self.dead {
            canvas.draw(
                &self.grave,
                graphics::DrawParam::new()
                    .dest(vec2(self.x_pos, GRAVE_Y))
                    .scale(vec2(self.scale_x, 1.0)),
            );
        } else {
            canvas.draw(
                &self.body,
                graphics::DrawParam::new()
                    .dest(vec2(self.x_pos, PLAYER_Y))
                    .scale(vec2(self.scale_x, 1.0)),
            );
        }
    }
}
