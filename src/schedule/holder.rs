use crate::world::unsafe_world::UnsafeWorldContainer;

use super::{Schedulable, Schedule, FlowFrequency};

/// @POSSIBLE SOLUTION: Maybe to compensate the Schedule holding Schedule issue,
/// we can implement a parent trait to Schedule which can be implemented
/// by schedule holder, and then through type checks inside a schedule,
/// if we find a schedule holder, we extract schedules from them and execute them
/// in parallel with ours.
///
pub struct ScheduleHolder {
    executions: Vec<Box<dyn Schedule>>,
    ticks: u32,
    frequency: FlowFrequency
}

impl ScheduleHolder {
    pub fn new(frequency: FlowFrequency) -> Self {
        Self { executions: vec![], ticks: 0, frequency }
    }

    pub fn add(&mut self, s: Box<dyn Schedule>) {
        self.executions.push(s);
    }

    pub fn run_all(&mut self, world: &UnsafeWorldContainer) {
        if self.ticks == 0 {
            for schedule in &mut self.executions {
                schedule.run_schedule(world);
            }
        }

        self.ticks += 1;
        self.ticks = match self.frequency {
            FlowFrequency::Once => 1,
            _ => self.ticks % self.frequency as u32
        };

    }
}

// impl Schedule for ScheduleHolder {
//     fn run_schedule(&mut self, world: &UnsafeWorldContainer) {
//         for schedule in &mut self.executions {
//             schedule.run_schedule(world);
//         }
//     }

//     fn add_boxed(&mut self, item: Box<dyn Schedulable>) {
//         self.executions.push(item);
//     }

//     fn set_schedule_frequency(&mut self, freq: super::ScheduleFrequency) {
//         todo!()
//     }
// }
