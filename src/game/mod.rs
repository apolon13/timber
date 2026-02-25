use crate::motion::DirectionMovement;
use crate::object::scenic::ScenicObject;
use crate::object::player::{Player, PlayerSide};
use crate::object::tree::Tree;
use ggez::event::EventHandler;
use ggez::glam::vec2;
use ggez::graphics::{Canvas, Color, Drawable, Image, PxScale, Text, TextAlign, TextLayout};
use ggez::input::keyboard;
use ggez::{Context, GameResult, graphics};

const DEFAULT_GAME_TIME_IN_SECONDS: f32 = 360.0;
const HUD_MARGIN: f32 = 20.0;
const TITLE_FONT_SIZE: f32 = 120.0;
const HUD_FONT_SIZE: f32 = 40.0;

#[derive(Clone, Copy)]
enum GamePhase {
    Paused,
    Playing,
    GameOver,
}

pub struct State {
    background: Image,
    tree: Tree,
    bee: ScenicObject,
    clouds: Vec<ScenicObject>,
    phase: GamePhase,
    player: Player,
    scores: i32,
    time_remaining: f32,
}

impl State {
    fn make_tree(ctx: &Context) -> GameResult<Tree> {
        Ok(Tree::new(
            Image::from_path(ctx, "/graphics/tree.png")?,
            Image::from_path(ctx, "/graphics/branch.png")?,
        ))
    }

    fn make_bee(ctx: &Context) -> GameResult<ScenicObject> {
        Ok(ScenicObject::new(
            Image::from_path(ctx, "/graphics/bee.png")?,
            500.0..999.0,
            DirectionMovement::Left,
        ))
    }

    fn make_player(ctx: &Context) -> GameResult<Player> {
        Ok(Player::new(
            Image::from_path(ctx, "/graphics/player.png")?,
            Image::from_path(ctx, "/graphics/rip.png")?
        ))
    }

    fn make_clouds(ctx: &Context) -> GameResult<Vec<ScenicObject>> {
        let cloud_img = Image::from_path(ctx, "/graphics/cloud.png")?;
        Ok(vec![
            ScenicObject::new(cloud_img.clone(), 0.0..150.0, DirectionMovement::Left),
            ScenicObject::new(cloud_img.clone(), 150.0..250.0, DirectionMovement::Right),
            ScenicObject::new(cloud_img, 250.0..350.0, DirectionMovement::Left),
        ])
    }

    pub fn new(ctx: &Context) -> GameResult<Self> {
        Ok(Self {
            player: Self::make_player(ctx)?,
            phase: GamePhase::Paused,
            time_remaining: DEFAULT_GAME_TIME_IN_SECONDS,
            scores: 0,
            background: Image::from_path(ctx, "/graphics/background.png")?,
            tree: Self::make_tree(ctx)?,
            bee: Self::make_bee(ctx)?,
            clouds: Self::make_clouds(ctx)?,
        })
    }

    fn reset(&mut self, ctx: &Context) -> GameResult<()> {
        self.scores = 0;
        self.phase = GamePhase::Playing;
        self.time_remaining = DEFAULT_GAME_TIME_IN_SECONDS;
        self.tree = Self::make_tree(ctx)?;
        self.bee = Self::make_bee(ctx)?;
        self.clouds = Self::make_clouds(ctx)?;
        self.player = Self::make_player(ctx)?;
        Ok(())
    }

    fn handle_input(&mut self, ctx: &Context) -> GameResult<()> {
        if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Return) {
            self.reset(ctx)?;
        }
        if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::LShift) {
            self.phase = match self.phase {
                GamePhase::Playing => GamePhase::Paused,
                GamePhase::Paused => GamePhase::Playing,
                GamePhase::GameOver => GamePhase::GameOver,
            };
        }
        if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Left) {
            self.player.move_to(PlayerSide::Left);
        }
        if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Right) {
            self.player.move_to(PlayerSide::Right);
        }
        Ok(())
    }

    fn tick(&mut self, dt: f32) {
        if self.player.is_dead() {
            self.phase = GamePhase::GameOver;
        }
        self.time_remaining -= dt;
        if self.time_remaining <= 0.0 {
            self.phase = GamePhase::GameOver;
            return;
        }
        self.tree.grow(dt, || {
            self.scores = self.scores + 1
        });
        self.bee.fly(dt);
        self.player.dodge(&self.tree);
        for cloud in &mut self.clouds {
            cloud.fly(dt);
        }
    }

    fn draw_centered_text(canvas: &mut Canvas, text: &str, screen_w: f32, screen_h: f32) {
        Text::new(text)
            .set_font("komikap")
            .set_layout(TextLayout::center())
            .set_scale(PxScale::from(TITLE_FONT_SIZE))
            .draw(
                canvas,
                graphics::DrawParam::new().dest(vec2(screen_w / 2.0, screen_h / 2.0)),
            );
    }

    fn draw_hud(&self, ctx: &Context, canvas: &mut Canvas) {
        let (screen_w, _) = ctx.gfx.drawable_size();
        Text::new(format!("Scores: {}", self.scores))
            .set_font("komikap")
            .set_scale(PxScale::from(HUD_FONT_SIZE))
            .draw(
                canvas,
                graphics::DrawParam::new().dest(vec2(HUD_MARGIN, HUD_MARGIN)),
            );
        Text::new(format!("Time: {}", self.time_remaining as i32))
            .set_font("komikap")
            .set_scale(PxScale::from(HUD_FONT_SIZE))
            .set_layout(TextLayout {
                h_align: TextAlign::End,
                v_align: TextAlign::Begin,
            })
            .draw(
                canvas,
                graphics::DrawParam::new().dest(vec2(screen_w - HUD_MARGIN, HUD_MARGIN)),
            );
    }

    fn draw_game_objects(&self, canvas: &mut Canvas) {
        self.tree.draw(canvas);
        self.player.draw(canvas);
        canvas.draw(
            self.bee.image(),
            graphics::DrawParam::new().dest(self.bee.position()),
        );
        for cloud in &self.clouds {
            canvas.draw(
                cloud.image(),
                graphics::DrawParam::new().dest(cloud.position()),
            );
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.handle_input(ctx)?;
        if let GamePhase::Playing = self.phase {
            self.tick(ctx.time.delta().as_secs_f32());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(30, 30, 46));
        canvas.draw(&self.background, graphics::DrawParam::default());
        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        match self.phase {
            GamePhase::Paused => {
                Self::draw_centered_text(
                    &mut canvas,
                    "Press Left Shift to continue",
                    screen_w,
                    screen_h,
                );
            }
            GamePhase::GameOver => {
                Self::draw_centered_text(
                    &mut canvas,
                    format!("Scores: {} \n Press Enter", self.scores).as_str(),
                    screen_w,
                    screen_h,
                );
            }
            GamePhase::Playing => {
                self.draw_hud(ctx, &mut canvas);
                self.draw_game_objects(&mut canvas);
            }
        }
        Ok(canvas.finish(ctx)?)
    }
}
