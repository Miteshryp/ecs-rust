pub mod DAG;
pub mod holder;
pub mod parallel;
pub mod schedulable;

use std::error::Error;

use crate::{
    system::{
        dependency::{SystemDependencies, SystemMetadata},
        param::InitError,
    },
    world::unsafe_world::UnsafeWorldContainer,
};

use self::schedulable::{DependentSystems, IntoSchedulable, Schedulable};


pub enum ScheduleHolderFrequency {
    Once = 0,
    Always = 1,
    Alternate = 2, // runs every 2 executions
}

impl Clone for ScheduleHolderFrequency {
    fn clone(&self) -> Self {
        match self {
            Self::Once => Self::Once,
            Self::Always => Self::Always,
            Self::Alternate => Self::Alternate,
        }
    }
}
impl Copy for ScheduleHolderFrequency {}

pub trait Schedule {
    fn run_schedule(&mut self, world: &UnsafeWorldContainer);

    // fn add<Marker>(&mut self, func: impl IntoSchedulable<Marker>);
    fn add<Marker>(&mut self, func: impl IntoSchedulable<Marker>) where Self: Sized;
    fn add_boxed(&mut self, item: Box<dyn Schedulable>);
    fn add_ordered(&mut self, systems: DependentSystems);
}

