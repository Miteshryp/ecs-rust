use std::{any::TypeId, borrow::BorrowMut, sync::{Arc, Mutex}, thread, vec};

use tokio::sync::RwLock;

use crate::system::{dependency::SystemDependencies, param::SystemParam};

use super::{Schedulable, Schedule};


// atomic lock ->

pub struct ParallelSchedule {
    atomic_world_lock: Mutex<u32>,
    systems: Vec<Arc<RwLock<Box<dyn Schedulable>>> >,
}

impl ParallelSchedule {
    pub fn new() -> Self {
        Self {
            atomic_world_lock: Mutex::new(0),
            systems: vec![]
        }
    }
}

impl Schedule for ParallelSchedule {
    fn run_schedule(&mut self, world: &crate::world::unsafe_world::UnsafeWorldContainer) {
        let mut remaining_system: Vec<usize> = (0..self.systems.len()).into_iter().collect();
        
        while !remaining_system.is_empty() {

            let mut executables = vec![];
            let failed_extractions = remaining_system.iter().filter_map(|index| {
                // 1. Extraction process
                let mut system = match self.systems.get_mut(*index) {
                    Some(lock_arc) => lock_arc.clone().try_write_owned().unwrap(),
                    None => {
                        let err_str = r#"
                            Failed to get hold of the system while running inside a parallel schedule.
                        "#;
                        panic!("{err_str}");
                    },
                };

                match system.initialise_dependencies(world) {
                    Some(_) => {
                        // 2. Running process
                        let join = std::thread::spawn(move || {
                            system.run();
                        });
                        executables.push(join);

                        None
                    },
                    None => Some(index)
                }
            }).cloned().collect();

            // Joining threads before proceeding to the next batch
            for e in executables {
                let _ = e.join().unwrap();
            }

            remaining_system = failed_extractions;
        }


    }
    
    fn add<S: Schedulable + 'static >(&mut self, item: S) {
        self.systems.push(Arc::new(RwLock::new(Box::new(item))));
    }

    fn add_boxed(&mut self, item: Box<dyn Schedulable>) {
        self.systems.push(Arc::new(RwLock::new(item)))
    }
}