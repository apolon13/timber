use ggez::glam::{Vec2, vec2};
use ggez::graphics;
use ggez::graphics::{Canvas, Image, Rect};
use rand::random_range;

const TREE_X: f32 = 810.0;
const BRANCH_LEFT_X: f32 = 810.0;
const BRANCH_RIGHT_X: f32 = 1100.0;
const BRANCH_SPEED: f32 = 400.0;
const SPAWN_Y: f32 = -50.0;
const MAX_Y: f32 = 850.0;
const TRAVEL_DISTANCE: f32 = MAX_Y - SPAWN_Y;
const BRANCH_SPEED_STEP: f32 = 20.0;

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
    time_passed: f32,
    next_speed_step: f32,
    branch_speed: f32,
}

impl Tree {
    pub fn new(trunk: Image, branch: Image) -> Tree {
        let mut tree = Tree {
            branch_speed: BRANCH_SPEED,
            next_speed_step: BRANCH_SPEED_STEP,
            time_passed: 0.0,
            trunk_image: trunk,
            branch_image: branch,
            max_branches: 3,
            branches: Vec::new(),
            spawn_timer: 0.0,
        };
        tree.spawn_timer = tree.spawn_interval();
        tree
    }

    fn spawn_interval(&self) -> f32 {
        TRAVEL_DISTANCE / (self.branch_speed * self.max_branches as f32)
    }

    pub fn grow<F: FnMut()>(&mut self, dt: f32, mut on_branch_spawn: F) {
        self.time_passed += dt;
        if self.time_passed > self.next_speed_step {
            self.branch_speed = self.branch_speed + 20.0;
            self.next_speed_step = self.next_speed_step + BRANCH_SPEED_STEP;
        }
        for branch in &mut self.branches {
            branch.tick(dt * self.branch_speed);
        }
        self.branches.retain(|b| !b.is_offscreen());

        self.spawn_timer += dt;
        let interval = self.spawn_interval();
        if self.spawn_timer >= interval && self.branches.len() < self.max_branches {
            self.branches.push(Branch::new());
            self.spawn_timer = 0.0;
            on_branch_spawn();
        }
    }

    pub fn hit_by_a_branch(&self, rect: &Rect) -> bool {
        for branch in self.branches.iter() {
            if branch.hit(rect) {
                return true;
            }
        }
        false
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

    fn tick(&mut self, speed: f32) {
        self.y += speed;
    }

    fn is_offscreen(&self) -> bool {
        self.y > MAX_Y
    }

    fn dest(&self) -> Vec2 {
        vec2(self.x, self.y)
    }

    fn hit(&self, player: &Rect) -> bool {
        let rect = match self.side {
            BranchSide::Left => Rect::new(self.x - 440.0, self.y - 80.0, 440.0, 80.0),
            BranchSide::Right => Rect::new(self.x, self.y, 440.0, 80.0),
        };
        rect.overlaps(player)
    }

    fn rotation(&self) -> f32 {
        match self.side {
            BranchSide::Left => 180.0_f32.to_radians(),
            BranchSide::Right => 0.0,
        }
    }
}
