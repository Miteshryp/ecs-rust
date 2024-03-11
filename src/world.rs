use std::{any::TypeId, borrow::Borrow, cell::{Cell, Ref, RefCell, RefMut}, collections::HashMap, ops::{Deref, DerefMut}, rc::Rc};

use crate::{
    component::{component_manager::ComponentManager, Component},
    entity::{entity_manager::EntityManager, Entity}, system::EcsManager,
};


/// ### World struct
/// 
/// The [`World`] struct is responsible for storing the state information of
/// the ECS application running.
pub struct World {
    active: bool,
    cleanup: bool,

    /// Structure responsible for managing entities in the world
    entity_manager: EntityManager,

    /// Component systems based on component types
    component_systems: HashMap<TypeId, Box<dyn EcsManager>>,

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
        self.component_systems.contains_key(&TypeId::of::<C>())
    }

    ///
    /// Returns a mutable reference of a [`ComponentSystem`] object
    /// which is present in the [`EntityManager`] object
    ///
    /// WARNING: The component must be registered in the system, otherwise the
    /// function will result in a panic
    ///
    fn get_manager_mut<C: Component + Sized + 'static>(&mut self) -> &mut ComponentManager<C> {
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
    fn get_manager<C: Component + Sized + 'static>(&self) -> &ComponentManager<C> {
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

impl World {
    pub fn new() -> Self {
        Self {
            active: false,
            cleanup: false,
            entity_manager: EntityManager::new(),
            component_systems: HashMap::new(),
        }
    }


    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }

    pub fn remove_entity(&mut self, entity_id: Entity) {

        // Dispose all components attached to the entity
        for (_, system) in &mut self.component_systems {
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
    /// only after registering the component type in the manager.
    ///
    pub fn register_component<C: Component + 'static>(&mut self) {
        println!("Registering component");
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
    /// ### Description:
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

    /// ### Description
    /// 
    /// Returns all mutable components of a specific type defined by the generic type
    /// parameter.
    /// 
    /// This function initiates a call to the appropriate component manager for the 
    /// type and gets the list of mutable components.
    /// 
    /// ### Returns
    /// 
    /// A pair of mutable Component and Entity references.
    // pub(crate) fn get_all_components_mut<C: Component + 'static>(&mut self) -> Vec<(RefMut<'_, C>, &Entity)> {
    //     let system = self.get_manager_mut::<C>();
    //     system.get_all_components_mut()
    // }

    ///
    /// ### Description:
    /// 
    /// Used to get a mutable reference of the component attached to the
    /// specified entity.
    ///
    /// #### SAFETY:
    /// 
    /// The component type requested by the user must be registered in the system
    pub fn get_component_mut_ref<C: Component + 'static>(&mut self, entity_id: Entity) -> RefMut<'_, C> {
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

    /// Collection functions
    pub fn get_all_entities(&self) -> Vec<Entity> {
        todo!()
    }
}


pub(crate) struct UnsafeWorldContainer {
    pub(crate) world: Cell<World>,
}

impl UnsafeWorldContainer {
    pub(crate) fn new() -> Self {
        Self {
            world: Cell::new(World::new())
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
    /// to components out of the world for processing in [`BaseSystem::process_update`](BaseSystem)
    pub fn get_world_mut(&self) -> &mut World {
        unsafe { &mut *(self.world.as_ptr()) }
    }
}


pub type WorldArg = World;
// pub type WorldArg = Cell<World>;
// pub type WorldArg = Rc<Cell<World>>;
// pub type WorldArg = Rc<RefCell<World>>;
// pub type WorldArg<'a> = std::cell::RefMut<'a, World> ;
