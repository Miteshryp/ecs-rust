use crate::world::unsafe_world::UnsafeWorldContainer;

pub mod serial;
pub mod parallel;


pub trait Schedule {
    fn run_schedule(&mut self, world: &UnsafeWorldContainer);
    fn add<S: Schedulable + 'static>(&mut self, item: S);
    fn add_boxed(&mut self, item: Box<dyn Schedulable>);
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
    fn into_schedulable(self) -> Box<dyn Schedulable>;
}