use crate::map::*;
use crate::prelude::*;

const NUMS_ROOMS: usize = 20;

// Build map with map, rooms, and origin 
// i.e. where player begins.
pub struct MapBuilder {
    pub map: Map,
    // Rect comes from bracker-lib.
    // helper function for calcs w/ rectangles.
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn fill(&mut self, tile: TileType) {
        self.map
            .tiles
            .iter_mut() // gives a mutable iterator. 
            .for_each( |t| *t = tile); // deref t to write to the reference of 
    }

    pub fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() > NUMS_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10), 
                rng.range(1, SCREEN_WIDTH - 10), 
                rng.range(2, 10), 
                rng.range(2, 10)
            );

            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room)
            }
        }
    }
}