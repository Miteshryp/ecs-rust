use crate::{
    component::{
        component_system::ComponentSystem, Component, ComponentHandler
    }, entity::{entity_system::EntityManager, Entity, EntityId}, system::System
};

pub struct World {
    entity_manager: EntityManager
}

impl World {
    pub fn new() -> Self {
        Self { entity_manager: EntityManager::new() }
    }

    pub fn register_component_with_handler<C, Handler>(&mut self, handler: Handler)
    where
        C: Component + Sized + 'static,
        Handler: ComponentHandler<C>,
    {
        self.entity_manager.register_component::<C, Handler>();
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
}
