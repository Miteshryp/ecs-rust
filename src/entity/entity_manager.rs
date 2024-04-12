use std::{any::TypeId};
use hashbrown::HashMap;

use crate::{
    component::{component_manager::ComponentManager, Component},
    world::World,
};
use super::Entity;

/// EntityManager is a struct which is responsible for managing
/// entity related operations such as:
///     1. Managing components belonging to an entity.
///     2. Providing APIs to access components in a specific entity.
///     3. Implementing an event emission system to enable events.
///
///

pub(crate) struct EntityManager {
    /// Store for Entities
    entities: Vec<Entity>,

    // Empty indexes
    empty_index: Vec<usize>,
    
}

impl EntityManager {
    fn generate_entity_id(&mut self) -> Entity {

        // If we have a whole, we fill it, else we create a new position
        if !self.empty_index.is_empty() {
            let index = self.empty_index.pop().unwrap();
            
            Entity {
                index: index as u32,
                generation: self.entities[index].generation + 1,
            }
        } else {
            Entity {
                index: self.entities.len() as u32,
                generation: 0,
            }
        }
    }
}

/// Public member implementations
impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            empty_index: vec![],
        }
    }


    /// 
    /// Returns the vector of all active entities living 
    /// in the world entity manager.
    /// 
    pub fn get_active_entities(&self) -> hashbrown::HashSet<&Entity> {
        let valid_entities: hashbrown::HashSet<&Entity> = self.entities.iter().filter(|e| {
            !self.empty_index.contains(&(e.index as usize))
        }).collect();

        valid_entities
    }

    ///
    /// Creates an Entity and gives it's EntityId
    ///
    /// Returns an EntityId, which must be used to perform
    /// all further operations on the entity
    ///
    pub fn create_entity(&mut self) -> Entity {
        let entity_id = self.generate_entity_id();
        let entity_index = entity_id.index;

        if entity_index == self.entities.len() as u32 {
            self.entities.push(entity_id);
        } else {
            self.entities[entity_index as usize] = entity_id;
        }

        entity_id
    }

    ///
    /// Removes the entity and all components attached to it
    ///
    /// The [`entity_id`](EntityId) passed in the parameter is
    /// invalidated and any future operations on the entity
    /// will result in a panic
    ///
    pub fn dispose_entity_id(&mut self, entity_id: Entity) {
        assert!(self.entities[entity_id.index as usize].generation == entity_id.generation, 
            "Failed to dispose entity id {:?}: ID does not exist in the system anymore. It might have been deleted previously", entity_id); 
        self.empty_index.push(entity_id.index as usize);
    }
}
