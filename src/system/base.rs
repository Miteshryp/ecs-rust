use super::{dependency::{SystemDependencies, SystemMetadata}, param::{InitError, SystemParam}};
use ecs_macros::implement_tuples;
use log;
use crate::{
    world::{unsafe_world::UnsafeWorldContainer},
};

use super::System;
use crate::schedule::Schedulable;
use crate::schedule::IntoSchedulable;


// @TODO Document
// /// NOTE:
// /// The marker has to exist as a generic parameters in order to distinguish
// /// the implementation of [SerialSystemExecutor] for different types of FnMut()

pub trait SystemExtractor<MarkerFunc> {
    fn extract_dependency_metadata(&mut self, deps: &mut SystemMetadata);
    // fn extract_dependency_metadata(&mut self, deps: &mut SystemDependencies);
    // fn extract_dependencies(&mut self, world: &UnsafeWorldContainer, deps: &mut SystemDependencies) -> (Option<InitError>, Option<SystemDependencies>); 
    fn extract_dependencies(&mut self, world: &UnsafeWorldContainer, deps: &mut SystemDependencies) -> Option<InitError>; 
}


/// @SAFETY:
/// A System Executor interface is thread safe since all the required
/// extractions have already been done by the system, and hence the 
/// system no is now self dependent for execution.
pub trait SystemExecutor<Marker>: Send {
    fn run(&mut self, dependencies: &mut SystemDependencies);
}

pub trait SystemMarker<Marker>: Send + Sync {}

macro_rules! impl_system_function {
    ($($param: ident),*) => {

        #[allow(non_snake_case)]
        impl<Func, $($param: SystemParam + 'static),*> IntoSchedulable<fn ($($param),*) -> ()> for Func
        where 
            Func: Send + Sync + 'static + FnMut($($param),*) -> () 
        {
            type Output = System<fn ($($param),*) -> (), Func>;
            fn into_schedulable(self) -> Box<Self::Output> {
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
            fn extract_dependency_metadata(&mut self, dependencies: &mut SystemMetadata) {
                $(
                    dependencies.push_dependency_metadata::<$param>();
                )*
            }

            // fn extract_dependency_metadata(&mut self, dependencies: &mut SystemDependencies) {
            //     $(
            //         dependencies.push_dependency_metadata::<$param>();
            //     )*
            // }

            // fn extract_dependencies(&mut self, world: &UnsafeWorldConainer, dependencies: &mut SystemDependencies) -> (Option<InitError>, Option<SystemDependencies>) {
            fn extract_dependencies(&mut self, world: &UnsafeWorldContainer, dependencies: &mut SystemDependencies) -> Option<InitError> {
                // let mut dependencies = SystemDependencies::new();
                // Create extractor instances for supplied extractor types.
                $(

                    // For a DAG based parallel system, we actually ensure that
                    // this function never fails, since the predicate of a DAG 
                    // based parallel execution graph is that no two nodes with
                    // conflicts will execute in parallel.


                    // @TODO: Remove the return type of None for param if there is a 
                    //          conflict, since in the new DAG parallel system, we cannot
                    //          really have a conflict
                    //        A Serial schedule has no chance of a conflict.
                    //        So the only possible error is InitError, which means that the
                    //          requested resoruce does not exist in the world, in which case
                    //          we want to stop the system from executing
                    let $param = match $param::initialise(world.get_world_mut()) {
                        (None, Some(x)) => x,

                        // If any one of the extractor acquisition fails, we 
                        // cleanup all the extractors which were successful by 
                        // leaving the function. 

                        // NEW CONDITION: In a DAG parallel graph, this should never happen
                        //                In a serial schedule, this can NEVER happen
                        (None, None) => {
                            let err_str = "System faced contention. Will retry in next iteration";
                            log::error!(
                                "{err_str}"
                                // "System failed to initialise extractors in a serial schedule"
                            );
                            // return (None, None)
                            panic!("{err_str}");
                        },

                        // (Some(x), None) => return (Some(x), None),
                        (Some(x), None) => return Some(x),

                        _ => panic!("Invalid initialisation result")
                    };

                    // Pushing the dependency to keep a record
                    // This might result in panic if the system function is trying to access
                    // a world resource in a conflicting way (mut-mut, mut-read)
                    dependencies.push_dependency::<$param>($param);
                )*

                // (None, Some(dependencies))
                None
            }
        }


        #[allow(non_snake_case)]
        impl<Func, $($param: SystemParam + 'static),*> SystemExecutor<fn ($($param),*) -> ()> for Func
        where 
            Func: Send + Sync + 'static + FnMut($($param),*) -> () 
        {
            fn run(&mut self, dependencies: &mut SystemDependencies) {
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
