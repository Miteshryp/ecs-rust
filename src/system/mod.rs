pub mod base;
pub mod param;

use std::marker::PhantomData;

use crate::world::unsafe_world::UnsafeWorldContainer;
use self::base::{System, SystemFunction};


/// Structure to hold an arbituary type of system function
pub struct SystemHolder<Marker, Func>
where
    Func: SystemFunction<Marker>,
{
    pub(crate) func: Func,
    pub(crate) _marker: PhantomData<Marker>,
}

impl<Marker, Func> SystemHolder<Marker, Func> where Func: SystemFunction<Marker>{
    pub fn new(system: Func) -> Self 
    {
        Self {
            func: system,
            _marker: std::marker::PhantomData
        }
    }
}

// The issue here is that the marker and func trait are going to be arbituary
// based on the type of system being inserted in the App, so we cannot
// directly store this structure in the App struct.
//
// Hence we need this struct to implement another type, and that type can be dynamically
// dispatched and stored in a box in App. This type makes sense to be system since,
// well it is.



impl<Marker, Func> System for SystemHolder<Marker, Func>
where
    Func: SystemFunction<Marker>,
{
    fn run_system(&mut self, world_container: &UnsafeWorldContainer) {
        self.func.run(world_container)
    }
}
