use super::{dependency::SystemDependencies, param::SystemParam};
use ecs_macros::implement_tuples;
use log;
use crate::{
    world::{unsafe_world::UnsafeWorldContainer},
};

use super::System;
use crate::schedule::Schedulable;
use crate::schedule::IntoSchedulable;

// pub trait System {
//     fn acquire_dependencies(&mut self, world_container: &UnsafeWorldContainer);
//     fn run_system(&mut self);
//     // fn run_system(&mut self, world_container: &UnsafeWorldContainer);
// }

// @DONE: Update SerialSystemExecutor use case in the SystemHolder 
//          struct. SystemHolder needs to run the system, 
//          but it can no longer run it since both interfaces
//          need different parameters now.
//  
//          Changed architecture for trait implementation on functions
//          and Scheduling system

// /// ### Definition
// /// A trait which is implemented on all desired function types.
// /// The implementation is carried by the [`implement_system_function`](Self::implementation_system_function)
// /// macro
// ///
// /// NOTE:
// /// The marker has to exist as a generic parameters in order to distinguish
// /// the implementation of [SerialSystemExecutor] for different types of FnMut()
// pub trait SerialSystemExecutor<Marker> {
//     fn run(&mut self, world: &UnsafeWorldContainer);
// }

// pub trait ParallelSystemExecutor<Marker> {
//     fn run(&mut self, world: &UnsafeWorldContainer, atomic_lock: &std::sync::Mutex<u32>);
// }

pub trait SystemExtractor<MarkerFunc> {
    fn extract_dependencies(&mut self, world: &UnsafeWorldContainer) -> Option<SystemDependencies>; 
}


/// @SAFETY:
/// A System Executor interface is thread safe since all the required
/// extractions have already been done by the system, and hence the 
/// system no is now self dependent for execution.
pub trait SystemExecutor<Marker>: Send {
    fn run(&mut self, dependencies: SystemDependencies);
}

pub trait SystemMarker<Marker>: Send + Sync {}

macro_rules! impl_system_function {
    ($($param: ident),*) => {

        #[allow(non_snake_case)]
        impl<Func, $($param: SystemParam + 'static),*> IntoSchedulable<fn ($($param),*) -> ()> for Func
        where 
            Func: Send + Sync + 'static + FnMut($($param),*) -> () 
        {
            fn into_schedulable(self) -> Box<dyn Schedulable> {
                let system = System::new(self);
                Box::new(system)
            }
        }


        impl<Func, $($param: SystemParam + 'static),*> SystemMarker<fn ($($param),*) -> ()> for Func
        where
            Func: Send + Sync + 'static + FnMut($($param),*) -> ()
        {}

        #[allow(non_snake_case)]
        impl<Func, $($param: SystemParam + 'static),*> SystemExtractor<fn ($($param),*) -> ()> for Func
        where
            Func: Send + Sync + 'static + FnMut($($param),*) -> ()
        {
            fn extract_dependencies(&mut self, world: &UnsafeWorldContainer) -> Option<SystemDependencies> {
                let mut dependencies = SystemDependencies::new();
                // Create extractor instances for supplied extractor types.
                $(
                    let $param = match $param::initialise(world.get_world_mut()) {
                        Some(x) => x,

                        // If any one of the extractor acquisition fails, we 
                        // cleanup all the extractors which were successful by 
                        // leaving the function. 
                        None => {
                            log::error!(
                                "System failed to initialise extractors in a serial schedule"
                            );
                            return None
                        }
                    };
                    dependencies.push_dependency::<$param>($param);
                )*

                Some(dependencies)
            }
        }


        #[allow(non_snake_case)]
        impl<Func, $($param: SystemParam + 'static),*> SystemExecutor<fn ($($param),*) -> ()> for Func
        where 
            Func: Send + Sync + 'static + FnMut($($param),*) -> () 
        {
            fn run(&mut self, mut dependencies: SystemDependencies) {
                fn call_inner<$($param),*>(
                    mut f: impl FnMut($($param),*) -> (),
                    $($param: $param),*
                ) {
                    f($($param),*)
                }

                $(
                    let mut $param = dependencies.pop_dependency::<$param>();
                )*

                call_inner(self, $(*$param),*);
            }
        }
    };
}

// MAX 20 parameter limit on a functional system
// @RUST: Can be removed if rust implements variadic templates.
implement_tuples!(impl_system_function, 0, 20, F);
