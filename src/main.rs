#![allow(non_snake_case)]

use bracket_lib::prelude::*;
use bracket_terminal::prelude::*;
use env_logger;

mod ui;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::Space => {}
                _ => {}
            }
        }

        ui::draw_battle(ctx);
    }
}

fn main() -> BError {
    env_logger::init();

    // Display
    let context = BTermBuilder::vga(80, 25)
        .with_title("XHeroes")
        .with_font("vga8x16.png", 8, 16)// WORKAROUND: https://github.com/amethyst/bracket-lib/issues/231
        .build()?;

    let gs = State {};

    main_loop(context, gs)
}