use std::{any::{Any, TypeId}, slice::Iter, sync::mpsc::{self, Receiver, Sender}};
use hashbrown::HashMap;

use crate::system::param::{EventReader, EventWriter};

use super::Event;

pub struct EventManager {
    events: HashMap<TypeId, Vec<Box<dyn Event>>>,
    event_reader: Receiver<Box<dyn Event>>, // channel to be created
    event_writer: Sender<Box<dyn Event>>,
}

impl EventManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        
        Self {
            events: HashMap::new(),
            event_reader: rx,
            event_writer: tx
        }
    }

    pub fn refresh_update(&mut self) {
        // Cleaning the Read Vectors
        for event_vec in self.events.values_mut() {
            event_vec.clear();
        }


        // Rewriting the Read Vectors
        let mut event = self.event_reader.try_recv();

        // Keep handling events until the reader has no exhausted.
        while event.is_ok() {
            // Event type vec already exists, then we simply push the event
            if let Some(event_vec) = self.events.get_mut(&event.type_id()) {
                event_vec.push(event.unwrap());
            } 
            // Else we need to register the event type and then insert the incoming event
            else {
                let write_event = event.unwrap();
                self.events.insert(write_event.type_id(), vec![write_event]);
            }
            event = self.event_reader.try_recv(); // reading the next element in the FIFO queue
        }
    }


    pub fn get_reader<E: Event + 'static>(&mut self) -> Option<EventReader<E>> {
        match self.events.get(&E::type_id()) {
            Some(event_vec) => Some(EventReader {
                reader: event_vec,
                _marker: std::marker::PhantomData
            }),
            None => None
        }
    }

    pub fn get_writer(&mut self) -> EventWriter {
        EventWriter {
            writer_channel: self.event_writer.clone(),
        }
    }
}


impl EventManager {
    pub(crate) fn get_event_vec<E: Event + 'static>(&self) -> Option<Iter<'_, Box<dyn Event>>> {
        if let Some(event_vec) = self.events.get(&TypeId::of::<E>()) {
            Some(event_vec.iter())
        } else {
            None
        }
    }
}