use chess_lib::board::pieces::{get_legal_moves, move_piece, Color, Move, Piece, PieceType};
use chess_lib::game::Game;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{self, Canvas, DrawParam, Rect};
use ggez::{Context, ContextBuilder, GameResult};
use grid::{Grid, GridPosition};

pub mod grid;

const TILE_SIZE: f32 = 75.0;
const BLACK_COLOR: graphics::Color = graphics::Color::BLACK;
const WHITE_COLOR: graphics::Color = graphics::Color::WHITE;

// Start x, y of grid
const GRID_X: f32 = 0.0;
const GRID_Y: f32 = 0.0;

fn main() -> GameResult {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Chess", "Cool Game Author")
        .window_setup(
            WindowSetup::default()
                .title("Cool chess game")
                .samples(ggez::conf::NumSamples::Four),
        )
        .window_mode(WindowMode::default().dimensions(
            TILE_SIZE * 8.0 + GRID_X * 2.0,
            TILE_SIZE * 8.0 + GRID_Y * 2.0,
        ))
        .add_resource_path("./resources")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = Chess::new(&mut ctx)?;

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct PieceImages {
    white_pawn: graphics::Image,
    white_rook: graphics::Image,
    white_bishop: graphics::Image,
    white_knight: graphics::Image,
    white_queen: graphics::Image,
    white_king: graphics::Image,

    black_pawn: graphics::Image,
    black_rook: graphics::Image,
    black_bishop: graphics::Image,
    black_knight: graphics::Image,
    black_queen: graphics::Image,
    black_king: graphics::Image,
}

impl PieceImages {
    pub fn new(ctx: &mut Context) -> GameResult<PieceImages> {
        Ok(PieceImages {
            white_pawn: graphics::Image::from_path(ctx, "/w_pawn_2x_ns.png")?,
            white_rook: graphics::Image::from_path(ctx, "/w_rook_2x_ns.png")?,
            white_bishop: graphics::Image::from_path(ctx, "/w_bishop_2x_ns.png")?,
            white_knight: graphics::Image::from_path(ctx, "/w_knight_2x_ns.png")?,
            white_queen: graphics::Image::from_path(ctx, "/w_queen_2x_ns.png")?,
            white_king: graphics::Image::from_path(ctx, "/w_king_2x_ns.png")?,

            black_pawn: graphics::Image::from_path(ctx, "/b_pawn_2x_ns.png")?,
            black_rook: graphics::Image::from_path(ctx, "/b_rook_2x_ns.png")?,
            black_bishop: graphics::Image::from_path(ctx, "/b_bishop_2x_ns.png")?,
            black_knight: graphics::Image::from_path(ctx, "/b_knight_2x_ns.png")?,
            black_queen: graphics::Image::from_path(ctx, "/b_queen_2x_ns.png")?,
            black_king: graphics::Image::from_path(ctx, "/b_king_2x_ns.png")?,
        })
    }
}

struct Drawables {
    selected_frame: graphics::Mesh,
    possible_move_dot: graphics::Mesh,
}

impl Drawables {
    pub fn new(ctx: &mut Context) -> GameResult<Drawables> {
        let mb = &mut graphics::MeshBuilder::new();
        mb.rectangle(
            graphics::DrawMode::stroke(5.0),
            Rect::new(0.0, 0.0, TILE_SIZE, TILE_SIZE),
            graphics::Color::RED,
        )?;
        let frame = graphics::Mesh::from_data(ctx, mb.build());
        let mb = &mut graphics::MeshBuilder::new();
        mb.circle(
            graphics::DrawMode::fill(),
            vec2(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
            TILE_SIZE / 8.0,
            0.1,
            graphics::Color::RED,
        )?;
        let dot = graphics::Mesh::from_data(ctx, mb.build());
        Ok(Drawables {
            selected_frame: frame,
            possible_move_dot: dot,
        })
    }
}

struct Selected {
    piece: Piece,
    position: GridPosition,
    moves: Vec<Move>,
}

pub struct Chess {
    game: Game,
    grid: Grid,
    piece_images: PieceImages,
    selected_piece: Option<Selected>,
    drawables: Drawables,
}

impl Chess {
    pub fn new(ctx: &mut Context) -> GameResult<Chess> {
        Ok(Chess {
            // ...
            game: Game::new(Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".into())),
            grid: Grid::new(ctx)?,
            piece_images: PieceImages::new(ctx)?,
            selected_piece: None,
            drawables: Drawables::new(ctx)?,
        })
    }
}

pub fn board2grid(x: usize, y: usize) -> Vec2 {
    vec2(GRID_X + x as f32 * TILE_SIZE, GRID_Y + y as f32 * TILE_SIZE)
}

pub fn draw_piece(
    chess: &mut Chess,
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    piece_type: PieceType,
    color: Color,
) {
    let image = match piece_type {
        PieceType::PAWN => {
            if color == Color::WHITE {
                &chess.piece_images.white_pawn
            } else {
                &chess.piece_images.black_pawn
            }
        }
        PieceType::ROOK => {
            if color == Color::WHITE {
                &chess.piece_images.white_rook
            } else {
                &chess.piece_images.black_rook
            }
        }
        PieceType::BISHOP => {
            if color == Color::WHITE {
                &chess.piece_images.white_bishop
            } else {
                &chess.piece_images.black_bishop
            }
        }
        PieceType::KNIGHT => {
            if color == Color::WHITE {
                &chess.piece_images.white_knight
            } else {
                &chess.piece_images.black_knight
            }
        }
        PieceType::QUEEN => {
            if color == Color::WHITE {
                &chess.piece_images.white_queen
            } else {
                &chess.piece_images.black_queen
            }
        }
        PieceType::KING => {
            if color == Color::WHITE {
                &chess.piece_images.white_king
            } else {
                &chess.piece_images.black_king
            }
        }
        PieceType::EMPTY => panic!("Bruh"),
    };

    let scale = Vec2::new(
        TILE_SIZE / image.height() as f32, // Scale width the same as height
        TILE_SIZE / image.height() as f32,
    );
    let position = board2grid(x, y);
    canvas.draw(
        image,
        DrawParam::new()
            .dest(position + TILE_SIZE * 0.5)
            .scale(scale * 0.8)
            .offset(vec2(0.5, 0.5)),
    );
}

impl EventHandler for Chess {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        canvas.set_sampler(graphics::Sampler::linear_clamp());
        canvas.draw(&self.grid.mesh, vec2(GRID_X, GRID_Y));

        for y in 0..8 {
            for x in 0..8 {
                let piece = &self.game.board.pieces[y][x];
                if piece.color == Color::EMPTY {
                    continue;
                }
                draw_piece(self, &mut canvas, x, y, piece.piece_type, piece.color);
            }
        }

        if let Some(selected) = &self.selected_piece {
            let pos = board2grid(selected.position.x, selected.position.y);
            canvas.draw(&self.drawables.selected_frame, pos);

            for grid_pos in &selected.moves {
                let pos = board2grid(grid_pos.0 as usize, grid_pos.1 as usize);
                canvas.draw(&self.drawables.possible_move_dot, pos);
            }
        }

        // Draw code here...
        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        if button != MouseButton::Left {
            return Ok(());
        }
        let position = match self.grid.screen2grid(x, y) {
            Some(t) => t,
            None => return Ok(()),
        };

        if let Some(selected) = &self.selected_piece {
            // Piece already selected, maybe move? :)
            if selected
                .moves
                .contains(&Move(position.x as i32, position.y as i32))
            {
                move_piece(
                    Move(position.x as i32, position.y as i32),
                    selected.position.x as i32,
                    selected.position.y as i32,
                    &mut self.game,
                )
                .unwrap();
                self.selected_piece = None;
            }
        }

        let piece = self.game.board.pieces[position.y][position.x];
        if piece.piece_type != PieceType::EMPTY && piece.color == self.game.turn {
            let moves = get_legal_moves(
                self.game.board,
                position.x as i32,
                position.y as i32,
                piece.color,
            );
            self.selected_piece = Some(Selected {
                piece,
                position,
                moves,
            });
        } else {
            self.selected_piece = None;
        }
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        //self.selected_piece = None;
        Ok(())
    }
}
