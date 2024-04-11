use crate::world::unsafe_world::UnsafeWorldContainer;

use super::{Schedulable, Schedule};

pub struct SerialSchedule {
    executions: Vec<Box<dyn Schedulable>>,
}

impl Schedulable for SerialSchedule {
    fn initialise_dependencies(&mut self, world: &UnsafeWorldContainer) -> Option<()> {
        todo!()
    }

    fn run(&mut self) {
        todo!()
    }
}

// impl Schedule for SerialSchedule {
//     fn run_schedule(&mut self, world: &UnsafeWorldContainer) {
//         todo!()
//     }
    
//     fn add<S: Schedulable>(item: S) {
//         todo!()
//     }
// }