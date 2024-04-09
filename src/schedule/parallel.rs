use std::sync::Mutex;

use super::Schedule;

pub struct ParallelSchedule {
    atomic_world_lock: Mutex<u32>,
    
    // graph of systems
}

impl Schedule for ParallelSchedule {}