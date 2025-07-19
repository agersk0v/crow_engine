use super::super::entity::EntityId;
use crate::crowengine::components::use_shader::UseShader;

use std::any::{Any, TypeId};
use std::collections::HashMap;

use std::iter;

pub trait Component: Any {}
impl<T: Any> Component for T {}

pub struct ComponentStorage {
    pub storages: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentStorage {
    pub fn new() -> Self {
        let mut storages: HashMap<TypeId, Box<dyn Any>> = HashMap::new();
        storages.insert(
            TypeId::of::<UseShader>(),
            Box::new(HashMap::<EntityId, UseShader>::new()),
        );
        ComponentStorage { storages }
    }

    pub fn insert<T: Component>(&mut self, entity: EntityId, component: T) {
        let type_id = TypeId::of::<T>();

        let storage = self
            .storages
            .entry(type_id)
            .or_insert_with(|| Box::new(HashMap::<EntityId, T>::new()))
            .downcast_mut::<HashMap<EntityId, T>>()
            .unwrap();

        storage.insert(entity, component);
    }

    #[allow(dead_code)]
    pub fn get<T: Component>(&self, entity: &EntityId) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.storages
            .get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<HashMap<EntityId, T>>())
            .and_then(|map| map.get(entity))
    }

    #[allow(dead_code)]
    pub fn get_mut<T: Component>(&mut self, entity: &EntityId) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.storages
            .get_mut(&type_id)
            .and_then(|boxed| boxed.downcast_mut::<HashMap<EntityId, T>>())
            .and_then(|map| map.get_mut(entity))
    }

    pub fn query_mut<T: Component>(&mut self) -> Box<dyn Iterator<Item = (EntityId, &mut T)> + '_> {
        let type_id = TypeId::of::<T>();

        match self.storages.get_mut(&type_id) {
            Some(boxed_map) => {
                if let Some(typed_map) = boxed_map.downcast_mut::<HashMap<EntityId, T>>() {
                    Box::new(typed_map.iter_mut().map(|(id, comp)| (*id, comp)))
                } else {
                    Box::new(iter::empty())
                }
            }
            None => Box::new(iter::empty()),
        }
    }

    pub fn get_all<T: Component>(&self) -> Option<&HashMap<EntityId, T>> {
        let type_id = TypeId::of::<T>();
        self.storages
            .get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<HashMap<EntityId, T>>())
    }
}
