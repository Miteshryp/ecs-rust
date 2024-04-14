use std::{
    any::TypeId,
    borrow::BorrowMut,
    sync::{Arc, Mutex},
    thread, vec,
};

use tokio::sync::RwLock;

use crate::{
    system::{dependency::SystemDependencies, param::SystemParam},
    world::unsafe_world::UnsafeWorldContainer,
};

use super::{FlowFrequency, Schedulable, Schedule, DAG::DependencyGraph};

pub struct ParallelSchedule {
    // systems: Vec<Arc<RwLock<Box<dyn Schedulable>>>>,
    dependency_graph: DependencyGraph
}

impl ParallelSchedule {
    pub fn new() -> Self {
        // Self { systems: vec![] }
        Self { dependency_graph: DependencyGraph::new() }
    }
}

impl Schedule for ParallelSchedule {
    /// @TODO: Change the execution logic to be based on a dependency graph
    ///      This acyclic graph needs to determine conflicting systems based on
    ///      their dependency conflicts and then construct the graph accordingly
    ///
    ///      Once this is constructed, we can start executing nodes with in-degree = 0.
    ///
    /// @TODO: Determine how the graph has to be constructed between 2 nodes with conflict,
    ///     i.e. the criteria for determining the direction of the edge.
    




    fn run_schedule(&mut self, world: &crate::world::unsafe_world::UnsafeWorldContainer) {
        self.dependency_graph.execute_system_graph(world);
    }


    
    // fn run_schedule(&mut self, world: &crate::world::unsafe_world::UnsafeWorldContainer) {
        // let mut remaining_system: Vec<usize> = (0..self.systems.len()).into_iter().collect();

        // while !remaining_system.is_empty() && self.ticks == 0 {


        // while !remaining_system.is_empty() {
        //     let mut executables = vec![];
        //     let failed_extractions: Vec<usize> = remaining_system.iter().filter_map(|index| {
        //         // 1. Extraction process
        //         let mut system = match self.systems.get_mut(*index) {
        //             Some(lock_arc) => lock_arc.clone().try_write_owned().unwrap(),
        //             None => {
        //                 let err_str = r#"
        //                     Failed to get hold of the system while running inside a parallel schedule.
        //                 "#;
        //                 panic!("{err_str}");
        //             },
        //         };

        //         match system.initialise_dependencies(world) {
        //             // (None, Some(_)) => {
        //             //     // 2. Running process
        //             //     let join = std::thread::spawn(move || {
        //             //         system.run();
        //             //     });
        //             //     executables.push(join);

        //             //     None
        //             // },
        //             // (None, None) => Some(index),
        //             // (Some(x), None) => None,

        //             None => {
        //                 // 2. Running process
        //                 let join = std::thread::spawn(move || {
        //                     system.run();
        //                 });
        //                 executables.push(join);

        //                 None
        //             }
        //             Some(_) => {
        //                 None
        //             },
                    
        //             // @CLEANUP:
        //             _ => panic!("Invalid initialisation result")
        //         }
        //     }).cloned().collect();

        //     // Joining threads before proceeding to the next batch
        //     for e in executables {
        //         let _ = e.join().unwrap();
        //     }

        //     // @TODO: Improve the response system to get information on
        //     //      the type of initialisation failure to automatically
        //     //      get out of such situations by dropping the system
        //     //      entirely.
        //     if failed_extractions.len() == remaining_system.len() {
        //         let err_str = r#"
        //             The parallel system ran into unresolvable state.
        //             Did you create functions with resources or components which do not exist?
        //         "#;
        //         panic!("{err_str}")
        //     }

        //     remaining_system = failed_extractions;
        // }

        // self.ticks += 1;

        // self.ticks = match self.frequency {
        //     ScheduleFrequency::Once => 0,
        //     _ => self.ticks % (self.frequency as u32)

        // };
    // }

    // @TODO: Document
    fn add_boxed(&mut self, item: Box<dyn Schedulable>) {
        // self.systems.push(Arc::new(RwLock::new(item)))
        self.dependency_graph.add_boxed_system(item);
    }

    // fn set_schedule_frequency(&mut self, freq: super::ScheduleFrequency) {
    //     self.frequency = freq;
    // }
}


