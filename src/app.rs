use std::{
    cell::RefCell,
    rc::Rc,
    sync::mpsc::{channel, Receiver},
};

use crate::{
    component::Component,
    ecs_base::ECSBase,
    schedule::{
        holder::ScheduleHolder,
        parallel::ParallelSchedule,
        schedulable::{IntoSchedulable, Schedulable},
        FlowFrequency, Schedule,
    },
    system::{base::SystemMarker, System},
    world::{command_type::CommandFunction, unsafe_world::UnsafeWorldContainer, World},
};

/// ### ECS App
/// The main Application responsible for enclosing the ECS system
///
/// The [`App`] struct is the user level interface for starting the ECS system
/// and initiating the interactions with the system.
/// The main function of the [`App`] struct is to register different user systems
/// which handle different [`Component`]s and [`ComponentSystem`]s across the system
///
/// ---
///
/// The [`App`] struct contains 2 fields:
///     - #### world:
///         In laymens terms, this is the state container of the system. See
///         [`World`] for more info.

///     - #### systems:
///         This is a vector of systems stored in the `App` scheduled for execution
///         on the Application. This vector stores a `BaseSystem` type, however this
///         `BaseSystem` class is implemented in a specialised way by the derivable
///         macro for the type of the system. See [`ecs_macro`](ecs_macro) for more
///
/// ---
///
/// ### Example:
///
/// ```
/// // @TODO: Write this example
/// fn main() {
///     
/// }
/// ```
///
/// @TODO: Determing the Schedule flow architecture
pub struct App {
    world_container: UnsafeWorldContainer,
    schedule_flows: Vec<ScheduleHolder>,

    // Command buffers being received by the world
    // command_buffer: Receiver<CommandFunction>,
    command_buffer: Receiver<Box<dyn FnMut(&mut World) -> ()>>,
}

// pub trait SystemParam {}

impl App {
    pub fn new() -> Self {
        // let (sx, rx) = channel::<CommandFunction>();
        let (sx, rx) = channel::<Box<dyn FnMut(&mut World) -> ()>>();

        App {
            world_container: UnsafeWorldContainer::new(sx),
            schedule_flows: vec![],
            command_buffer: rx, // systems: vec![],
        }
    }

    ///
    /// ### Description
    ///
    /// @TODO: Define and Document a Schedule flow properly
    /// @TODO: Design and implement a solution to adjust flow execution
    ///     frequency
    ///
    /// Registers a Schedule flow in the App.
    /// The order of registration determines the execution priority of the
    /// flow being registered.
    /// Flows that get registered first will get executed first.
    ///
    /// ### Return Value:
    /// An index representing the priority order of the registered flow
    /// Lower the order, higher the priority.
    pub fn register_flow(&mut self, frequency: FlowFrequency) -> usize {
        let index = self.schedule_flows.len();
        self.schedule_flows.push(ScheduleHolder::new(frequency));
        index
    }

    ///
    /// ### Description
    ///
    /// Adds a schedulable item into a specified schedule flow.
    /// This determines the order or frequency of flow execution.
    pub fn add_to_flow(&mut self, flow_index: usize, item: impl Schedule + 'static) {
        self.schedule_flows[flow_index].add(Box::new(item));
    }

    /// ### Description
    /// 
    /// Sets the world as active and start the update cycle
    /// The update cycle continues untill a insereted system in a schedule
    /// sets the world as inactive
    pub fn start(&mut self) {
        self.world_container.get_world_mut().set_active(true);
        while self.world_container.get_world().is_active() {
            self.update();
        }
    }

    ///
    /// ### Description
    /// 
    ///  Calls the update process on all the systems in the App.
    /// 
    /// @ISSUE: Events are being flushed at the start of the frame.
    /// Now, since only event reader system function are not dependent 
    /// on any world resource, they are going to be executed first in the
    /// system schedule. 
    /// This is a problem since the event readers will not read anything
    /// at the start of a schedule, and in the next frame
    /// 
    pub fn update(&mut self) {

        // Flushing events from buffer.
        self.world_container.get_world_mut().update_event_state();

        for flow in &mut self.schedule_flows {
            flow.run_all(&self.world_container);

            // Flushing and executing the command buffer
            let mut result = self.command_buffer.try_recv();

            while result.is_ok() {
                let mut command = result.unwrap();

                // Executing the command buffer
                (command)(self.world_container.get_world_mut());

                result = self.command_buffer.try_recv();
            }
        }
    }

    pub fn register_component<C: Component + 'static>(&mut self) {
        self.world_container
            .get_world_mut()
            .register_component::<C>();
    }

    // pub fn process_events(&mut self) {
    //     for system in &mut self.systems {
    //         system.process_events(&mut self.world_container);
    //     }
    // }
}
