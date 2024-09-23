use crate::{BLACK_COLOR, TILE_SIZE, WHITE_COLOR};
use ggez::graphics::Rect;
use ggez::{graphics, Context, GameResult};

pub struct Grid {
    pub mesh: graphics::Mesh,
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
}
