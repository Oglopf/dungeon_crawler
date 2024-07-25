use bracket_lib::prelude::*;
use crate::map::*;

pub struct State {
    pub map: Map,
}

impl State {
    pub fn new() -> Self {
        State {
            map: Map::new(),
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
        self.map.render(ctx);
    }
}