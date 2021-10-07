use legion::Entity;
use std::collections::VecDeque;
use crate::actions::Action;
use strum::{EnumIter};

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, EnumIter)]
#[allow(unused)]
pub(crate) enum RunState {
    GameStart,
    WaitingForInput,
    Run,
}

/// List of actors who can act at at this moment.
/// Probably shouldn't contain PCs and NPCs at the same moment?
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct CurrentActors(pub Vec<Entity>);

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct ActionQueue(pub VecDeque<Action>);