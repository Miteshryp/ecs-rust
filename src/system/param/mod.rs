mod query;
mod event;
mod resource;
mod command_buffer;

pub use query::*;
pub use event::*;
pub use resource::*;
pub use command_buffer::*;


use std::{
    any::{Any, TypeId}, error::Error, path::Display,
};

use crate::{
    ecs_base::ECSBase, world::World
};


#[derive(Debug)]
pub struct InitError {}
impl Error for InitError {
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
    
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
    
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Initialisation of a requested System Param failed")
    }
}


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
    /// Passing an InitError will result in the system execution being skipped
    /// 
    /// @NOTE: We need the mutable pointer to be passed in the initialise function.
    ///     See [ResourceHandle] or [MutResourceHandle] docs for more info
    
    fn initialise(world: *mut World) -> (Option<InitError>, Option<Self>) where Self: Sized;

    // Typeid of the resources whose lock it takes from the world
    fn get_resource_access_type() -> hashbrown::HashSet<TypeId>; 

    // Whether or not the param is mutable in nature
    fn is_resource_access_mut() -> bool; 
}




