use bracket_lib::prelude::{BTerm, VirtualKeyCode};
use legion::*;
use crate::components::{Name, Health, TurnEligible, Actor};
use crate::resources::{CurrentActors, ActionQueue};
use crate::actions::Action;

pub(crate) fn draw_world(ctx: &mut BTerm, world: &World, resources: &Resources) {
    let mut ui_offset = 0;
    ctx.cls();


    let mut actors: Vec<_> = <(&Name, &Health, &Actor)>::query().iter(world).collect();
    actors.sort_by_key(|(name, _, _)| name.0.clone());
    for (name, health, actor) in actors.iter() {
        ctx.print(0, ui_offset, format!("{}: {} -> {}", name.0, health.0, actor.initiative));
        ui_offset += 1;
    }

    ui_offset += 1;
    ctx.print(0, ui_offset, "Current actor(s):");
    ui_offset += 1;
    ctx.print(0, ui_offset, "=================");
    ui_offset += 1;

    let current_actors = resources.get::<CurrentActors>().unwrap();
    for actor_entity in &current_actors.0 {
        let name = world.entry_ref(*actor_entity).unwrap().get_component::<Name>().unwrap().clone();
        ctx.print(0, ui_offset, format!("  {}", name.0));
        ui_offset += 1;
    };
}

pub(crate) fn player_input(ctx: &mut BTerm, world: &mut World, resources: &mut Resources) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Space => {
                let current_actors = resources.get::<CurrentActors>().unwrap();
                if let Some(me) = current_actors.0.first() {
                    // find suitable target
                    let my_team = world.entry(*me).unwrap().get_component::<Actor>().unwrap().team;
                    if let Some(enemy) = <(Entity, &Actor)>::query()
                        .iter(world)
                        .filter(|(_entity, actor)| actor.team != my_team)
                        .map(|(entity, _actor)| entity)
                        .next()
                    {
                        resources.get_mut::<ActionQueue>().unwrap().0.push_back(Action::Attack {
                            origin: *me,
                            target: *enemy,
                            damage: 15
                        })
                    }
                    // end his turn
                    world.entry(*me).unwrap().remove_component::<TurnEligible>()
                }
            }
            _ => {}
        }
    }
}