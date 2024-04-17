use std::{
    any::TypeId,
    marker::PhantomData,
    sync::mpsc::Sender,
};

use crate::ecs_base::ECSBase;
use ecs_macros::SystemParam;

use crate::{events::Event, world::World};

use super::{InitError, SystemParam};

/// ## Description
/// A [SystemParam] type used in systems to read events
/// from [`EventManager`](crate::events::event_manager::EventManager) in a world.
///
/// @NOTE: [EventReader]s and [EventWriter]s are a special types
/// of system param since they do not conflict with any world based
/// resource. As a result of this, they can get scheduled in the parallel
/// DAG on the top, and readers and writers can get executed in parallel
///
/// Hence, the events written by an [EventWriter] can only be read in the next
/// schedule execution cycle, when the buffer has been stored in the
/// [`EventManager`](crate::events::event_manager::EventManager)
#[derive(SystemParam)]
pub struct EventReader<E: Event + 'static> {
    pub(crate) reader: *const Vec<Box<dyn Event>>,
    pub(crate) _marker: PhantomData<E>,
}

impl<E: Event + 'static> EventReader<E> {
    pub fn read_events(&self) -> Vec<&E> {
        let vec = unsafe { &*self.reader };

        vec.iter()
            .map(|e| e.as_any().downcast_ref::<E>().unwrap())
            .collect()
    }

    pub fn read_owned_events(&self) -> Vec<&E> {
        let vec = unsafe { &*self.reader };

        vec.iter()
            .map(|e| {
                e.as_any().downcast_ref::<E>().unwrap().to_owned()
            })
            .collect()
    }
}

impl<E: Event + 'static> SystemParam for EventReader<E> {
    fn initialise(world: &World) -> (Option<InitError>, Option<Self>) {
        match world.get_event_reader() {
            Some(reader) => (None, Some(reader)),
            None => (Some(InitError {}), None), // Event type is not registered. Skip the system execution
        }
    }

    fn get_resource_access_type() -> hashbrown::HashSet<TypeId> {
        hashbrown::HashSet::new()
    }

    fn is_resource_access_mut() -> bool {
        false
    }
}

/// ### Description
///
/// A [SystemParam] type used to write event into an
/// [EventManager](crate::events::event_manager::EventManager) of an
/// assigned [`world`](crate::World).
/// The written events can only be read in the next frame if written
/// in a parallel schedule
///
/// @NOTE: For event system behaviour explaination, see notes for [EventReader]
#[derive(SystemParam)]
pub struct EventWriter {
    pub(crate) writer_channel: Sender<Box<dyn Event>>,
}

impl EventWriter {
    /// Sends the event to the world [crate::events::event_manager::EventManager]
    /// to be processed by event reader systems.
    pub fn send_event<E: Event + 'static>(&self, event: E) {
        self.writer_channel.send(Box::new(event)).unwrap()
    }
}

impl SystemParam for EventWriter {
    fn initialise(world: &World) -> (Option<InitError>, Option<Self>)
    where
        Self: Sized,
    {
        (None, Some((*world).get_event_writer()))
    }

    fn get_resource_access_type() -> hashbrown::HashSet<TypeId> {
        hashbrown::HashSet::new()
    }

    fn is_resource_access_mut() -> bool {
        false
    }
}
