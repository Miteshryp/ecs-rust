pub(crate) mod unsafe_world;
pub mod command_type;

use std::{any::TypeId, sync::{mpsc::{channel, Receiver, Sender}, Arc}};

use tokio::sync::{
    OwnedRwLockReadGuard, OwnedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};

use hashbrown::HashMap;

use crate::{
    component::{
        component_manager::{ComponentManager, EcsManager},
        Component,
    },
    entity::{entity_manager::EntityManager, Entity},
    events::{event_manager::EventManager, Event},
    resource::{Resource, ResourceId},
    system::param::{EventReader, EventWriter, MutResourceHandle, SystemQuery},
};

use self::command_type::CommandFunction;

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

    // Event processing unit
    event_manager: EventManager,

    command_sender: Sender<CommandFunction>,
    // command_receiver: Receiver<CommandFunction>,

    /// Resources present in the world
    resources: HashMap<ResourceId, Arc<RwLock<Box<dyn Resource>>>>,
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

    /// ### Description
    ///
    /// Checks if the component has been registered
    ///
    /// This validation must be performed before performing operations
    /// on a component
    fn check_component_registered<C: Component + 'static>(&self) -> bool {
        self.component_managers.contains_key(&TypeId::of::<C>())
    }

    // @DONE: Think about the panic behavior, is it right?
    //          We're shifting to Options slowly
    // @TODO: Audit all functions to follow an Option based
    //          response process for undefined behavior.

    ///
    /// ### Description (Internal):
    ///
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
    /// ### Description (Internal):
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
    pub fn new(command_sender: Sender<CommandFunction>) -> Self {
        let (tx, rx) = channel::<CommandFunction>();
        Self {
            active: false,
            cleanup: false,
            entity_manager: EntityManager::new(),
            component_managers: HashMap::new(),
            event_manager: EventManager::new(),
            resources: HashMap::new(),
            command_sender,
        }
    }

    /// ### Description
    ///
    /// Creates an entity in the world and returns its [`id`](Entity)
    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }

    ///
    /// ### Description:
    ///
    /// Removes an entity from the world and deallocates all components
    /// attached to it.
    pub fn remove_entity(&mut self, entity_id: Entity) {
        // Dispose all components attached to the entity
        // @TODO: Check whether the entity is actually valid before launching remove_component_from_entity functions

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

        // @TODO: Remove assertion and write option based logic
        assert!(!self.resources.contains_key(&R::type_id()));

        // self.resources.insert(R::get_type(), Box::new(resource));
        // self.resources.insert(R::get_type(), RwLock::new(Box::new(resource)));

        self.resources
            .insert(R::type_id(), Arc::new(RwLock::new(Box::new(resource))));
    }

    pub fn remove_resource<R: Resource + Sized + 'static>(&mut self) {

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
        // @TODO: Remove assertion and add Option or Result here 
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
    // pub fn get_all_component_ids<C: Component + 'static>(&self) -> Vec<&Entity> {
    //     let system = self.get_manager::<C>();
    //     system.get_entities()
    // }

    

    pub fn update_event_state(&mut self) {
        self.event_manager.refresh_update();
    }

    /// 
    /// ### Description
    /// 
    /// Update the world based on the command buffers received
    pub fn update_world(&mut self) {

    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl World {
    // @SOLVED: If parallel systems try to create an event reader for the same event type
    // which hasn't been used previously, it may cause trouble to create the vector for the
    // event type
    pub(crate) fn get_event_reader<E: Event + 'static>(&mut self) -> Option<EventReader<E>> {
        self.event_manager.get_reader()
    }

    pub(crate) fn get_event_writer(&mut self) -> EventWriter {
        self.event_manager.get_writer()
    }

    // @TODO: Document
    pub(crate) fn get_component_ref_mut_lock<C: Component + 'static>(
        &mut self,
        entity_id: Entity,
    ) -> Option<OwnedRwLockWriteGuard<C>> {
        assert!(
            self.check_component_registered::<C>(),
            "Component not registered for use {}",
            C::get_name()
        );

        let system = self.get_manager_mut::<C>();
        match system.borrow_component_mut(entity_id) {
            Some(component_ref) => Some(component_ref),
            None => None,
        }
    }

    pub(crate) fn get_component_ref_lock<C: Component + 'static>(
        &mut self,
        entity_id: Entity,
    ) -> Option<OwnedRwLockReadGuard<C>> {
        assert!(
            self.check_component_registered::<C>(),
            "Component not registered for use {}",
            C::get_name()
        );

        let system = self.get_manager_mut::<C>();
        match system.borrow_component(entity_id) {
            Some(component_ref) => Some(component_ref),
            None => None,
        }
    }

    /// Returns an Owned Write Guard to a resource in the world
    ///
    // @TODO: Change function name to be more suitable
    pub(crate) fn get_resource_mut<R: Resource + Sized + 'static>(
        &mut self,
    ) -> Option<OwnedRwLockWriteGuard<Box<dyn Resource>>> {
        match self.resources.get(&R::type_id()) {
            Some(x) => {
                match x.clone().try_write_owned() {
                    Ok(lock) => Some(lock),
                    Err(_) => None,
                }
            },
            None => None,
        }

        // match self
        //     .resources
        //     .get(&R::type_id())
        //     .unwrap()
        //     .clone()
        //     .try_write_owned()
        // {
        //     Ok(x) => Some(x),
        //     Err(_) => None,
        // }
    }


    pub(crate) fn get_resource_ref<R: Resource + Sized + 'static>(
        &self,
    ) -> Option<OwnedRwLockReadGuard<Box<dyn Resource>>> {
        match self.resources.get(&R::type_id()) {
            Some(x) => {
                match x.clone().try_read_owned() {
                    Ok(lock) => Some(lock),
                    Err(_) => None,
                }
            },
            None => None,
        }

        // match self
        //     .resources
        //     .get(&R::type_id())
        //     .unwrap()
        //     .clone()
        //     .try_read_owned()
        // {
        //     Ok(x) => Some(x),
        //     Err(_) => None,
        // }
    }


    ///
    /// ### Description
    ///
    ///  Returns an array of [`EntityIds`](Entity) which have the
    /// components with the types specified in the input parameter
    ///
    /// ### Parmameters
    /// - `Query` [SystemQuery] type defining the components to be fetched
    ///         from the world
    ///
    pub(crate) fn get_entities_with_components<QueryType: SystemQuery>(
        &self,
    ) -> hashbrown::HashSet<&Entity> {
        // Getting all active entities in the world
        // Initially we assume it asks for all entities
        let mut active_entities = self.entity_manager.get_active_entities();

        // Array of TypeId of Components that the query demands
        let component_ids = QueryType::get_query_entities();

        // Finding appropriate component manager for each type
        for cid in component_ids {
            let component_manager = match self.component_managers.get(&cid) {
                Some(x) => x,
                None => {
                    log::error!(
                        "Failed to get manager: TypeId {:?} does not belong to a registered component",
                        cid
                    );
                    return hashbrown::HashSet::new();
                }
            };

            // We only keep the intersection of entities with all the previous
            // component and the entities with the current component.
            // This ensures that we only shortlist entities which have
            // all the enlisted components attached to them
            let component_entities: hashbrown::HashSet<&Entity> =
                component_manager.get_entities().into_iter().collect();
            active_entities = component_entities
                .intersection(&active_entities)
                .cloned()
                .collect();
        }

        active_entities
    }


    // pub(crate) fn get_command_receiver(&self) -> Receiver<CommandFunction> {
    //     self.command_receiver.
    // }


    pub(crate) fn get_command_writer(&self) -> Sender<CommandFunction> {
        self.command_sender.clone()
    }

}
