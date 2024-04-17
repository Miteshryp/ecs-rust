use std::{
    cell::Cell,
    sync::mpsc::Sender,
};

use crate::world::World;

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
pub struct UnsafeWorldContainer {
    pub(crate) world: Cell<World>,
}

impl UnsafeWorldContainer {
    // pub(crate) fn new(command_sender: Sender<CommandFunction>) -> Self {
    pub(crate) fn new(command_sender: Sender<Box<dyn FnMut(&mut World) -> ()>>) -> Self {
        Self {
            world: Cell::new(World::new(command_sender)),
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