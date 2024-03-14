use std::{
    any::TypeId,
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell, RefMut},
    collections::HashMap,
};

use crate::{
    component::{
        component_manager::{ComponentManager, EcsManager},
        resource::{Resource, ResourceId, ResourceRaw},
        Component,
    },
    entity::{entity_manager::EntityManager, Entity},
};

/// ### World struct
///
/// The [`World`] struct is responsible for storing the state information of
/// the ECS application running.
///
/// ### Description
///
/// The world struct also acts as an API for the user to modify or access the
/// current state of the system based on logic defined in a system designed by the
/// user.
///
/// It has the following fields
/// 1. [`entity manager`](World::entity_manager) - This is the entity manager responsible
/// for generating generational id. For more info on generational id, See [Entity]. This structure generates
/// ids for entities, which are used by [`component managers`]()
///
/// 2. [`component manager`](World::component_managers) - A component manager is a struct
/// which implements the EcsManager trait and is responsible for handling affairs related
/// to a specific type of user defined component. A component to be handled in the world
/// must first be registered in the world, which in turn creates the appropriate
/// manager for the component.
///
///
///
pub struct World {
    active: bool,
    cleanup: bool,

    /// Structure responsible for managing entities in the world
    entity_manager: EntityManager,

    /// Component systems based on component types
    component_managers: HashMap<TypeId, Box<dyn EcsManager>>,

    /// Resources present in the world
    // resources: HashMap<ResourceId, RefCell<Box<dyn Resource>>>,
    resources: HashMap<ResourceId, Box<RefCell<dyn Resource>>>,
}

/// Private member implementations
impl World {
    ///
    /// Get the type id of component. Used to fetch the system
    /// of the component based on it's type id
    ///
    fn get_component_type_id<C: Component + 'static>() -> TypeId {
        TypeId::of::<C>()
    }

    /// Checks if the component has been registered
    ///
    /// This validation must be performed before performing operations
    /// on a component
    fn check_component_registered<C: Component + 'static>(&self) -> bool {
        self.component_managers.contains_key(&TypeId::of::<C>())
    }

    // @TODO: Think about the panic behavior, is it right?

    ///
    /// ### Description:
    /// Returns a mutable reference of a [`ComponentSystem`] object
    /// which is present in the [`EntityManager`] object
    ///
    /// WARNING: The component must be registered in on of the [`component manager`](World::component_managers), otherwise the
    /// function will result in a panic
    ///
    fn get_manager_mut<C: Component + Sized + 'static>(&mut self) -> &mut ComponentManager<C> {
        let system = self
            .component_managers
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
    /// WARNING: The component must be registered on of the [`component manager`](World::component_managers), otherwise the
    /// function will result in a panic
    ///
    fn get_manager<C: Component + Sized + 'static>(&self) -> &ComponentManager<C> {
        let system = self
            .component_managers
            .get(&Self::get_component_type_id::<C>())
            .unwrap();
        let system = system
            .as_any()
            .downcast_ref::<ComponentManager<C>>()
            .unwrap();

        return system;
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            active: false,
            cleanup: false,
            entity_manager: EntityManager::new(),
            component_managers: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    /// Creates an entity in the world and returns its id
    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }

    /// Removes an entity from the world and deallocates all components
    /// attached to it.
    pub fn remove_entity(&mut self, entity_id: Entity) {
        // Dispose all components attached to the entity
        for (_, system) in &mut self.component_managers {
            if system.has_component(entity_id) {
                system.remove_component_from_entity(entity_id);
            }
        }

        self.entity_manager.dispose_entity_id(entity_id);
    }

    ///### Description
    ///
    /// Registers a component type in the manager by creating a
    /// component systems for handling the component
    ///
    /// Components of this type can be attached to generated entities
    /// only after registering the component type in the [`manager`](World::component_managers).
    ///
    pub fn register_component<C: Component + 'static>(&mut self) {
        if self.check_component_registered::<C>() {
            println!("Component already registered: {}", C::get_name());
            return;
        }

        // Creating system for the component type
        self.component_managers.insert(
            Self::get_component_type_id::<C>(),
            Box::new(ComponentManager::<C>::new()),
        );
    }

    /// Adding resource to the world
    pub fn add_resource<R: Resource + Sized + 'static>(&mut self, resource: R) {
        assert!(!self.resources.contains_key(&R::get_type()));
        // self.resources
        //     .insert(R::get_type(), RefCell::new(Box::new(resource)));
        self.resources
            .insert(R::get_type(), Box::new(RefCell::new(resource)));
    }

    pub fn get_resource_mut<R: Resource + Sized + 'static>(&mut self) -> RefMut<'_, R> {
        assert!(self.resources.contains_key(&R::get_type()));
        self.resources.get_mut(&R::get_type()).unwrap().try_borrow_mut().unwrap()
    }

    // pub fn get_resource_mut<R: Resource + Sized + 'static>(&mut self) -> RefMut<'_, R> {
    //     assert!(self.resources.contains_key(&R::get_type()));
    //     self.resources
    //         .get_mut(&R::get_type())
    //         .unwrap()
    //         .try_borrow_mut()
    //         .unwrap()
    // }

    ///
    /// ### Description
    ///
    /// Adds the given component type into the system
    ///
    /// WARNING: Calling this function with a component which
    ///     is not registered will result in a panic
    ///
    pub fn add_component_to_entity<C: Component + Sized + 'static>(
        &mut self,
        entity_id: Entity,
        component: C,
    ) {
        assert!(
            self.check_component_registered::<C>(),
            "Component not registered for use: {}",
            C::get_name()
        );

        let system = self.get_manager_mut::<C>();
        system.add_component_to_entity(entity_id, component);
    }

    ///
    /// ### Description
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
        entity_id: Entity,
    ) {
        assert!(
            !self.has_component::<C>(entity_id),
            "Component Removal failed: Component does not exist in the entity with id {:?}",
            entity_id
        );
        let system = self.get_manager_mut::<C>();
        system.remove_component_from_entity(entity_id);
    }

    ///
    /// ### Description
    ///
    /// Returns true if the given component type is attached to
    /// the entity, false otherwise
    ///
    /// #### SAFETY:
    ///
    /// Calling this function with a component which is not registered
    /// in the system will result in a panic
    ///
    pub fn has_component<C: Component + Sized + 'static>(&self, entity_id: Entity) -> bool {
        assert!(
            self.check_component_registered::<C>(),
            "Component not registered for use {}",
            C::get_name()
        );

        // Getting the appropriate system for the component
        let system = self.get_manager::<C>();

        // Querying for component presence
        system.has_component(entity_id)
    }

    /// ### Description
    ///
    /// Returns all components of a specific type defined by the generic type
    /// parameter.
    ///
    /// This function initiates a call to the appropriate component manager for the
    /// type and gets the list of components.
    ///
    /// ### Returns
    ///
    /// A pair of Component and Entity references.
    pub fn get_all_component_ids<C: Component + 'static>(&self) -> Vec<&Entity> {
        let system = self.get_manager::<C>();
        system.get_all_component_ids()
    }

    ///
    /// ### Description:
    ///
    /// Used to get a mutable reference of the component attached to the
    /// specified entity.
    ///
    /// #### SAFETY:
    ///
    /// The component type requested by the user must be registered in the system
    pub fn get_component_mut_ref<C: Component + 'static>(
        &mut self,
        entity_id: Entity,
    ) -> RefMut<'_, C> {
        assert!(
            self.check_component_registered::<C>(),
            "Component not registered for use {}",
            C::get_name()
        );

        let system = self.get_manager_mut::<C>();
        system.borrow_component_mut(entity_id)
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

