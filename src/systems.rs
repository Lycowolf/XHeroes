use legion::{system, Entity, Query, EntityStore};
use legion::systems::CommandBuffer;
use crate::components::{Actor, TurnEligible, Health};
use legion::world::SubWorld;
use crate::resources::{RunState, CurrentActors, ActionQueue};
use crate::actions::Action;
use log::{debug};

/// When everyone's turn ended, marks everyone as eligible to play to start the next turn
#[system]
pub(crate) fn next_turn(
    cmd: &mut CommandBuffer,
    world: &SubWorld,
    query: &mut Query<(Entity, &Actor, Option<&TurnEligible>)>,
) {
    if query.iter(world).filter(|(_, _, eligible)| eligible.is_some()).count() > 0 {
        return;
    }
    query.for_each(world, |(entity, _, _)| {
        cmd.add_component(*entity, TurnEligible);
    });
}

// TODO: add PlayerControlled component and set run_state properly
/// Computes a list of Actors who's turn is now by selecting the set of actors (more than one!)
/// with maximum initiative.
#[system]
pub(crate) fn turn_assignment(
    world: &SubWorld,
    query: &mut Query<(Entity, &Actor, Option<&TurnEligible>)>,
    #[resource] run_state: &mut RunState,
    #[resource] current_actors: &mut CurrentActors,
) {
    current_actors.0.clear();

    // Having no-one eligible is normal: it happens on init
    let fastest_initiative = query
        .iter(world)
        .filter_map(|(_entity, actor, eligible)| {
            if eligible.is_some() {
                Some(actor.initiative)
            } else {
                None
            }
        })
        .max();

    if let Some(fastest) = fastest_initiative {
        query.for_each(world, |(entity, actor, eligible)| {
            if eligible.is_some() {
                if actor.initiative == fastest {
                    current_actors.0.push(*entity)
                }
            }
        })
    }
}

/// Walks action queue and resolves all actions
#[system]
#[write_component(Health)]
pub(crate) fn action_resolution(world: &mut SubWorld, #[resource] action_queue: &mut ActionQueue) {
    for action in action_queue.0.drain(..) {
        debug!("Resolving action: {:?}", action);
        match action {
            Action::Attack { origin: _, target, damage } => {
                let mut health_entry = world.entry_mut(target)
                    .expect(format!("{:?} is targeting nonexistent entity", action).as_str());
                let mut health = health_entry
                    .get_component_mut::<Health>()
                    .expect("Target of {:?} doesn't have Health component");
                health.0 -= damage;
            }
        }
    }
}

/// Queues removal of all entities with zero or negative health. Don't forget to flush command buffers afterwards!
#[system(for_each)]
pub(crate) fn death(cmd: &mut CommandBuffer, entity: &Entity, health: &Health) {
    if health.0 <= 0 {
        cmd.remove(*entity)
    }
}