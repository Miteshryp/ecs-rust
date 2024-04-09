mod query;
mod event;
mod resource;

pub use query::*;
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
    world::{unsafe_world::UnsafeWorldContainer, World},
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

    /// Interface to setup world extractor parameter to 
    /// fetch World state for functional systems.
    /// If the initialisation fails, it must return None to ensure
    /// that the system does not execute, and the other dependencies required
    /// by the system are freed by the world system to avoid resource starvation
    /// in a parallel setting.
    /// 
    /// @NOTE: We need the mutable pointer to be passed in the initialise function.
    ///     See [ResourceHandle] or [MutResourceHandle] docs for more info
     
    fn initialise(world: *mut World) -> Option<Self> where Self: Sized;
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
