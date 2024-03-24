use std::{any::{Any, TypeId}, collections::HashMap, slice::Iter, sync::mpsc::{self, Receiver, Sender}};

use crate::system::param::{EventReader, EventWriter};

use super::Event;

pub struct EventManager {
    // event_reader: Vec<Box<dyn Event>>,
    // event_writer: Vec<Box<dyn Event>>,

    // events: Vec<Box<dyn Event>>, // events from the previous frame
    events: HashMap<TypeId, Vec<Box<dyn Event>>>,
    read_channel: Receiver<Box<dyn Event>>, // channel to be created
    write_channel: Sender<Box<dyn Event>>,
}

impl EventManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        
        Self {
            // event_reader: vec![],
            // event_writer: vec![]
            // events: vec![],

            events: HashMap::new(),
            read_channel: rx,
            write_channel: tx
        }
    }

    pub fn refresh_update(&mut self) {
        // Cleaning the Read Vectors
        for event_vec in self.events.values_mut() {
            event_vec.clear();
        }


        // Rewriting the Read Vectors
        let mut event = self.read_channel.try_recv();

        while event.is_ok() {
            // event_vec.push(event.unwrap());
            if let Some(event_vec) = self.events.get_mut(&event.type_id()) {
                // Event type vec already exists
                event_vec.push(event.unwrap());
            } else {
                // Need to register the event type and insert the incoming event
                let write_event = event.unwrap();
                self.events.insert(write_event.type_id(), vec![write_event]);
            }
            event = self.read_channel.try_recv(); // reading the next element in the FIFO queue
        }
    }


    pub fn get_reader<E: Event + 'static>(&mut self) -> EventReader<E> {
        // EventReader {
        //     manager: &self
        // }

        if let Some(event_vec) = self.events.get(&E::type_id()) {
            EventReader {
                reader: event_vec,
                _marker: std::marker::PhantomData
            }
        } else {
            // @ERROR
            // @TODO: This line potentially creates conflict in a parallel
            // system (When 2 different system try to read resource which does not exists,
            // It may end up in 2 insert operations. Although at the end only a new empty vec
            // should be inserted, it is still a potential threat.)
            self.events.insert(E::type_id(), vec![]);
            EventReader {
                reader: self.events.get(&E::type_id()).unwrap(),
                _marker: std::marker::PhantomData
            }
        }
    }

    pub fn get_writer(&mut self) -> EventWriter {
        EventWriter {
            writer_channel: self.write_channel.clone(),
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