/// ### Description
/// Structure for storing unsafe world reference in ECS App
///
/// The unsafe world container stores the World instance in a
/// Cell structure, hence allowing us to use interior mutability
/// on the World instance.
///
/// We require world to have interior mutability for following reasons
/// - Systems being launched in the ECS systems take in a mutable
/// reference to the world API so that the user can have the flexibility
/// to make modifications to world based on component update logic
/// - We want to supply a mutable reference the current component being
/// processed in the system, which requires us to make both mutable and immutable
/// references to the world structure
/// - This however poses problems, since we need mutable reference for user interface,
/// but need the immutable world interface ourselves at the same time.
/// - Though this code is not unsafe since we do not use the immutable
/// reference once we get the entity ids, we are nevertheless punished by the
/// rust borrow checker for getting a mutable reference to a memory which
/// has been used as immutable in the same context.
/// - Hence, we use this unsafe structure to bypass the bounds of rust typechecker
/// while taking the responsibility of memory safety on ourselves.
///
/// For more on component specific safety, see [ComponentManager]
///
///
pub(crate) struct UnsafeWorldContainer {
    pub(crate) world: Cell<World>,
}

impl UnsafeWorldContainer {
    pub(crate) fn new() -> Self {
        Self {
            world: Cell::new(World::new()),
        }
    }

    /// SAFETY:
    /// The caller must ensure the safety of the memory access for world.
    pub fn get_world(&self) -> &World {
        unsafe { &(*self.world.as_ptr()) }
    }

    /// Returns an unchecked mutable reference of to the world
    /// SAFETY:
    /// The caller must ensure that the mutable reference being borrowed
    /// from this function is safe to be accessed.
    /// This function is supposed to be called to get mutable references
    /// to components out of the world for processing in [`process_update`](BaseSystem::process_update)
    pub fn get_world_mut(&self) -> &mut World {
        unsafe { &mut *(self.world.as_ptr()) }
    }
}

pub type WorldArg = World;
