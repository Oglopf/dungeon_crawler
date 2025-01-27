#![warn(clippy::all, clippy::pedantic)]

mod map;
mod map_builder;
mod player;
mod state;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use crate::state::*;
}

use prelude::*;

fn main() -> BError {
    println!("main() is running!");

    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawl")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
