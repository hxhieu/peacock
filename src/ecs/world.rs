use crate::ecs::components::{Position, Velocity};
use specs::{Builder, World, WorldExt};

pub struct WorldContext {
    pub world: World,
}

impl WorldContext {
    pub fn new() -> Self {
        let mut world = World::new();
        // Register core components
        world.register::<Position>();
        world.register::<Velocity>();

        Self { world }
    }
}
