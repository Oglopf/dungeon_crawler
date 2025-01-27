use crate::{map::*, map_builder, player::Player, MapBuilder, SCREEN_HEIGHT, SCREEN_WIDTH};
use bracket_lib::prelude::*;

pub struct State {
    pub map: Map,
    pub player: Player,
}

impl State {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

// bracket-lit defines a Trait named GameState
// for games state structures. Requires object implement tick().
// Implement a trait similar to implementing a method
// on a struct. You implement the trait for the struct
// and define functions which the Trait expects, here tick().
impl GameState for State {
    // Takes a mutable instance of self and a mutable context of type BTerm.
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}
