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
    dependency::SystemDependencies, param::ResourceHandle,
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
    fn initialise_dependencies(&mut self, world: &UnsafeWorldContainer) -> Option<()> {
        self.dependencies = match self.func.extract_dependencies(world) {
            Some(x) => x,
            None => return None,
        };
        Some(())
    }

    fn run(&mut self) {
        let deps = std::mem::replace(&mut self.dependencies, SystemDependencies::new());
        self.func.run(deps);
    }
}

#[derive(Resource)]
struct SampleResource {}

fn test(k: ResourceHandle<SampleResource>) {

}

fn test_function() {
    let sys = System::new(test);
}



// ecs_macros::implement_tuples!(impl_function_into_schedulable, 0, 20, F);





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

    pub fn initialise_dependencies(&mut self, world: &UnsafeWorldContainer) -> Option<()> {
        self.dependencies = match self.func.extract_dependencies(world) {
            Some(deps) => deps,
            None => return None,
        };

        Some(())
    }

    pub fn run(&mut self) {
        let dependencies = std::mem::replace(&mut self.dependencies, SystemDependencies::new());
        self.func.run(dependencies);
    }
}

// @NOTE: Maybe this comment is not important now?
// The issue here is that the marker and func trait are going to be arbituary
// based on the type of system being inserted in the App, so we cannot
// directly store this structure in the App struct.
//
// Hence we need this struct to implement another type, and that type can be dynamically
// dispatched and stored in a box in App. This type makes sense to be system since,
// well it is.
