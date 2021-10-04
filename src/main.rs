#![allow(non_snake_case)]

use bracket_lib::prelude::*;
use env_logger;
use legion::*;

mod ui;
mod components;
mod resources;
mod systems;
mod actions;

use components::*;
use crate::resources::{RunState, CurrentActors, ActionQueue};

struct State {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.schedule.execute(&mut self.world, &mut self.resources);
        ui::player_input(ctx, &mut self.world, &mut self.resources);
        ui::draw_world(ctx, &self.world, &self.resources);
    }
}

impl Default for State {
    fn default() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();
        init_ecs(&mut world, &mut resources);

        let schedule = Schedule::builder()
            .add_system(systems::action_resolution_system())
            .add_system(systems::death_system())
            .flush()
            // ---- turn starts -----
            .add_system(systems::next_turn_system())
            .add_system(systems::turn_assignment_system())
            .build();


        Self { world, resources, schedule }
    }
}

fn init_ecs(world: &mut World, resources: &mut Resources) {
    // Starting entities
    let actors = vec![
        Actor::new("Hero".into(), 100, 10, Team::Heroes),
        Actor::new("Villain".into(), 20, 5, Team::EvilCorp),
        Actor::new("Villain clone".into(), 15, 5, Team::EvilCorp),
        Actor::new("Mook".into(), 5, 1, Team::EvilCorp),
    ];
    world.extend(actors);

    // Resources
    resources.insert(RunState::GameStart);
    resources.insert(CurrentActors::default()); // this will be filled by the first run of the turn_assignment_system
    resources.insert(ActionQueue::default());
}

fn main() -> BError {
    env_logger::init();

    // Display
    let context = BTermBuilder::vga(80, 25)
        .with_title("XHeroes")
        .with_font("vga8x16.png", 8, 16)// WORKAROUND: https://github.com/amethyst/bracket-lib/issues/231
        .build()?;

    let gs = State::default();

    main_loop(context, gs)
}
