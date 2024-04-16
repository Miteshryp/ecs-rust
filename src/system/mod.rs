pub mod base;
pub mod dependency;
pub mod param;

use std::marker::PhantomData;

use crate::resource::Resource;
use crate::system::param::SystemParam;
use crate::ECSBase;
use ecs_macros::Resource;
use std::any::Any;

use self::{
    base::{SystemExecutor, SystemExtractor, SystemMarker},
    dependency::{SystemDependencies, SystemMetadata},
    param::{InitError, ResourceHandle},
};
use crate::{schedule::schedulable::Schedulable, world::unsafe_world::UnsafeWorldContainer};

/// ### Description
///
/// Structure to hold a serial system function.
/// This is a function which is parker in a Serial Schedule
/// for execution
pub struct System<Marker, Func>
where
    Marker: Send + Sync,
    Func: SystemExecutor<Marker> + SystemExtractor<Marker> + SystemMarker<Marker> + Send + Sync,
{
    pub(crate) func: Func,
    pub(crate) dependencies: SystemDependencies,
    pub(crate) _marker: PhantomData<Marker>,
}

/// @TODO: Write @SAFETY
unsafe impl<Marker, Func> Sync for System<Marker, Func>
where
    Marker: Send + Sync,
    Func: SystemExecutor<Marker> + SystemExtractor<Marker> + SystemMarker<Marker> + Send + Sync,
{
}
unsafe impl<Marker, Func> Send for System<Marker, Func>
where
    Marker: Send + Sync,
    Func: SystemExecutor<Marker> + SystemExtractor<Marker> + SystemMarker<Marker> + Send + Sync,
{
}

///
/// @NOTE: Schedulable is only ever going to be implemented
///         for a system as of now (Apr 14, 2024). So while
///         reading this code, consider this as an interface
///         for a system, which contains the function, metadata, etc
impl<Marker, Func> Schedulable for System<Marker, Func>
where
    Marker: Send + Sync,
    Func: SystemExecutor<Marker> + SystemExtractor<Marker> + SystemMarker<Marker> + Send + Sync,
{
    /// For description, see [Schedulable::initialise_dependency_metadata]
    fn initialise_dependency_metadata(&mut self) -> SystemMetadata {
        let mut new_metadata = SystemMetadata::new();
        self.func.extract_dependency_metadata(&mut new_metadata);
        new_metadata
    }


    /// For description, see [Schedulable::initialise_dependencies]
    fn initialise_dependencies(&mut self, world: &UnsafeWorldContainer) -> Option<InitError> {
        self.func
            .extract_dependencies(world, &mut self.dependencies)
    }


    /// For description, see [Schedulable::run]
    fn run(&mut self) {

        // Running the system.
        //
        // This clears out the acquired locks stored in the
        // [`dependencies`](crate::system::System::dependencies)
        self.func.run(&mut self.dependencies);
    }
}


// Interface for System
impl<Marker, Func> System<Marker, Func>
where
    Marker: Send + Sync,
    Func: SystemExecutor<Marker> + SystemExtractor<Marker> + SystemMarker<Marker> + Send + Sync,
{

    /// 
    /// ### Description 
    /// 
    /// Creates a new System wrapper object responsible
    /// for storing a system function along with data regarding
    /// its dependencies
    pub fn new(system: Func) -> Self {
        Self {
            func: system,
            dependencies: SystemDependencies::new(),
            _marker: std::marker::PhantomData,
        }
    }
}
