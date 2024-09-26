use crate::{BLACK_COLOR, GRID_X, GRID_Y, TILE_SIZE, WHITE_COLOR};
use ggez::graphics::Rect;
use ggez::{graphics, Context, GameResult};

pub struct Grid {
    pub mesh: graphics::Mesh,
}

#[derive(Clone, Copy, PartialEq)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for GridPosition {
    fn from(val: (usize, usize)) -> Self {
        GridPosition { x: val.0, y: val.1 }
    }
}

impl Grid {
    pub fn new(ctx: &mut Context) -> GameResult<Grid> {
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

        Ok(Grid {
            mesh: graphics::Mesh::from_data(ctx, mb.build()),
        })
    }

    pub fn screen2grid(&self, x: f32, y: f32) -> Option<GridPosition> {
        if x - GRID_X < 0.0 || x - GRID_X > TILE_SIZE * 8.0 {
            return None;
        }
        if y - GRID_Y < 0.0 || y - GRID_Y > TILE_SIZE * 8.0 {
            return None;
        }

        let grid_x = ((x - GRID_X) / (TILE_SIZE)) as usize;
        let grid_y = ((y - GRID_Y) / (TILE_SIZE)) as usize;

        Some((grid_x, grid_y).into())
    }
}
