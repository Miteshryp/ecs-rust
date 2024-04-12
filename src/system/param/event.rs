use std::{any::{Any, TypeId}, marker::PhantomData, sync::mpsc::Sender};

use crate::ECSBase;
use ecs_macros::SystemParam;

use crate::{events::Event, world::{unsafe_world::UnsafeWorldContainer, World}};

use super::{InitError, SystemParam};


/// ## Description
/// A [SystemParam] type used in systems to read events
/// from [`EventManager`](crate::events::event_manager::EventManager) in a world.


#[derive(SystemParam)]
pub struct EventReader<E: Event + 'static> {
    pub(crate) reader: *const Vec<Box<dyn Event>>,
    pub(crate) _marker: PhantomData<E>,
}

impl<E: Event + 'static> EventReader<E> {
    pub fn read_events(&self) -> Vec<&E> {
        let vec = unsafe { &*self.reader };

        vec
            .iter()
            .map(|e| e.as_any().downcast_ref::<E>().unwrap())
            .collect()
    }
}

impl<E: Event + 'static> SystemParam for EventReader<E> {
    fn initialise(world: *mut World) -> (Option<InitError>, Option<Self>) {
        unsafe {
            match (*world).get_event_reader() {
                Some(reader) => (None, Some(reader)),
                None => (Some(InitError{}), None), // Event type is not registered. Skip the system execution
            }
        }
    }
}

/// ## Description
///
/// A [SystemParam] type used to write event into [EventManager](crate::events::event_manager::EventManager) of an
/// assigned [`world`](crate::World).
///
pub struct EventWriter {
    pub(crate) writer_channel: Sender<Box<dyn Event>>,
}

impl EventWriter {
    pub fn send_event<E: Event + 'static>(&self, event: E) {
        self.writer_channel.send(Box::new(event)).unwrap()
    }
}

// impl SystemParam for EventWriter {
//     fn as_any(&self) -> &dyn Any {
//         self as &dyn Any
//     }

//     fn as_any_mut(&mut self) -> &mut dyn Any{
//         self as &mut dyn Any
//     }

//     fn initialise(world: *mut World) -> Option<Self> {
//         unsafe {
//             Some((*world).get_event_writer())
//         }
//     }
// }