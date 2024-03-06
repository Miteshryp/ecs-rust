use super::{Component, ComponentHandler};
use crate::{entity::EntityId, system::System};

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

/// Contains the interface for implementing the logic for handling a specific type of component
///
/// @NOTE: One entity can only have one instance of a type of component, so maybe component id is useless
///     For a component, we can simply use the entity id to identify the component attached to the entity
pub struct ComponentSystem<Comp>
where
    Comp: Component,
{
    system_id: TypeId,

    /// Contiguous array of components which enables us to do faster
    /// update operations due to better cache locality.
    components: Vec<Comp>,

    /// Stores the entity ids of the components in the components vector
    /// in the same order as the components are presented in the array
    /// above.
    entity_ids: Vec<EntityId>,

    /// [EntityId] to 'index' mapping to boost lookup
    ///
    /// NOTE: We use [EntityId] to identify a component in the system since
    ///     each component in existence has to be attached to an entity,
    ///     therefore for a component system managing a specific type of component,
    ///     each component is attached to a unique entity, hence we can use
    ///     entity id as the identifier for the components
    entity_component_map: HashMap<EntityId, usize>,

    /// Handler attached to the system object for enabling custom updation
    /// of the components in the system
    handler: Box<dyn ComponentHandler<Comp>>,
}
impl<Comp> System for ComponentSystem<Comp> where Comp: Component + 'static {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}



/// Public functions of the ComponentSystem struct
impl<Comp> ComponentSystem<Comp>
where
    Comp: Component + 'static,
{
    /// System initialisation function. Used to create a new system to handle the specified type of component
    pub fn new<Handler>() -> Self
    where
        Handler: ComponentHandler<Comp>,
    {
        ComponentSystem {
            system_id: TypeId::of::<Self>(),
            components: vec![],
            entity_ids: vec![],
            entity_component_map: HashMap::new(),
            handler: Box::new(Handler::new()),
        }
    }

    /// Adds a component into the system based on the stack-build object passed
    /// as a parameter
    pub fn add_component_to_entity(&mut self, entity_id: EntityId, component: Comp) {
        if self.entity_component_map.contains_key(&entity_id) {
            // Entity already has the component attached to it.
            // @TODO: Handle error
            return;
        }

        let component_index = self.components.len();
        self.components.push(component);
        self.entity_component_map
            .insert(entity_id, component_index)
            .unwrap();
    }

    /// Removes a component from the entity if the component was attached,
    pub fn remove_component_from_entity(&mut self, entity_id: EntityId) {
        if !self.entity_component_map.contains_key(&entity_id) {
            // Entity does not have the component attached to it, hence removal is not possible
            // @TODO: Handle error
            return;
        }

        let components_length = self.components.len();
        let component_index = self.entity_component_map.get(&entity_id).unwrap();

        // O(1) removal time, without disturbing majority of the elements indexes
        self.components.swap_remove(*component_index);
        self.entity_ids.swap_remove(*component_index);

        // Updating the lookup table with the replaced indexes
        if components_length - 1 != *component_index {
            self.entity_component_map
                .insert(self.entity_ids[*component_index], *component_index)
                .unwrap();
        }
        self.entity_component_map.remove(&entity_id).unwrap();
    }

    /// Returns 
    ///     true if the component is present in the entity
    ///     false otherwise
    pub fn has_component(&self, entity_id: EntityId) -> bool {
        self.entity_component_map.contains_key(&entity_id)
    }

    pub fn borrow_component(&self, entity_id: EntityId) -> &Comp {

        // @TODO: Change all verification checks into asserts
        if !self.entity_component_map.contains_key(&entity_id) {
            panic!("Component does not exist in the entity id: {entity_id}")
        }

        // Getting the component from the array.
        // @SAFETY: Component is guarenteed to be present in the vector since 
        //      the entity id is present in the lookup table
        self.components.get(*self.entity_component_map.get(&entity_id).unwrap()).unwrap()
    }

    pub fn borrow_component_mut(&mut self, entity_id: EntityId) -> &mut Comp {
        // @TODO: Change all verification checks into asserts
        if !self.entity_component_map.contains_key(&entity_id) {
            panic!("Component does not exist in the entity id: {entity_id}")
        }

        // Getting the component from the array.
        // @SAFETY: Component is guarenteed to be present in the vector since 
        //      the entity id is present in the lookup table
        self.components.get_mut(*self.entity_component_map.get(&entity_id).unwrap()).unwrap()
    }

    pub fn get_system_id(&self) -> TypeId {
        self.system_id
    }
}
