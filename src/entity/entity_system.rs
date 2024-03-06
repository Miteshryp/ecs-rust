use std::{any::TypeId, collections::HashMap};

use crate::{
    component::{component_system::ComponentSystem, Component, ComponentHandler},
    entity::Entity,
    system::System,
};

use super::EntityId;

/// EntityManager is a struct which is responsible for managing
/// entity related operations such as:
///     1. Managing components belonging to an entity.
///     2. Providing APIs to access components in a specific entity.
///     3. Implementing an event emission system to enable events.
///  
/// @TODO: Think if it is a better design to store the component systems
///     inside the entity manager itself, since a single entity manager is
///     going to be a singleton in a World struct.
///
///

pub struct EntityManager {
    /// Store for Entities
    entities: Vec<Entity>,

    /// Component systems based on component types
    component_systems: HashMap<TypeId, Box<dyn System>>,

    /// Lookup table for entities
    entity_id_map: HashMap<EntityId, usize>,
}

/// Private member implementations
impl EntityManager {
    /// Checks if the component has been registered
    ///
    /// This validation must be performed before performing operations
    /// on a component
    fn check_component_registered<C: Component + 'static>(&self) -> bool {
        self.component_systems.contains_key(&TypeId::of::<C>())
    }

    fn get_component_type_id<C: Component + 'static>() -> TypeId {
        TypeId::of::<C>()
    }

    fn get_system_mut<C: Component + Sized + 'static>(&mut self) -> &mut ComponentSystem<C> {
        let system = self
            .component_systems
            .get_mut(&Self::get_component_type_id::<C>())
            .unwrap();
        let system = system
            .as_any_mut()
            .downcast_mut::<ComponentSystem<C>>()
            .unwrap();

        return system;
    }

    fn get_system_ref<C: Component + Sized + 'static>(&self) -> &ComponentSystem<C> {
        let system = self
            .component_systems
            .get(&Self::get_component_type_id::<C>())
            .unwrap();
        let system = system
            .as_any()
            .downcast_ref::<ComponentSystem<C>>()
            .unwrap();

        return system;
    }
}

/// Public member implementations
impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            component_systems: HashMap::new(),
            entity_id_map: HashMap::new(),
        }
    }

    /// Creates an entity in contiguous block
    pub fn create_entity(&mut self) -> EntityId {
        todo!()
    }

    /// Removes the entity and all components attached to it
    pub fn remove_entity(&mut self, entity_id: EntityId) {
        todo!()
    }

    /// Creates a component systems for handling the component
    pub fn register_component<C: Component + 'static, Handler: ComponentHandler<C> + 'static>(
        &mut self,
    ) {
        if self.check_component_registered::<C>() {
            println!("Component already registered: {}", C::get_name());
            return;
        }

        // Creating system for the component type
        self.component_systems.insert(
            Self::get_component_type_id::<C>(),
            Box::new(ComponentSystem::<C>::new::<Handler>()),
        );
    }

    ///
    /// Adds the given component type into the system
    ///
    /// WARNING: Calling this function with a component which
    ///     is not registered will result in a panic
    ///
    pub fn add_component_to_entity<C: Component + Sized + 'static>(
        &mut self,
        entity_id: EntityId,
        component: C,
    ) {
        assert!(
            self.check_component_registered::<C>(),
            "Component not registered for use: {}",
            C::get_name()
        );

        let system = self.get_system_mut::<C>();
        system.add_component_to_entity(entity_id, component);
    }

    ///
    /// Removes the given component type from the entity
    ///
    /// WARNING: Calling this function with a component which is not
    ///     registered or not a part of the given entity_id will result
    ///     in a panic
    ///
    ///
    pub fn remove_component_from_entity<C: Component + Sized + 'static>(
        &mut self,
        entity_id: EntityId,
    ) {
        let system = self.get_system_mut::<C>();
        system.remove_component_from_entity(entity_id);
    }

    ///
    /// Returns true if the given component type is attached to 
    /// the entity, false otherwise
    /// 
    /// WARNING: Calling this function with a component which is not registered
    ///     in the system will result in a panic
    ///
    pub fn has_component<C: Component + Sized + 'static>(&self, entity_id: EntityId) -> bool {
        if !self.check_component_registered::<C>() {
            panic!("Component not registered for use {}", C::get_name())
        }

        // Getting the appropriate system for the component
        let system = self.get_system_ref::<C>();

        // Querying for component presence
        system.has_component(entity_id)
    }

    /// 
    /// Used to get a mutable reference of the component attached to the
    /// specified entity.
    /// 
    pub fn get_component_mut_ref<C: Component + 'static>(&mut self, entity_id: EntityId) -> &mut C {
        // @TODO: Change into assert
        if !self.check_component_registered::<C>() {
            panic!("Component not registered for use {}", C::get_name())
        }

        let system = self.get_system_mut::<C>();
        system.borrow_component_mut(entity_id)
    }
}
