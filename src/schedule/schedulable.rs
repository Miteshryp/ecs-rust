use crate::{
    system::{dependency::SystemMetadata, param::InitError},
    world::unsafe_world::UnsafeWorldContainer,
};

/// @SAFETY:
/// Any schedulable element is thread safe only after its dependencies
/// have been initialised and stored in the struct which is to be transferred
/// across thread boundaries.
/// This property must be maintained by all [Schedule]s
// pub trait Schedulable: Sync {
pub trait Schedulable: Send + Sync {

    /// Function used by a schedule to get metadata about the 
    /// world based resources required by the system
    fn initialise_dependency_metadata(&mut self) -> SystemMetadata;

    /// Attempts to initialise the system by acquiring locks on
    /// resources required for system execution
    fn initialise_dependencies(&mut self, world: &UnsafeWorldContainer) -> Option<InitError>;

    /// Executes the system based on the resource locks acquired
    /// **NOTE:** This should always be executed after the 
    /// [`initialise_dependencies`](Schedulable::initialise_dependencies) function
    fn run(&mut self);
}

pub trait IntoSchedulable<Marker> {
    type Output: Schedulable + 'static;

    fn into_schedulable(self) -> Box<Self::Output>;

    /// System B is executed before System A in a relatively sequential order
    fn after<M>(self, system: impl IntoSchedulable<M>) -> DependentSystems;

    /// System A is executed before System B in a relatively sequential order
    fn before<M>(self, system: impl IntoSchedulable<M>) -> DependentSystems;
}

pub struct DependentSystems {
    pub(crate) systems: Vec<Box<dyn Schedulable>>,
}

impl DependentSystems {
    pub fn queue_to_back<M>(&mut self, mut system: impl IntoSchedulable<M>) {
        self.systems.push(system.into_schedulable());
    }

    pub fn queue_to_front<M>(&mut self, mut system: impl IntoSchedulable<M>) {
        self.systems.insert(0, system.into_schedulable());
    }
}