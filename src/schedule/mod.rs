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

use self::schedulable::{DependentSystems, Schedulable};


pub enum FlowFrequency {
    Once = 0,
    Always = 1,
    Alternate = 2, // runs every 2 executions
}

impl Clone for FlowFrequency {
    fn clone(&self) -> Self {
        match self {
            Self::Once => Self::Once,
            Self::Always => Self::Always,
            Self::Alternate => Self::Alternate,
        }
    }
}
impl Copy for FlowFrequency {}

pub trait Schedule {
    fn run_schedule(&mut self, world: &UnsafeWorldContainer);
    fn add_boxed(&mut self, item: Box<dyn Schedulable>);
    fn add_ordered(&mut self, systems: DependentSystems);
}

