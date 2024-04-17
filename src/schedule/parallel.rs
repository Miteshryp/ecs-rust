use super::{schedulable::IntoSchedulable, Schedulable, Schedule, graph::DependencyGraph};

pub struct ParallelSchedule {
    /// A structure storing the systems inserted into a schedule
    /// as a Directed Acyclic Graph. This graph can handle
    /// parameteric or induced dependencies in a given set of
    /// system and can construct and execute a task graph capable of
    /// running independent systems in parallel.
    ///
    /// This structure is the core component of the parallel scheduler.
    dependency_graph: DependencyGraph,
}

impl ParallelSchedule {
    pub fn new() -> Self {
        Self {
            dependency_graph: DependencyGraph::new(),
        }
    }
}

impl Schedule for ParallelSchedule {
    /// ### Description
    ///
    /// Runs the Execution graph constructed by all the systems inserted
    /// into the dependency graph.
    ///
    /// @NOTE:
    /// This function does not guarantee the running of each schedule inserted
    /// into the system. Execution will automatically be stalled
    /// for systems whose dependency initialisation fails
    ///
    /// These systems will continue execution on a cycle when the
    /// initialisation of the system parameters suceeds
    ///
    /// For more info on which schedule will execute and which
    /// won't, see [SystemParam]
    fn run_schedule(&mut self, world: &crate::world::unsafe_world::UnsafeWorldContainer) {
        self.dependency_graph.execute_system_graph(world);
    }

    /// ### Description
    /// 
    /// Adds a singular schedulable system into the schedule
    /// after user has converted it into a [Schedulable] 
    /// type using the [crate::IntoSchedulable::into_schedulable]
    fn add_boxed(&mut self, item: Box<dyn Schedulable>) {
        self.dependency_graph.add_boxed_system(item);
    }

    fn add<Marker>(&mut self, func: impl IntoSchedulable<Marker>) {
        self.dependency_graph.add_boxed_system(func.into_schedulable());
    }

    fn add_ordered(&mut self, systems: super::DependentSystems) {
        self.dependency_graph.add_dependent_systems(systems);
    }
}
