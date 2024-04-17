mod query;
mod event;
mod resource;
mod command_buffer;
mod component_collection;
mod cross_components_collection;

pub use query::*;
pub use event::*;
pub use resource::*;
pub use command_buffer::*;
pub use component_collection::*;
pub use cross_components_collection::*;


use std::{
    any::{TypeId}, error::Error, path::Display,
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
    ///
    /// ### Description
    /// 
    /// Interface to setup world extractor parameter to 
    /// fetch World state for functional systems.
    /// 
    /// If the initialisation fails, it must return None to ensure
    /// that the scheduler knows not to executed the system, 
    /// and the other dependencies required by the system are
    /// freed by the world system to avoid resource starvation
    /// in a parallel setting.
    /// 
    /// **NOTE:**
    /// Running a system with failed dependency initialisation will
    /// result in a crash.
    /// 
    /// Passing an InitError will result in the system execution being skipped
    /// 
    /// @NOTE: We get a mutable pointer to the world as a input to the initialise method
    ///         This is to facilitate RwLockGuard acquisition from the world for Resource
    ///         handles. &mut World makes rust believe the returned guard does not have
    ///         lifetime which lives long enough, so we fool the borrow checker this way
    ///         to make it think that the reference is static.
    fn initialise(world: &World) -> (Option<InitError>, Option<Self>) where Self: Sized;

    ///
    /// ### Description
    /// 
    /// Returns a vec of [TypeId]s of the world-based resources whose lock 
    /// this parameter intends to take from the world
    fn get_resource_access_type() -> hashbrown::HashSet<TypeId>; 

    ///
    /// ### Description
    /// 
    /// Indicated whether or not the resource access method of this 
    /// [SystemParam] type is mutable in nature or not.
    fn is_resource_access_mut() -> bool; 
}




