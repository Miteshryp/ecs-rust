use std::{any::TypeId, marker::PhantomData, sync::mpsc::Sender};

use crate::{events::Event, world::{unsafe_world::UnsafeWorldContainer, World}};

use super::SystemParam;


/// ## Description
/// A [SystemParam] type used in systems to read events
/// from [`EventManager`](crate::events::event_manager::EventManager) in a world.

pub struct EventReader<E: Event + 'static> {
    // pub(crate) manager: &'a EventManager,
    // pub(crate) reader: &'a Vec<Box<dyn Event>>,
    pub(crate) reader: *const Vec<Box<dyn Event>>,
    pub(crate) _marker: PhantomData<E>,
}

impl<E: Event + 'static> EventReader<E> {
    // pub fn read_events(&self) -> Option<Vec<&E>> {
    // sys1 -> EventReader<E1> { &v }
    // sys2 -> EventReader<E1> { &v }

    // if let Some(event_iter) = self.manager.get_event_vec::<E>() {
    //     Some(
    //         event_iter
    //             .map(|b| b.as_any().downcast_ref::<E>().unwrap())
    //             .into_iter()
    //             .collect(),
    //     )
    // } else {
    //     None
    // }
    // }

    pub fn read_events(&self) -> Vec<&E> {
        let vec = unsafe { &*self.reader };

        vec
            .iter()
            .map(|e| e.as_any().downcast_ref::<E>().unwrap())
            .collect()
    }
}

impl<E: Event + 'static> SystemParam for EventReader<E> {
    fn initialise(world: *mut World) -> Option<Self> {
        unsafe {
            Some((*world).get_event_reader())
        }
    }

    fn type_id() -> TypeId
    where
        Self: Sized + 'static,
    {
        TypeId::of::<Self>()
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

impl SystemParam for EventWriter {
    fn initialise(world: *mut World) -> Option<Self> {
        unsafe {
            Some((*world).get_event_writer())
        }
        // world.get_world_mut().get_event_writer()
    }
}