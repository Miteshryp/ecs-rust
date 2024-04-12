mod query;
mod event;
mod resource;
mod command_buffer;

pub use query::*;
pub use event::*;
pub use resource::*;
pub use command_buffer::*;


use std::{
    any::{Any, TypeId},
};

use crate::{
    ecs_base::ECSBase, world::World
};


/// 
/// ### Description
/// 
/// This is a trait defined on all types which can be taken as an input
/// dependency in a system. This trait type is responsible for recognizing
/// functions as system and coordinating system based trait implementations
/// on the given function type.
/// 
/// @SAFETY:
/// Any system param is going to be allocated in a schedule, and 
/// run across different threads. Also, any given SystemParam 
/// reference will only be sent to a single thread, hence removing
/// any possibility of contention of resources allocated in the SystemParam,
/// 
/// With the above assertions, any SystemParam is safe to
/// be Send or Referenced across a thread boundary.
/// 
/// 
/// 
pub trait SystemParam: ECSBase {
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




