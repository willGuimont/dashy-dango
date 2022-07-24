use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::collections::hash_set::Iter;

pub trait BaseComponent {}

pub type Entity = u16;
pub type ComponentStore = HashMap<Entity, Box<dyn BaseComponent>>;
pub type ComponentHash = TypeId;

pub struct Registry {
    last_entity_id: Entity,
    valid_entities: HashSet<Entity>,
    to_destroy_entity: HashSet<Entity>,
    components: HashMap<ComponentHash, ComponentStore>,
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

    pub fn add_component<T: BaseComponent + 'static>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        if !self.components.contains_key(&type_id) {
            self.components.insert(type_id, ComponentStore::new());
        }
        self.components.get_mut(&type_id).unwrap().insert(entity, Box::new(component));
    }

    pub fn remove_component<T: BaseComponent + 'static>(&mut self, entity: Entity) {
        let type_id = TypeId::of::<T>();
        if self.components.contains_key(&type_id) {
            self.components.remove(&type_id);
        }
    }
}
