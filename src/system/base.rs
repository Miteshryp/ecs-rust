use super::param::SystemParam;
use ecs_macros::implement_tuples;

use crate::{
    world::{unsafe_world::UnsafeWorldContainer},
};

pub trait System {
    fn run_system(&mut self, world_container: &UnsafeWorldContainer);
}

/// ### Definition
/// A trait which is implemented on all desired function types.
/// The implementation is carried by the [`implement_system_function`](Self::implementation_system_function)
/// macro
///
/// NOTE:
/// The marker has to exist as a generic parameters in order to distinguish
/// the implementation of [SystemFunction] for different types of FnMut()
pub trait SystemFunction<Marker> {
    fn run(&mut self, world: &UnsafeWorldContainer);
}

macro_rules! impl_system_function {
    ($($param: ident),*) => {

        #[allow(non_snake_case)]
        impl<Func, $($param: SystemParam),*> SystemFunction<fn ($($param),*) -> ()> for Func
        where
            Func: Send + Sync + 'static + FnMut($($param),*) -> ()
        {
            fn run(&mut self, world: &UnsafeWorldContainer) {
                fn call_inner<$($param),*>(
                    mut f: impl FnMut($($param),*) -> (),
                    $($param: $param),*
                ) {
                    f($($param),*)
                }

                // Acquire acquisition atomic lock to initiate atomic acquisition
                // of extractors
                let atomic_lock = (*world.get_world_mut()).acquire_acquisition_lock();

                // Create extractor instances for supplied extractor types.
                $(
                    let $param = match $param::initialise(world.get_world_mut()) {
                        Some(x) => x,

                        // If any one of the extractor acquisition fails, we 
                        // cleanup all the extractors which were successful by 
                        // leaving the function. 
                        // We also release the atomic lock to allow other 
                        // systems to get extractors.
                        None => {
                            (world.get_world_mut()).return_acquisition_lock(atomic_lock);
                            return;
                        }
                    };
                )*

                // All extractors initialised successfully. 
                // Returning the lock for other systems to access the world for extraction.
                world.get_world_mut().return_acquisition_lock(atomic_lock);

                // Running the system
                call_inner(self, $($param),*)
            }
        }
    };
}

// MAX 20 parameter limit on a functional system
// @RUST: Can be removed if rust implements variadic templates.
implement_tuples!(impl_system_function, 0, 20, F);
