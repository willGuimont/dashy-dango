use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::collections::hash_set::Iter;

use crate::abort::Abort;

pub trait BaseComponent {
    fn as_any(&self) -> &dyn Any;
}

pub type Entity = u16;
type ComponentStore = HashMap<Entity, Box<dyn BaseComponent>>;
type ComponentHash = TypeId;
type ComponentsMap = HashMap<ComponentHash, ComponentStore>;

pub struct Registry {
    last_entity_id: Entity,
    alive_entities: HashSet<Entity>,
    // to_destroy_entity: HashSet<Entity>,
    components: ComponentsMap,
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            last_entity_id: 0,
            alive_entities: HashSet::new(),
            components: HashMap::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let new_id = self.last_entity_id;
        self.alive_entities.insert(new_id);
        self.last_entity_id += 1;
        new_id
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.alive_entities.contains(&entity)
    }

    pub fn all_entities(&self) -> Iter<Entity> {
        self.alive_entities.iter()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.alive_entities.remove(&entity);
        self.components.iter_mut().for_each(|(_, cs)| {
            cs.remove(&entity);
        });
    }

    pub fn add_component<T: BaseComponent + 'static + Clone>(&mut self, entity: Entity, component: T) -> Option<()> {
        let type_id = TypeId::of::<T>();
        self.components.entry(type_id).or_insert_with(ComponentStore::new);
        self.components.get_mut(&type_id)
            .map(|c| c.insert(entity, Box::new(component)))
            .map(|_| ())
    }

    pub fn remove_component<T: BaseComponent + 'static + Clone>(&mut self, entity: Entity) {
        let type_id = TypeId::of::<T>();
        self.components.get_mut(&type_id)
            .map(|c| c.remove(&entity));
    }

    pub fn has_component<T: BaseComponent + 'static + Clone>(&self, entity: Entity) -> bool {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)
            .map(|cs| cs.contains_key(&entity))
            .unwrap_or_default()
    }

    pub fn get_component<T: BaseComponent + 'static + Clone>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if !self.has_component::<T>(entity) {
            return None;
        }
        self.components
            .get(&type_id)
            .abort()
            .get(&entity)
            .abort()
            .as_any()
            .downcast_ref::<T>()
    }
}

#[macro_export]
macro_rules! has_all_components {
    ($self:expr, $entity:expr) => (true);
    ($self:expr, $entity:expr, $($component:path),*) => ({
        let result = true;
        $(
            let result = result && $self.has_component::<$component>($entity);
        )*
        result
    });
}

#[macro_export]
macro_rules! get_components {
    ($self:expr, $entity:expr) => (());
    ($self:expr, $entity:expr, $($component:path),*) => (
        (
            $($self.get_component::<$component>($entity),)*
        )
    );
}

#[macro_export]
macro_rules! get_components_clone {
    ($self:expr, $entity:expr) => (());
    ($self:expr, $entity:expr, $($component:path),*) => (
        (
            $($self.get_component::<$component>($entity).cloned(),)*
        )
    );
}

#[macro_export]
macro_rules! get_components_unwrap {
    ($self:expr, $entity:expr) => (());
    ($self:expr, $entity:expr, $($component:path),*) => (
        (
            $($self.get_component::<$component>($entity).abort(),)*
        )
    );
}

#[macro_export]
macro_rules! get_components_clone_unwrap {
    ($self:expr, $entity:expr) => (());
    ($self:expr, $entity:expr, $($component:path),*) => (
        (
            $($self.get_component::<$component>($entity).cloned().abort(),)*
        )
    );
}

#[macro_export]
macro_rules! entities_with {
    ($self:expr, $($component:path),*) => ({
        let entities: Vec<Entity> = ($self.all_entities().filter(|e| has_all_components!($self, **e, $($component),*)).cloned().collect());
        entities
    })
}

#[macro_export]
macro_rules! entities_with_components {
    ($self:expr) => (());
    ($self:expr, $($component:path),*) => ({
        entities_with!($self, $($component),*)
            .iter()
            .map(|e| {
                (e, get_components_unwrap!($self, *e, $($component),*))
            })
    });
}

#[macro_export]
macro_rules! add_components {
    ($self:expr, $entity:expr) => ();
    ($self:expr, $entity:expr, $($component:path),*) => ({
        let result = true;
        $(
            $self.add_component($entity, $component).abort();
        )*
        result
    });
}
