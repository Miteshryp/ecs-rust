use std::{
    any::TypeId,
    marker::PhantomData,
    slice::Iter,
    sync::mpsc::{self, Sender},
};

use crate::{
    component::resource::Resource,
    events::{event_manager::EventManager, Event},
    world::World,
};
use mpsc::channel;

/// Trait implemented by all types which can be passed into a functional
/// system as a system param. These are types which associate the requested
/// world state type to the appropriate world object based on the metadata
/// provided to the type
pub trait SystemParam {
    fn type_id() -> TypeId
    where
        Self: Sized + 'static,
    {
        TypeId::of::<Self>()
    }

    // We pass a mut reference of the World to ensure that if the initialisation
    // requires a pointer access with safety guarentees, that may be facilitated.
    // (We cannot get a pointer through &World)
    fn initialise(world: &mut World) -> Self;
}

/// ## Description
/// A [SystemParam] type used in systems to read events
/// from [`EventManager`](crate::events::event_manager::EventManager) in a world.

pub struct EventReader<'a> {
    pub(crate) manager: &'a EventManager,
    // pub(crate) reader: &'a Vec<Box<dyn Event>>,
    // pub(crate) reader: *const Vec<Box<dyn Event>>,
    // pub(crate) _marker: PhantomData<E>
}

impl EventReader<'_> {
    pub fn read_events<E: Event>(&self) -> Option<Vec<&E>> {
        // sys1 -> EventReader<E1> { &v }
        // sys2 -> EventReader<E1> { &v }

        if let Some(event_iter) = self.manager.get_event_vec::<E>() {
            Some(
                event_iter
                    .map(|b| b.as_any().downcast_ref::<E>().unwrap())
                    .into_iter()
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl SystemParam for EventReader<'_> {
    fn initialise(world: &mut World) -> Self {
        world.get_event_reader()
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
    pub fn send_event<E: Event>(&self, event: E) {
        self.writer_channel.send(Box::new(event)).unwrap()
    }
}

impl SystemParam for EventWriter {
    fn initialise(world: &mut World) -> Self {
        world.get_event_writer()
    }
}

pub struct ResourceHandle<R>
where
    R: Resource,
{
    world: *const World,
    _marker: PhantomData<R>,
}

pub struct MutResourceHandle<R>
where
    R: Resource,
{
    world: *mut World,
    _marker: PhantomData<R>,
}

/// ## Description
/// Extract the specified type of Components from the world to query.
/// The struct supplies the component's along with their [`Entity`](crate::entity::Entity)
/// if and only if
///     - All components specified are attached to the entity_id.
///     - All components are free for use and not being held by another system for use. (NOTE: This has a high chance of resulting in a deadlock through mutual starvation)
///
pub struct Query {
    // SAFETY: @TODO
    world: *mut World,
}
