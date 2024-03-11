use super::Component;
use crate::{
    entity::{entity_manager::EntityManager, Entity},
    system::EcsManager,
    world::World,
};

use std::{
    any::{Any, TypeId}, borrow::Borrow, cell::{Ref, RefCell, RefMut}, collections::HashMap
};

/// Contains the interface for implementing the logic for handling a specific type of component
///
/// @NOTE: One entity can only have one instance of a type of component, so maybe component id is useless
///     For a component, we can simply use the entity id to identify the component attached to the entity
pub(crate) struct ComponentManager<Comp>
where
    Comp: Component,
{
    system_id: TypeId,

    /// Contiguous array of components which enables us to do faster
    /// update operations due to better cache locality.
    /// RefCell dynamically checks the mutable references to a component, 
    /// hence we cannot have more than one mutable reference to a component
    components: Vec<RefCell<Comp>>, 
    

    /// Stores the entity ids of the components in the components vector
    /// in the same order as the components are presented in the array
    /// above.
    entity_ids: Vec<Entity>,

    /// [Entity] to 'index' mapping to boost lookup
    ///
    /// NOTE: We use [Entity] to identify a component in the system since
    ///     each component in existence has to be attached to an entity,
    ///     therefore for a component system managing a specific type of component,
    ///     each component is attached to a unique entity, hence we can use
    ///     entity id as the identifier for the components
    entity_component_map: HashMap<Entity, usize>,
}
impl<Comp> EcsManager for ComponentManager<Comp>
where
    Comp: Component + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    /// System initialisation function. Used to create a new system to handle the specified type of component
    fn new() -> Self {
        ComponentManager {
            system_id: TypeId::of::<Self>(),
            components: vec![],
            entity_ids: vec![],
            entity_component_map: HashMap::new(),
        }
    }

    /// Removes a component from the entity if the component was attached,
    fn remove_component_from_entity(&mut self, entity_id: Entity) {
        // Entity must have a component of type [`Comp`] attached for removal to be possible
        assert!(
            self.entity_component_map.contains_key(&entity_id),
            "Entity [{:?}] does not have a component of type \'{}\'",
            entity_id,
            Comp::get_name()
        );

        let components_length = self.components.len();
        let component_index = self.entity_component_map.get(&entity_id).unwrap();

        // O(1) removal time, without disturbing majority of the elements indexes
        self.components.swap_remove(*component_index);
        self.entity_ids.swap_remove(*component_index);

        // Updating the lookup table for the replaced indexes if any element
        // in the middle of the array was removed.
        if components_length - 1 != *component_index {
            self.entity_component_map
                .insert(self.entity_ids[*component_index], *component_index)
                .unwrap();
        }

        // Removing removed entity_id from lookup
        self.entity_component_map.remove(&entity_id).unwrap();
    }

    ///
    /// Used to find whether the component is attached to the `entity_id`
    ///
    /// Returns
    ///     true if the component is present in the entity
    ///     false otherwise
    fn has_component(&self, entity_id: Entity) -> bool {
        self.entity_component_map.contains_key(&entity_id)
    }
}

/// Public functions of the ComponentSystem struct
impl<Comp> ComponentManager<Comp>
where
    Comp: Component + 'static,
{
    /// Adds a component into the system based on the stack-build object passed
    /// as a parameter
    pub fn add_component_to_entity(&mut self, entity_id: Entity, component: Comp) {
        // Entity should not already have the component attached to it.
        assert!(
            !self.entity_component_map.contains_key(&entity_id),
            "Component addition to Entity [{:?}] failed: Duplicate components are not allowed in entities.", 
            entity_id
        );

        let component_index = self.components.len();
        // self.components.push(component);

        self.components.push(RefCell::new(component));
        self.entity_ids.push(entity_id);
        self.entity_component_map.insert(entity_id, component_index);
        // .unwrap();
    }

    pub fn borrow_component(&self, entity_id: Entity) -> Ref<'_, Comp> {
        assert!(
            self.entity_component_map.contains_key(&entity_id),
            "Component does not exist in the entity id: {:?}",
            entity_id
        );

        // @SAFETY: Component is guarenteed to be present in the vector since
        //      the entity id is present in the lookup table
        self.components
            .get(*self.entity_component_map.get(&entity_id).unwrap())
            .unwrap()
            .borrow()
    }

    pub fn borrow_component_mut(&self, entity_id: Entity) -> RefMut<'_, Comp> {
        assert!(
            self.entity_component_map.contains_key(&entity_id),
            "Component does not exist in the entity id: {:?}",
            entity_id
        );

        // @SAFETY: Component is guarenteed to be present in the vector since
        //      the entity id is present in the lookup table
        self.components
            .get(*self.entity_component_map.get(&entity_id).unwrap())
            .unwrap()
            .borrow_mut()
    }

    pub fn get_all_component_ids(&self) -> Vec<&Entity> {
        // self.components.iter().map(|c| c.borrow()).zip(self.entity_ids.iter()).collect()
        self.entity_ids.iter().collect()
    }

    // pub fn get_all_components_mut(&self) -> Vec<(RefMut<'_, Comp>, &Entity)> {
    //     let v: Vec<(RefMut<'_, Comp>, &Entity)> = self
    //         .components
    //         .iter()
    //         .map(|c| c.borrow_mut())
    //         .zip(self.entity_ids.iter())
    //         .collect();
    //     println!("{}", v.len());
    //     v
    // }

    pub fn get_system_id(&self) -> TypeId {
        self.system_id
    }
}