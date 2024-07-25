use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            // vec is tiled with floor tiles. Same floor 
            // tile over all vectors.
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    // The map must be able to draw itself.
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                // Get the unique integer each (x, y) maps to.
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }
}

// Add a public function to calculate a tile index.
pub fn map_idx(x: i32, y: i32) -> usize {
    // Using row-first encoding, so each (x, y) is converted into 
    // a vector at an index. The formula is what gives that index.
    // map_idx: (x, y) -> idx
    ((y * SCREEN_WIDTH) + x) as usize // convert result to usize
}

