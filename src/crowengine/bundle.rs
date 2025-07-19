use super::commands::Commands;
use super::components::components::Component;
use super::entity::EntityId;

pub trait Bundle {
    fn insert_all(self, commands: &mut Commands, entity: EntityId);
}

impl<A: Component> Bundle for (A,) {
    fn insert_all(self, commands: &mut Commands, entity: EntityId) {
        let (a,) = self;
        commands.insert(entity, a);
    }
}

impl<A: Component, B: Component> Bundle for (A, B) {
    fn insert_all(self, commands: &mut Commands, entity: EntityId) {
        let (a, b) = self;
        commands.insert(entity, a);
        commands.insert(entity, b);
    }
}

impl<A: Component, B: Component, C: Component> Bundle for (A, B, C) {
    fn insert_all(self, commands: &mut Commands, entity: EntityId) {
        let (a, b, c) = self;
        commands.insert(entity, a);
        commands.insert(entity, b);
        commands.insert(entity, c);
    }
}

impl<A: Component, B: Component, C: Component, D: Component> Bundle for (A, B, C, D) {
    fn insert_all(self, commands: &mut Commands, entity: EntityId) {
        let (a, b, c, d) = self;
        commands.insert(entity, a);
        commands.insert(entity, b);
        commands.insert(entity, c);
        commands.insert(entity, d);
    }
}

impl<A: Component, B: Component, C: Component, D: Component, E: Component> Bundle
    for (A, B, C, D, E)
{
    fn insert_all(self, commands: &mut Commands, entity: EntityId) {
        let (a, b, c, d, e) = self;
        commands.insert(entity, a);
        commands.insert(entity, b);
        commands.insert(entity, c);
        commands.insert(entity, d);
        commands.insert(entity, e);
    }
}

impl<A: Component, B: Component, C: Component, D: Component, E: Component, F: Component> Bundle
    for (A, B, C, D, E, F)
{
    fn insert_all(self, commands: &mut Commands, entity: EntityId) {
        let (a, b, c, d, e, f) = self;
        commands.insert(entity, a);
        commands.insert(entity, b);
        commands.insert(entity, c);
        commands.insert(entity, d);
        commands.insert(entity, e);
        commands.insert(entity, f);
    }
}
