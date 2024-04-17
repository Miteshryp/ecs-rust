use super::{
    dependency::{SystemDependencies, SystemMetadata},
    param::{InitError, SystemParam},
};
use crate::world::World;
use ecs_macros::implement_tuples;
use log;

use super::System;
use crate::schedule::schedulable::{IntoSchedulable, DependentSystems};

///
/// ### Description
/// 
///  This trait is responsible for creating the [SystemParam] 
/// objects required by the function and storing them in a 
/// [SystemDependency] object, which is to be stored in a
/// [System] object
/// 
/// NOTE:
/// The marker has to exist as a generic parameters in order to distinguish
/// the implementation of [SerialSystemExecutor] for different types of FnMut()

pub trait SystemExtractor<MarkerFunc> {
    fn extract_dependency_metadata(&mut self, deps: &mut SystemMetadata);
    fn extract_dependencies(
        &mut self,
        world: &World,
        deps: &mut SystemDependencies,
    ) -> Option<InitError>;
}

/// @SAFETY:
/// A System Executor interface is thread safe since all the required
/// extractions have already been done by the system, and hence the
/// system is now self dependent for execution.
pub trait SystemExecutor<Marker>: Send {
    fn run(&mut self, dependencies: &mut SystemDependencies);
}



///
/// ### Description
/// 
///  This is an empty trait implementation, which only exists to verify that
/// the function paramters of the system function are compatible with the ECS system
/// 
/// This trait fails to be implemented for functions with parameters which
/// do not match the [SystemParam] trait, hence we can detect if a function
/// is compatible with the system by finding whether it implements this trait 
/// or not
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

            fn after<M>(self, system: impl IntoSchedulable<M>) -> DependentSystems {
                DependentSystems {
                    systems: vec![system.into_schedulable(), self.into_schedulable()]
                }
            }

            fn before<M>(self, system: impl IntoSchedulable<M>) -> DependentSystems {
                DependentSystems {
                    systems: vec![self.into_schedulable(), system.into_schedulable()]
                }
            }
        }



        /// Implementation of the [SystemMarker] type on all possible functions
        /// declared in the application
        #[allow(non_snake_case)]
        impl<Func, $($param: SystemParam + 'static),*> SystemMarker<fn ($($param),*) -> ()> for Func
        where
            Func: Send + Sync + 'static + FnMut($($param),*) -> ()
        {}



        
        #[allow(non_snake_case)]
        impl<Func, $($param: SystemParam + 'static),*> SystemExtractor<fn ($($param),*) -> ()> for Func
        where
            Func: Send + Sync + 'static + FnMut($($param),*) -> ()
        {
            // See description in [SystemExtractor]
            fn extract_dependency_metadata(&mut self, dependencies: &mut SystemMetadata) {
                $(
                    dependencies.push_dependency_metadata::<$param>();
                )*
            }

            // See description in [SystemExtractor]
            fn extract_dependencies(&mut self, world: &World, dependencies: &mut SystemDependencies) -> Option<InitError> {
                // Create extractor instances for supplied extractor types.
                $(

                    // For a DAG based parallel system, we actually ensure that
                    // this function never fails, since the predicate of a DAG
                    // based parallel execution graph is that no two nodes with
                    // conflicts will execute in parallel.

                    // InitError, means that the requested resoruce does not 
                    // exist in the world, in which case we want to stop the
                    // system from executing
                    let $param = match $param::initialise(world) {
                        (None, Some(x)) => x,

                        
                        // In a DAG parallel graph, this should never happen
                        // In a serial schedule, this can NEVER happen
                        (None, None) => {
                            let err_str = "System faced contention. Will retry in next iteration";
                            log::error!(
                                "{err_str}"
                            );
                            panic!("{err_str}");
                        },

                        // If any one of the extractor acquisition fails, we
                        // cleanup all the extractors which were successful by
                        // leaving the function.
                        (Some(x), None) => return Some(x),

                        _ => panic!("Invalid initialisation result")
                    };

                    // Pushing the dependency to keep a record
                    // This might result in panic if the system function is trying to access
                    // a world resource in a conflicting way (mut-mut, mut-read)
                    dependencies.push_dependency::<$param>($param);
                )*

                None
            }
        }


        /// This trait is responsible for running the underlying function
        /// based on a set of owned resources which are acquired using [SystemDependencies]
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
                    // Fetching and removing the system param resource
                    // dependency from the passed dependency.
                    let $param = dependencies.pop_dependency::<$param>();
                )*

                call_inner(self, $(*$param),*);
            }
        }
    };
}

// MAX 20 parameter limit on a functional system
// @RUST: Can be removed if rust implements variadic templates.
implement_tuples!(impl_system_function, 0, 20, F);
