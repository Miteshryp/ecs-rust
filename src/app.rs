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
/// which handle different systems inserted into the app in form of [`Schedule`]s
///
/// ---
///
/// The [`App`] struct contains 2 fields:
///     - #### world:
///         In laymens terms, this is the state container of the system. See
///         [`World`] for more info.
/// 
/// - #### schedule_holder:
///     This is a container which contains different types of [Schedule]s inserted
///     in a system to be executed in a predetermined order
///     There is definitely some overlap between a 
///     [`crate::schedule::holder::ScheduleHolder`] and [`Schedule`]s,
///     which can make it hard to understand. You can relate the 2 as follows:
/// 
///     While the schedule is responsible for ensuring that no 2 conflicting systems 
///     inserted in the schedule get a chance to execute in parallel, the schedule
///     flow is an enclosing entity around that schedule that allows user to define
///     certain settings and configurations on the schedules inserted in the flow.
///     
///     The world stores registered [ScheduleHolder]s in a queue and executes them
///     in a serial fashion each update cycle
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
    command_buffer: Receiver<Box<dyn FnMut(&mut World) -> ()>>,
}


impl App {
    pub fn new() -> Self {
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
    /// Registers a [`holder`](ScheduleHolder) in the App.
    /// The order of registration determines the execution priority of the
    /// [`holder`](ScheduleHolder) being registered.
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
    /// Used to add a [`Schedulable`] item (which is system function) into a specified
    /// [`holder`](ScheduleHolder) position. 
    /// This determines the order or frequency of execution for the holder.
    pub fn add_to_flow(&mut self, flow_index: usize, item: impl Schedule + 'static) {
        self.schedule_flows[flow_index].add(Box::new(item));
    }

    /// ### Description
    /// 
    /// Sets the world as active and starts the update cycle
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
    /// Calls the update process on all the systems in the [App].
    /// 
    /// This method is not required to be called explicitly by the user
    /// since it is automatically executed untill required when the 
    /// [`app`](App) is started
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

    /// @TODO: Document
    pub fn register_component<C: Component + 'static>(&mut self) {
        self.world_container
            .get_world_mut()
            .register_component::<C>();
    }
}
