use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::collections::hash_set::Iter;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::abort::Abort;

pub trait BaseComponent {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub type Entity = u16;
type ComponentStore = HashMap<Entity, Box<dyn BaseComponent>>;
type ComponentHash = TypeId;
type ComponentsMap = HashMap<ComponentHash, ComponentStore>;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
struct EntityNotFound(Entity);

impl Display for EntityNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "entity {} not found", self.0)
    }
}

impl Error for EntityNotFound {}

pub struct Registry {
    last_entity_id: Entity,
    valid_entities: HashSet<Entity>,
    to_destroy_entity: HashSet<Entity>,
    components: ComponentsMap,
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            last_entity_id: 0,
            valid_entities: HashSet::new(),
            to_destroy_entity: HashSet::new(),
            components: HashMap::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let new_id = self.last_entity_id;
        self.valid_entities.insert(new_id);
        self.last_entity_id += 1;
        new_id
    }

    pub fn is_valid(&self, entity: Entity) -> bool {
        self.valid_entities.contains(&entity)
    }

    pub fn all_entities(&self) -> Iter<Entity> {
        self.valid_entities.iter()
    }

    fn destroy_entity(&mut self, entity: Entity) {
        self.valid_entities.remove(&entity);
        self.to_destroy_entity.remove(&entity);
        self.components.iter_mut().for_each(|(_, cs)| {
            cs.remove(&entity);
        });
    }

    pub fn mark_to_destroy(&mut self, entity: Entity) {
        self.to_destroy_entity.insert(entity);
    }

    pub fn destroy_marked_entities(&mut self) {
        let to_delete = self.to_destroy_entity.clone();
        to_delete.iter().for_each(|e| self.destroy_entity(*e));
    }

    pub fn add_component<T: BaseComponent + 'static + Clone>(&mut self, entity: Entity, component: T) -> Result<()> {
        let type_id = TypeId::of::<T>();
        self.components.entry(type_id).or_insert_with(ComponentStore::new);
        self.components.get_mut(&type_id)
            .map(|c| c.insert(entity, Box::new(component)))
            .ok_or_else(|| EntityNotFound(entity).into())
            .map(|_| ())
    }

    pub fn remove_component<T: BaseComponent + 'static + Clone>(&mut self, entity: Entity) {
        let type_id = TypeId::of::<T>();
        self.components.get_mut(&type_id)
            .map(|c| c.remove(&entity));
    }

    pub fn has_component<T: BaseComponent + 'static + Clone>(&self, entity: Entity) -> bool {
        let type_id = TypeId::of::<T>();
        if !self.is_valid(entity) {
            return false;
        }
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

    pub fn get_component_clone<T: BaseComponent + 'static + Clone>(&self, entity: Entity) -> Option<T> {
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
            .cloned()
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
            $($self.get_component_clone::<$component>($entity),)*
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
            $($self.get_component_clone::<$component>($entity).abort(),)*
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
