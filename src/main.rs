use chess_lib::game::Game;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, ContextBuilder, GameResult};

const TILE_SIZE: f32 = 125.0;
const BLACK_COLOR: Color = Color::BLACK;
const WHITE_COLOR: Color = Color::WHITE;

fn main() -> GameResult {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Chess", "Cool Game Author")
        .window_setup(WindowSetup::default().title("Cool chess game"))
        .window_mode(WindowMode::default().dimensions(1000.0, 1000.0))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = Chess::new(&mut ctx)?;

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct Chess {
    game: Game,
    grid: graphics::Mesh,
}

impl Chess {
    pub fn new(ctx: &mut Context) -> GameResult<Chess> {
        // Load/create resources such as images here.
        let mb = &mut graphics::MeshBuilder::new();
        for x in 0..8 {
            for y in 0..8 {
                let color = match x % 2 {
                    0 => {
                        if y % 2 == 0 {
                            WHITE_COLOR
                        } else {
                            BLACK_COLOR
                        }
                    }
                    _ => {
                        if y % 2 == 0 {
                            BLACK_COLOR
                        } else {
                            WHITE_COLOR
                        }
                    }
                };
                mb.rectangle(
                    graphics::DrawMode::fill(),
                    Rect::new(
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    ),
                    color,
                )?;
            }
        }
        Ok(Chess {
            // ...
            game: Game::new(None),
            grid: graphics::Mesh::from_data(ctx, mb.build()),
        })
    }
}

impl EventHandler for Chess {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&self.grid, Vec2::new(0.0, 0.0));
        // Draw code here...
        canvas.finish(ctx)
    }
}
