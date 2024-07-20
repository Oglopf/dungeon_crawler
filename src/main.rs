#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

struct State {
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            score: 0,
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
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

fn main() -> BError {
    println!("Hello, world!");

    let context = BTermBuilder::simple80x50()
                                .with_title("Dungeon Crawl")
                                .build()?;

    main_loop(context, State::new())
}
