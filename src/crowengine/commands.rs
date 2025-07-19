use super::bundle::Bundle;
use super::components::components::Component;
use super::entity::EntityId;
use super::world::World;

pub struct Commands<'a> {
    pub next_id: u32,
    world: &'a mut World,
}

impl<'a> Commands<'a> {
    pub fn new(world: &'a mut World) -> Self {
        Commands { next_id: 0, world }
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityId {
        let entity = self.create_entity();
        bundle.insert_all(self, entity);
        entity
    }

    pub fn insert<T: Component>(&mut self, entity: EntityId, component: T) {
        self.world.insert(entity, component);
    }

    pub fn create_entity(&mut self) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;
        id
    }
}
