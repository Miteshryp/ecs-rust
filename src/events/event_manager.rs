use std::{any::{TypeId}, sync::mpsc::{self, Receiver, Sender}};
use hashbrown::HashMap;

use crate::system::param::{EventReader, EventWriter};

use super::Event;


/// The unit of World which is responsible for flushing and storing 
/// buffers of different types of events being transmitted by the systems.
pub struct EventManager {

    /// Buffer storage for storing events read from the event_reader
    /// These buffers are flushed and updated on every call of 
    /// [EventManager::refresh_update]
    events: HashMap<TypeId, Vec<Box<dyn Event>>>,

    /// MPSC Channel reader end
    event_reader: Receiver<Box<dyn Event>>, // channel to be created

    /// MPSC Channel writer end
    event_writer: Sender<Box<dyn Event>>,
}

impl EventManager {

    /// ### Description
    /// 
    /// Creates a new [EventManager] object for the world
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        
        Self {
            events: HashMap::new(),
            event_reader: rx,
            event_writer: tx
        }
    }

    /// ### Description
    /// 
    /// Flushes the old event buffer and stores the newly 
    /// written event in the buffer for processing
    /// 
    /// [EventReader]s executed after this call will read 
    /// updated events.
    pub fn refresh_update(&mut self) {
        // Cleaning the Read Vectors
        for event_vec in self.events.values_mut() {
            event_vec.clear();
        }

        // Rewriting the Read Vectors
        let mut event = self.event_reader.try_recv();

        // Keep handling events until the reader has no exhausted.
        while event.is_ok() {
            let write_event: Box<dyn Event> = event.unwrap();

            // Event type vec already exists, then we simply push the event
            if let Some(event_vec) = self.events.get_mut(&write_event.event_type_id()) {
                event_vec.push(write_event);
            }

            // Else we need to register the event type and then insert the incoming event
            else {
                self.events.insert(write_event.event_type_id(), vec![write_event]);
            }
            event = self.event_reader.try_recv(); // reading the next element in the FIFO queue
        }
    }


    /// ### Description
    /// 
    /// Finds the vec storing the requested type of event and 
    /// returns the event reader if the buffer contains events
    /// to be read.
    /// 
    /// If no events were found, None is returned to indicate
    /// that system need not execute
    pub fn get_reader<E: Event + 'static>(&self) -> Option<EventReader<E>> {
        match self.events.get(&E::type_id()) {
            Some(event_vec) => {
                if event_vec.len() == 0 {
                    // No new event found, do not need to execute system
                    None
                }

                else {
                    Some(EventReader {
                        reader: event_vec,
                        _marker: std::marker::PhantomData
                    })
                }
            },
            None => None
        }
    }

    /// ### Description
    /// 
    /// Creates and returns a [EventWriter] capable of
    /// writing arbitrary types of event into the event queue
    /// 
    /// The written events can be processed in the next frame
    pub fn get_writer(&self) -> EventWriter {
        EventWriter {
            writer_channel: self.event_writer.clone(),
        }
    }
}