use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::collections::hash_set::Iter;

use crate::trace;

pub trait BaseComponent {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub type Entity = u16;
type ComponentStore = HashMap<Entity, Box<dyn BaseComponent>>;
type ComponentHash = TypeId;
type ComponentsMap = HashMap<ComponentHash, ComponentStore>;

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
        to_delete.iter().for_each(|e| self.destroy_entity(e.clone()));
    }

    // TODO error on adding component to non-valid entity?
    pub fn add_component<T: BaseComponent + 'static>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        if !self.components.contains_key(&type_id) {
            self.components.insert(type_id, ComponentStore::new());
        }
        self.components.get_mut(&type_id)
            .map(|c| c.insert(entity, Box::new(component)));
    }

    // TODO error on removing component to non-valid entity?
    pub fn remove_component<T: BaseComponent + 'static>(&mut self, entity: Entity) {
        let type_id = TypeId::of::<T>();
        self.components.get_mut(&type_id)
            .map(|c| c.remove(&entity));
    }

    pub fn has_component<T: BaseComponent + 'static>(&self, entity: Entity) -> bool {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id).map(|cs| cs.contains_key(&entity)).unwrap_or_default()
    }

    pub fn get_component<T: BaseComponent + 'static>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if !self.components.contains_key(&type_id) {
            return None;
        } else if !self.is_valid(entity) {
            return None;
        } else if !self.has_component::<T>(entity) {
            return None;
        }
        let cs = self.components.get(&type_id).unwrap();
        let c  = cs.get(&entity).unwrap();
        c.as_any().downcast_ref::<T>()
        // c.as_any().downcast_ref::<T>()
    }
}