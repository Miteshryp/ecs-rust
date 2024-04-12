use crate::world::unsafe_world::UnsafeWorldContainer;

pub mod holder;
pub mod parallel;

pub enum FlowFrequency {
    Once = 0,
    Always = 1,
    Alternate = 2// runs every 2 executions
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

    // fn set_schedule_frequency(&mut self, freq: ScheduleFrequency);
}



/// @SAFETY:
/// Any schedulable element is thread safe only after its dependencies
/// have been initialised and stored in the struct which is to be transferred
/// across thread boundaries.
/// This property must be maintained by all [Schedule]s
// pub trait Schedulable: Sync {
pub trait Schedulable: Send + Sync {
    fn initialise_dependencies(&mut self, world: &UnsafeWorldContainer) -> Option<()>;
    fn run(&mut self);
}



pub trait IntoSchedulable<Marker> {
    type Output: Schedulable + 'static;
    fn into_schedulable(self) -> Box<Self::Output>;
}