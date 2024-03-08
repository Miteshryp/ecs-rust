use crate::{
    component::{
        component_manager::ComponentManager, Component, ComponentHandler
    }, entity::{entity_manager::EntityManager, Entity, EntityId}, system::EcsManager
};

pub struct World {
    active: bool,
    cleanup: bool,
    entity_manager: EntityManager,
}

impl World {
    pub fn new() -> Self {
        Self { 
            active: false,
            cleanup: false,
            entity_manager: EntityManager::new() 
        }
    }

    pub fn register_component<C>(&mut self)
    where
        C: Component + Sized + 'static,
    {
        self.entity_manager.register_component::<C>();
    }

    pub fn create_entity(&mut self) -> EntityId {
        self.entity_manager.create_entity()
    }

    pub fn remove_entity(&mut self, entity_id: EntityId) {
        self.entity_manager.remove_entity(entity_id);
    }

    pub fn add_component_to_entity<C: Component + Sized + 'static>(&mut self, entity_id: EntityId, component: C) {
        assert!(!self.entity_manager.has_component::<C>(entity_id), "Component Attachment failed: Component already exists in the entity with id {}", entity_id);
        self.entity_manager.add_component_to_entity(entity_id, component);
    }

    pub fn remove_component_from_entity<C: Component + Sized + 'static>(&mut self, entity_id: EntityId) {
        assert!(self.entity_manager.has_component::<C>(entity_id), "Component Removal failed: Component does not exist in the entity with id {}", entity_id);
        self.entity_manager.remove_component_from_entity::<C>(entity_id);
    }


    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    pub fn is_active(&self) -> bool {
        self.active
    }


    /// Collection functions
    pub fn get_all_entities(&self) -> Vec<EntityId> {
        todo!()
    }

    pub fn get_all_components<C: Component + 'static>(&self) -> Vec<(&C, &EntityId)> {
        self.entity_manager.get_all_components::<C>()
    }

    pub fn get_all_components_mut<C: Component + 'static>(&mut self) -> Vec<(&mut C, &EntityId)> {
        self.entity_manager.get_all_components_mut::<C>()
    }
}
