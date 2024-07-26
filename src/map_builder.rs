use crate::map::*;
use crate::prelude::*;

const NUMS_ROOMS: usize = 20;

// Build map with map, rooms, and origin 
// i.e. where player begins.
pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}