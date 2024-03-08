use std::{any::TypeId, collections::HashMap};

use crate::{
    component::{component_manager::ComponentManager, Component, ComponentHandler},
    entity::Entity,
    system::EcsManager,
    world::World,
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

pub(crate) struct EntityManager {
    /// Store for Entities
    entities: Vec<Entity>,

    /// Component systems based on component types
    component_systems: HashMap<TypeId, Box<dyn EcsManager>>,

    /// Lookup table for entities
    entity_id_map: HashMap<EntityId, usize>,
}

/// Private member implementations
impl EntityManager {
    ///
    /// Get the type id of component. Used to fetch the system
    /// of the component based on it's type id
    ///
    fn get_component_type_id<C: Component + 'static>() -> TypeId {
        TypeId::of::<C>()
    }

    fn generate_entity_id(&self) -> EntityId {
        todo!()
    }

    /// Checks if the component has been registered
    ///
    /// This validation must be performed before performing operations
    /// on a component
    fn check_component_registered<C: Component + 'static>(&self) -> bool {
        self.component_systems.contains_key(&TypeId::of::<C>())
    }

    ///
    /// Returns a mutable reference of a [`ComponentSystem`] object
    /// which is present in the [`EntityManager`] object
    ///
    /// WARNING: The component must be registered in the system, otherwise the
    /// function will result in a panic
    ///
    fn get_system_mut<C: Component + Sized + 'static>(&mut self) -> &mut ComponentManager<C> {
        let system = self
            .component_systems
            .get_mut(&Self::get_component_type_id::<C>())
            .unwrap();
        let system = system
            .as_any_mut()
            .downcast_mut::<ComponentManager<C>>()
            .unwrap();

        return system;
    }

    ///
    /// Returns an immutable reference of a [`ComponentSystem`] object
    /// which is present in the [`EntityManager`] object
    ///
    /// WARNING: The component must be registered in the system, otherwise the
    /// function will result in a panic
    ///
    fn get_system<C: Component + Sized + 'static>(&self) -> &ComponentManager<C> {
        let system = self
            .component_systems
            .get(&Self::get_component_type_id::<C>())
            .unwrap();
        let system = system
            .as_any()
            .downcast_ref::<ComponentManager<C>>()
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

    ///
    /// Creates an entity in contiguous block
    ///
    /// Returns an EntityId, which must be used to perform
    /// all further operations on the entity
    ///
    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = self.generate_entity_id();
        let entity_index = self.entities.len();

        // Updating array and lookup
        self.entities.push(Entity { id: entity_id });
        self.entity_id_map.insert(entity_id, entity_index);

        return entity_id;
    }

    ///
    /// Removes the entity and all components attached to it
    ///
    /// The [`entity_id`](EntityId) passed in the parameter is
    /// invalidated and any future operations on the entity
    /// will result in a panic
    ///
    pub fn remove_entity(&mut self, entity_id: EntityId) {
        todo!()
    }

    ///
    /// Registers a component type in the manager by creating a
    /// component systems for handling the component
    ///
    /// Components of this type can be attached to generated entities
    /// only after registering the component type in the manager.
    ///
    pub fn register_component<C: Component + 'static>(&mut self) {
        if self.check_component_registered::<C>() {
            println!("Component already registered: {}", C::get_name());
            return;
        }

        // Creating system for the component type
        self.component_systems.insert(
            Self::get_component_type_id::<C>(),
            Box::new(ComponentManager::<C>::new()),
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
    /// **Description:**
    ///     Returns true if the given component type is attached to
    ///     the entity, false otherwise
    ///
    /// **SAFETY:**
    ///     - Calling this function with a component which is not registered
    ///     in the system will result in a panic
    ///
    pub fn has_component<C: Component + Sized + 'static>(&self, entity_id: EntityId) -> bool {
        assert!(
            self.check_component_registered::<C>(),
            "Component not registered for use {}",
            C::get_name()
        );

        // Getting the appropriate system for the component
        let system = self.get_system::<C>();

        // Querying for component presence
        system.has_component(entity_id)
    }

    pub fn get_all_components<C: Component + 'static>(&self) -> Vec<(&C, &EntityId)> {
        let system = self.get_system::<C>();
        system.get_all_components()
    }

    pub fn get_all_components_mut<C: Component + 'static>(&mut self) -> Vec<(&mut C, &EntityId)> {
        let system = self.get_system_mut::<C>();
        system.get_all_components_mut()
    }

    ///
    /// **Description:**
    ///     Used to get a mutable reference of the component attached to the
    ///     specified entity.
    ///
    /// **SAFETY**:
    ///    - The component type requested by the user must be registered in the system
    pub fn get_component_mut_ref<C: Component + 'static>(&mut self, entity_id: EntityId) -> &mut C {
        assert!(
            self.check_component_registered::<C>(),
            "Component not registered for use {}",
            C::get_name()
        );

        let system = self.get_system_mut::<C>();
        system.borrow_component_mut(entity_id)
    }
}
