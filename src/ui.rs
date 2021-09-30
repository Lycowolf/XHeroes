use bracket_lib::prelude::BTerm;

pub(crate) fn draw_battle(ctx: &mut BTerm,) {
    ctx.cls();

    ctx.print(0, 0, "Hello world!")
}