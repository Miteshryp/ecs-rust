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
use crate::{schedule::Schedulable, world::unsafe_world::UnsafeWorldContainer};

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

    // @TODO: Think about metadata decoupling as follows
    // pub(crate) dependency_metadata: SystemMetadata,
    pub(crate) _marker: PhantomData<Marker>,
}

/// @SAFETY:
/// A system contains dependencies, which due to design
/// contains RwLockGuards to world resources.
/// These RwLockGuards have
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

// @ISSUE: This is gonna result in an internal conflict, which is going to
// hang the system execution (Since it can't resolve it in any iteration)
// @TODO: Find a solution to this issue
//
// fn test(ComponentMutHandle<C1>, Query<C1, C2>) {
//
// }

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
    fn initialise_dependency_metadata(&mut self) -> SystemMetadata {
        let mut new_metadata = SystemMetadata::new();
        self.func.extract_dependency_metadata(&mut new_metadata);
        new_metadata
        // self.func.extract_dependency_metadata(&mut self.dependency_metadata);
    }

    fn initialise_dependencies(&mut self, world: &UnsafeWorldContainer) -> Option<InitError> {
        self.func
            .extract_dependencies(world, &mut self.dependencies)
    }

    fn run(&mut self) {
        // let deps = std::mem::replace(&mut self.dependencies, SystemDependencies::new());
        // self.func.run(deps);

        // @CHECK: dependencies should reset every frame
        self.func.run(&mut self.dependencies);
    }

    fn get_system_param_type_list(&self) -> hashbrown::HashSet<std::any::TypeId> {
        self.dependencies.get_system_param_dependencies()
    }

    // @DONE: Correct this. See TODO in SystemDependency::get_world_resource_dependencies
    // @TODO: Test this implementation.
    // fn check_dependency_conflict(&self, another: &SystemMetadata) -> bool {
    //     self.dependency_metadata.is_resource_clashing(&another)
    // }
    
    // fn get_dependency_metadata(&self) -> SystemMetadata {
    //     self.dependency_metadata.clone()
    // }
}

impl<Marker, Func> System<Marker, Func>
where
    Marker: Send + Sync,
    Func: SystemExecutor<Marker> + SystemExtractor<Marker> + SystemMarker<Marker> + Send + Sync,
{
    pub fn new(system: Func) -> Self {
        Self {
            func: system,
            dependencies: SystemDependencies::new(),
            // dependency_metadata: SystemMetadata::new(),
            _marker: std::marker::PhantomData,
        }
    }
}
