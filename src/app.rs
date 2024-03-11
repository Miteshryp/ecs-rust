use std::{cell::RefCell, rc::Rc};

use crate::{
    component::Component,
    system::{BaseSystem, ComponentSystem},
    world::{UnsafeWorldContainer, World},
};

/// ### ECS App
/// The main Application responsible for enclosing the ECS system
///
/// The [`App`] struct is the user level interface for starting the ECS system
/// and initiating the interactions with the system.
/// The main function of the [`App`] struct is to register different user systems
/// which handle different [`Component`]s and [`ComponentSystem`]s across the system
///
/// ---
///
/// The [`App`] struct contains 2 fields:
///     - #### world:
///         In laymens terms, this is the state container of the system. See
///         [`World`] for more info.

///     - #### systems:
///         This is a vector of systems stored in the `App` scheduled for execution
///         on the Application. This vector stores a `BaseSystem` type, however this
///         `BaseSystem` class is implemented in a specialised way by the derivable
///         macro for the type of the system. See [`ecs_macro`](ecs_macro) for more
///
/// ---
///
/// ### Example:
///
/// ```
/// // @TODO: Write this example
/// fn main() {
///     
/// }
/// ```
pub struct App {
    world_container: UnsafeWorldContainer,
    systems: Vec<Box<dyn BaseSystem>>,
}

impl App {
    pub fn new() -> Self {
        App {
            world_container: UnsafeWorldContainer::new(),
            systems: vec![],
        }
    }

    /// Registers the component of the system and adds the system
    /// into the app schedule.
    ///
    /// #### NOTE:
    /// The `Sys` generic type is a System type which should implement
    /// a [`ComponentSystem`] as well derive from the `ComponentSystem`
    /// macro, which implements the [`BaseSystem`] trait for the
    /// system.
    ///
    /// @NOTE:
    /// Currently, we allow systems to store a state in the struct, hence
    /// we allow the user to allocate the system themselves.
    /// If this feature turns out to be redundant, we might remove the
    /// need for manual allocation.
    pub fn add_component_system<Sys>(&mut self, system: Sys)
    where
        Sys: BaseSystem + ComponentSystem + 'static,
        <Sys as ComponentSystem>::ComponentType: Component + 'static,
    {
        self.systems.push(Box::new(system));

        // SAFETY:
        // This is the only existing reference to the world in the given scope,
        // hence there is no violation.
        self.world_container
            .get_world_mut()
            .register_component::<<Sys as ComponentSystem>::ComponentType>();
        // self.world.borrow_mut().register_component::<<Sys as ComponentSystem>::ComponentType>();
    }

    // @TODO: Add different types of systems
    // @TODO: Add schedules functionality

    // @TODO: Write documentation
    pub fn start(&mut self) {
        for system in &mut self.systems {
            system.process_start(&mut self.world_container);
        }

        // SAFETY: This is the only exisiting reference to the world in
        //      current scope, hence the safety rules ensure. The mutable
        //      reference to the world is lost before getting the immutable
        //      reference to the world again in the following line.
        self.world_container.get_world_mut().set_active(true);
        while self.world_container.get_world().is_active() {
            self.update();
        }
    }

    /// Calls the update process on all the systems in the App.
    pub fn update(&mut self) {
        for system in &mut self.systems {
            system.process_update(&mut self.world_container);
        }
    }
}
