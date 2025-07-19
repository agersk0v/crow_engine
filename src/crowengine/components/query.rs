use super::super::entity::EntityId;
use super::components::Component;
use super::components::ComponentStorage;
use std::collections::HashMap;

pub trait FetchComponent<'a> {
    type Component: Component;
    type RefType;

    fn fetch_from_component_storage(
        map: &'a HashMap<EntityId, Self::Component>,
        entity: &EntityId,
    ) -> Option<Self::RefType>;
}

impl<'a, T: Component> FetchComponent<'a> for &'a T {
    type Component = T;
    type RefType = &'a T;

    fn fetch_from_component_storage(
        map: &'a HashMap<EntityId, T>,
        entity: &EntityId,
    ) -> Option<Self::RefType> {
        map.get(entity)
    }
}

impl<'a, T: Component> FetchComponent<'a> for Option<&'a T> {
    type Component = T;
    type RefType = Option<&'a T>;

    fn fetch_from_component_storage(
        map: &'a HashMap<EntityId, T>,
        entity: &EntityId,
    ) -> Option<Self::RefType> {
        Some(map.get(entity))
    }
}

pub trait Query<'a> {
    type Item;

    #[allow(dead_code)]
    fn fetch(storage: &'a ComponentStorage) -> Box<dyn Iterator<Item = Self::Item> + 'a>;
}

impl<'a, A> Query<'a> for (A,)
where
    A: FetchComponent<'a> + 'a,
{
    type Item = (A::RefType,);

    fn fetch(storage: &'a ComponentStorage) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        let map = storage.get_all::<A::Component>();

        match map {
            Some(map) => {
                let iter = map
                    .iter()
                    .filter_map(move |(id, _)| Some((A::fetch_from_component_storage(map, id)?,)));
                Box::new(iter)
            }
            None => Box::new(std::iter::empty()),
        }
    }
}

impl<'a, A, B> Query<'a> for (A, B)
where
    A: FetchComponent<'a> + 'a,
    B: FetchComponent<'a> + 'a,
{
    type Item = (A::RefType, B::RefType);

    fn fetch(storage: &'a ComponentStorage) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        let a_map = storage.get_all::<A::Component>();
        let b_map = storage.get_all::<B::Component>();

        match (a_map, b_map) {
            (Some(a), Some(b)) => {
                let iter = a.iter().filter_map(move |(id, _)| {
                    Some((
                        A::fetch_from_component_storage(a, id)?,
                        B::fetch_from_component_storage(b, id)?,
                    ))
                });
                Box::new(iter)
            }
            _ => Box::new(std::iter::empty()),
        }
    }
}

impl<'a, A, B, C> Query<'a> for (A, B, C)
where
    A: FetchComponent<'a> + 'a,
    B: FetchComponent<'a> + 'a,
    C: FetchComponent<'a> + 'a,
{
    type Item = (A::RefType, B::RefType, C::RefType);

    fn fetch(storage: &'a ComponentStorage) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        let a_map = storage.get_all::<A::Component>();
        let b_map = storage.get_all::<B::Component>();
        let c_map = storage.get_all::<C::Component>();

        match (a_map, b_map, c_map) {
            (Some(a), Some(b), Some(c)) => {
                let iter = a.iter().filter_map(move |(id, _)| {
                    Some((
                        A::fetch_from_component_storage(a, id)?,
                        B::fetch_from_component_storage(b, id)?,
                        C::fetch_from_component_storage(c, id)?,
                    ))
                });
                Box::new(iter)
            }
            _ => Box::new(std::iter::empty()),
        }
    }
}

impl<'a, A, B, C, D> Query<'a> for (A, B, C, D)
where
    A: FetchComponent<'a> + 'a,
    B: FetchComponent<'a> + 'a,
    C: FetchComponent<'a> + 'a,
    D: FetchComponent<'a> + 'a,
{
    type Item = (A::RefType, B::RefType, C::RefType, D::RefType);

    fn fetch(storage: &'a ComponentStorage) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        let a_map = storage.get_all::<A::Component>();
        let b_map = storage.get_all::<B::Component>();
        let c_map = storage.get_all::<C::Component>();
        let d_map = storage.get_all::<D::Component>();

        match (a_map, b_map, c_map, d_map) {
            (Some(a), Some(b), Some(c), Some(d)) => {
                let iter = a.iter().filter_map(move |(id, _)| {
                    Some((
                        A::fetch_from_component_storage(a, id)?,
                        B::fetch_from_component_storage(b, id)?,
                        C::fetch_from_component_storage(c, id)?,
                        D::fetch_from_component_storage(d, id)?,
                    ))
                });
                Box::new(iter)
            }
            _ => Box::new(std::iter::empty()),
        }
    }
}

impl<'a, A, B, C, D, E> Query<'a> for (A, B, C, D, E)
where
    A: FetchComponent<'a> + 'a,
    B: FetchComponent<'a> + 'a,
    C: FetchComponent<'a> + 'a,
    D: FetchComponent<'a> + 'a,
    E: FetchComponent<'a> + 'a,
{
    type Item = (A::RefType, B::RefType, C::RefType, D::RefType, E::RefType);

    fn fetch(storage: &'a ComponentStorage) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        let a_map = storage.get_all::<A::Component>();
        let b_map = storage.get_all::<B::Component>();
        let c_map = storage.get_all::<C::Component>();
        let d_map = storage.get_all::<D::Component>();
        let e_map = storage.get_all::<E::Component>();

        match (a_map, b_map, c_map, d_map, e_map) {
            (Some(a), Some(b), Some(c), Some(d), Some(e)) => {
                let iter = a.iter().filter_map(move |(id, _)| {
                    Some((
                        A::fetch_from_component_storage(a, id)?,
                        B::fetch_from_component_storage(b, id)?,
                        C::fetch_from_component_storage(c, id)?,
                        D::fetch_from_component_storage(d, id)?,
                        E::fetch_from_component_storage(e, id)?,
                    ))
                });
                Box::new(iter)
            }
            _ => Box::new(std::iter::empty()),
        }
    }
}
