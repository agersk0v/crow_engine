use super::components::components::Component;
use super::components::components::ComponentStorage;
use super::components::query::Query;
use super::entity::EntityId;
use super::shaders::shader::Shader;
use std::collections::HashMap;

pub struct World {
    #[allow(dead_code)]
    pub resources: Resources,
    components: ComponentStorage,
}

pub struct Resources {
    pub shaders: HashMap<String, Shader>,
}

impl World {
    pub fn new(default_shader: Shader) -> Self {
        let mut shaders = HashMap::new();
        shaders.insert("default".to_string(), default_shader);

        Self {
            resources: Resources { shaders },
            components: ComponentStorage::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get<T: Component>(&self, entity: &EntityId) -> Option<&T> {
        self.components.get::<T>(entity)
    }

    pub fn insert<T: Component>(&mut self, entity: EntityId, component: T) {
        self.components.insert(entity, component);
    }

    pub fn query<'a, Q>(&'a self) -> impl Iterator<Item = Q::Item> + 'a
    where
        Q: Query<'a> + 'a,
    {
        Q::fetch(&self.components)
    }

    pub fn query_mut<T: Component>(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.components.query_mut::<T>()
    }
}
