use ggez::glam::{Vec2, vec2};
use ggez::graphics;
use ggez::graphics::{Canvas, Image};
use rand::random_range;

const TREE_X: f32 = 810.0;
const BRANCH_LEFT_X: f32 = 810.0;
const BRANCH_RIGHT_X: f32 = 1100.0;
const BRANCH_SPEED: f32 = 200.0;
const SPAWN_INTERVAL: f32 = 2.0;
const SPAWN_Y: f32 = -50.0;
const MAX_Y: f32 = 850.0;

#[derive(Copy, Clone)]
enum BranchSide {
    Left,
    Right,
}

pub struct Tree {
    trunk_image: Image,
    branch_image: Image,
    max_branches: usize,
    branches: Vec<Branch>,
    spawn_timer: f32,
}

impl Tree {
    pub fn new(trunk: Image, branch: Image) -> Tree {
        Tree {
            trunk_image: trunk,
            branch_image: branch,
            max_branches: 3,
            branches: Vec::new(),
            spawn_timer: SPAWN_INTERVAL,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        for branch in &mut self.branches {
            branch.tick(dt);
        }
        self.branches.retain(|b| !b.is_offscreen());

        self.spawn_timer += dt;
        if self.spawn_timer >= SPAWN_INTERVAL && self.branches.len() < self.max_branches {
            self.branches.push(Branch::new());
            self.spawn_timer = 0.0;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(
            &self.trunk_image,
            graphics::DrawParam::new().dest(vec2(TREE_X, 0.0)),
        );
        for branch in &self.branches {
            canvas.draw(
                &self.branch_image,
                graphics::DrawParam::new()
                    .dest(branch.dest())
                    .rotation(branch.rotation()),
            );
        }
    }
}

struct Branch {
    x: f32,
    y: f32,
    side: BranchSide,
}

impl Branch {
    fn new() -> Branch {
        let side = match random_range(0..100) {
            0..50 => BranchSide::Left,
            _ => BranchSide::Right,
        };
        Branch {
            x: match side {
                BranchSide::Left => BRANCH_LEFT_X,
                BranchSide::Right => BRANCH_RIGHT_X,
            },
            y: SPAWN_Y,
            side,
        }
    }

    fn tick(&mut self, dt: f32) {
        self.y += BRANCH_SPEED * dt;
    }

    fn is_offscreen(&self) -> bool {
        self.y > MAX_Y
    }

    fn dest(&self) -> Vec2 {
        vec2(self.x, self.y)
    }

    fn rotation(&self) -> f32 {
        match self.side {
            BranchSide::Left => 180.0_f32.to_radians(),
            BranchSide::Right => 0.0,
        }
    }
}
