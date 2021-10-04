// TODO: consider restricting component creation (so we can't have Actor without Name etc.)

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) struct Health(pub i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Actor {
    pub(crate) initiative: i32,
    pub(crate) team: Team,
}

impl Actor {
    pub(crate) fn new(name: String, health: i32, initiative: i32, team: Team) -> (Name, Health, Actor) {
        (Name(name), Health(health), Actor { initiative, team })
    }
}

/// Marks actors who are eligible to play (did not finish their turn yet)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TurnEligible;

// ================================================================
// These are not standalone components, do not use them as such
// ----------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Team {
    Heroes,
    EvilCorp
}