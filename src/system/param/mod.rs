mod component;
mod event;
mod resource;

pub use component::*;
pub use event::*;
pub use resource::*;


use std::{
    any::TypeId,
    marker::PhantomData,
    slice::Iter,
    sync::mpsc::{self, Sender},
};

use crate::{
    component::resource::Resource,
    events::{event_manager::EventManager, Event},
    world::World,
};


/// Trait implemented by all types which can be passed into a functional
/// system as a system param. These are types which associate the requested
/// world state type to the appropriate world object based on the metadata
/// provided to the type
pub trait SystemParam {
    fn type_id() -> TypeId
    where
        Self: Sized + 'static,
    {
        TypeId::of::<Self>()
    }

    // We pass a mut reference of the World to ensure that if the initialisation
    // requires a pointer access with safety guarentees, that may be facilitated.
    // (We cannot get a pointer through &World)
    fn initialise(world: &mut World) -> Self;
}



/// ## Description
/// Extract the specified type of Components from the world to query.
/// The struct supplies the component's along with their [`Entity`](crate::entity::Entity)
/// if and only if
///     - All components specified are attached to the entity_id.
///     - All components are free for use and not being held by another system for use. (NOTE: This has a high chance of resulting in a deadlock through mutual starvation)
///
pub struct Query {
    // SAFETY: @TODO
    world: *mut World,
}
