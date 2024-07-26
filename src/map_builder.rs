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
}