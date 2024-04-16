use crate::world::unsafe_world::UnsafeWorldContainer;

use super::{ScheduleHolderFrequency, Schedule, schedulable::{Schedulable}};

/// @SOLVED: Maybe to compensate the Schedule holding Schedule issue,
/// we can implement a parent trait to Schedule which can be implemented
/// by schedule holder, and then through type checks inside a schedule,
/// if we find a schedule holder, we extract schedules from them and execute them
/// in parallel with ours.
/// 
/// SOLVED Using SystemHolders
///
pub struct ScheduleHolder {
    executions: Vec<Box<dyn Schedule>>,
    ticks: u32,
    frequency: ScheduleHolderFrequency
}

impl ScheduleHolder {
    pub fn new(frequency: ScheduleHolderFrequency) -> Self {
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
            ScheduleHolderFrequency::Once => 1,
            _ => self.ticks % self.frequency as u32
        };

    }
}