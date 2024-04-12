pub mod base;
pub mod dependency;
pub mod param;

use std::marker::PhantomData;

use crate::resource::Resource;
use ecs_macros::Resource;
use crate::ECSBase;
use std::any::Any;
use crate::system::param::SystemParam;


use self::{
    base::{SystemExecutor, SystemExtractor, SystemMarker},
    dependency::SystemDependencies, param::{InitError, ResourceHandle},
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
{}
unsafe impl<Marker, Func> Send for System<Marker, Func>
where
    Marker: Send + Sync,
    Func: SystemExecutor<Marker> + SystemExtractor<Marker> + SystemMarker<Marker> + Send + Sync,
{}



impl<Marker, Func> Schedulable for System<Marker, Func>
where
    Marker: Send + Sync,
    Func: SystemExecutor<Marker> + SystemExtractor<Marker> + SystemMarker<Marker> + Send + Sync,
{
    fn initialise_dependencies(&mut self, world: &UnsafeWorldContainer) -> (Option<InitError>, Option<()>) {
        self.dependencies = match self.func.extract_dependencies(world) {
            (None, Some(x)) => x,
            default => return (default.0, None),
        };
        (None, Some(()))
    }

    fn run(&mut self) {
        let deps = std::mem::replace(&mut self.dependencies, SystemDependencies::new());
        self.func.run(deps);
    }
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
            _marker: std::marker::PhantomData,
        }
    }
}
