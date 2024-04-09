use ecs_macros::ECSBase;

use super::Component;
use crate::{
    ecs_base::ECSBase,
    entity::{Entity},
};

use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
};

use hashbrown::HashMap;

/// 
/// ### Description
/// 
/// Interface of Component Manager which is to be exposed to the world
/// struct.
/// 
pub trait EcsManager: ECSBase {
    fn new() -> Self
    where
        Self: Sized;
    fn remove_component_from_entity(&mut self, entity_id: Entity);
    fn has_component(&self, entity_id: Entity) -> bool;
    fn get_entities(&self) -> Vec<&Entity>;
}

///
/// ### Description
/// Structure for managing a components of a specific type
///
/// This is an internal structure which is stored in the [`world`](crate::App::world), and is responsible
/// for storing, modifying and ensuring safety of components created in a given world
/// The components stored in this struct are identified using their entity_id, where we
/// create a B-Tree structure to quickly trace a component in the array
///
///
///
/// This structure implements the base [EcsManager] interface because
/// a base trait is required for storing managers in the world, since
/// each manager has a different component type which cannot be known
/// during declaration, i.e. compile time.
///
/// #### SAFETY:
/// Also, each [`component`](ComponentManager::components) is stored as a [RefCell], which ensures
/// that the component does not have illegal references anywhere in the program.
///
///

#[derive(ECSBase)]
pub(crate) struct ComponentManager<Comp>
where
    Comp: Component,
{
    /// Contiguous array of components which enables us to do faster
    /// update operations due to better cache locality.
    /// RefCell dynamically checks the mutable references to a component,
    /// hence we cannot have illegal reference to a component
    components: Vec<RefCell<Comp>>,

    /// Stores the [entity ids](Entity) of the [`components`](ComponentManager::components)
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
    /// System initialisation function. Used to create a new system to handle the specified type of component
    fn new() -> Self {
        ComponentManager {
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
        // Removed element is replaced by the last element in the array
        self.components.swap_remove(*component_index);
        self.entity_ids.swap_remove(*component_index);

        // Updating the lookup table for the replaced indexes only if any element
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
    /// ### Description:
    ///
    /// Used to find whether the component is attached to the [`entity_id`](Entity)
    ///
    /// ### Return Value:
    /// [`true`](bool) if the component is present in the entity
    /// [`false`](bool) otherwise
    fn has_component(&self, entity_id: Entity) -> bool {
        self.entity_component_map.contains_key(&entity_id)
    }
    
    ///
    /// ### Description
    /// 
    /// Returns an array of all active [`EntityId`](Entity)s which have 
    /// the [`Component`] of type ['Comp'](ComponentManager<Comp>) attached to them
    fn get_entities(&self) -> Vec<&Entity> {
        self.entity_ids.iter().collect()
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

        self.components.push(RefCell::new(component));
        self.entity_ids.push(entity_id);
        self.entity_component_map.insert(entity_id, component_index);
    }

    /// 
    /// ### Description
    /// Used to get a immutable reference to a component in the manager
    ///
    /// #### SAFETY:
    /// [`components`](ComponentManager::components) is protected by a [`RefCell`], hence the reference
    /// to the component cannot be used illegally.
    pub fn borrow_component(&self, entity_id: Entity) -> Option<Ref<'_, Comp>> {
        match self
            .components
            .get(*self.entity_component_map.get(&entity_id).unwrap())
        {
            Some(component) => match component.try_borrow() {
                Ok(x) => Some(x),
                Err(err) => {
                    // Imutable reference is not possible
                    log::error!(
                        "Component [id: {:?}, type: {:?}] -> Immutable reference not possible. \n{}",
                        entity_id,
                        TypeId::of::<Comp>(),
                        err.to_string()
                    );
                    None
                }
            },
            None => {
                // Component does not exist in the entity
                log::error!(
                    "Component [id: {:?}, type: {:?}] -> Component does not exist in the entity.",
                    entity_id,
                    TypeId::of::<Comp>(),
                );
                None
            }
        }
    }

    /// Used to get a mutable reference to a component in the manager
    ///
    /// #### SAFETY:
    /// [`components`](ComponentManager::components) is protected by RefCell,
    /// hence illegal code crashes at runtime.
    /// Given that the validity of the world is protected by the scheduler, the
    /// RefMut returned from this function is guaranteed to be valid.
    pub fn borrow_component_mut(&self, entity_id: Entity) -> Option<RefMut<'_, Comp>> {
        match self
            .components
            .get(*self.entity_component_map.get(&entity_id).unwrap())
        {
            Some(component) => match component.try_borrow_mut() {
                Ok(x) => Some(x),
                Err(err) => {
                    // Mutable reference is not possible
                    log::error!(
                        "Component [id: {:?}, type: {:?}] -> Mutable reference not possible. \n{}",
                        entity_id,
                        TypeId::of::<Comp>(),
                        err.to_string()
                    );
                    None
                }
            },
            None => {
                // Component does not exist in the entity
                log::error!(
                    "Component [id: {:?}, type: {:?}] -> Component does not exist in the entity.",
                    entity_id,
                    TypeId::of::<Comp>(),
                );
                None
            }
        }
    }

    
    // Moved to EcsManager interface
    // /// Gets a vec of &[id](Entity) of all the components currently
    // /// alive in the manager
    // pub fn get_all_component_ids(&self) -> Vec<&Entity> {
    //     self.entity_ids.iter().collect()
    // }
}
