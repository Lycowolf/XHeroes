use legion::Entity;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Action {
    Attack {
        origin: Entity,
        target: Entity,
        damage: i32
    }
}