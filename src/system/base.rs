use std::any::TypeId;

use super::param::SystemParam;
use ecs_macros::implement_tuples;

use crate::{
    ecs_base::ECSBase,
    world::{World, unsafe_world::UnsafeWorldContainer},
};

use super::SystemHolder;

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

                // Create extractor instances for supplied extractor types.
                $(let $param = $param::initialise(world.get_world_mut());)*
                call_inner(self, $($param),*)
            }
        }
    };
}

// MAX 20 parameter limit on a functional system
// @RUST: Can be removed if rust implements variadic templates.
implement_tuples!(impl_system_function, 0, 20, F);
