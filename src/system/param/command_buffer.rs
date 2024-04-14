use std::sync::mpsc::Sender;

use ecs_macros::SystemParam;
use crate::system::SystemParam;
use crate::ecs_base::ECSBase;
use crate::world::command_type::{CommandFunction, CommandType};
use crate::world::World;

use super::InitError;



#[derive(SystemParam)]
pub struct CommandBufferWriter {
    pub(crate) writer_channel: Sender<CommandFunction>
}

impl SystemParam for CommandBufferWriter {
    fn initialise(world: *mut crate::world::World) -> (Option<InitError>, Option<Self>) where Self: Sized {
        unsafe {
            (
                None,
                Some(Self {
                    writer_channel: (*world).get_command_writer()
                })
            )   
        }
    }
    
    fn get_resource_access_type() -> hashbrown::HashSet<std::any::TypeId> {
        hashbrown::HashSet::new()
    }
    
    fn is_resource_access_mut() -> bool {
        false
    }
}

impl CommandBufferWriter {
    pub fn add_command(&self, func: fn(&mut  World) -> ()) {
        self.writer_channel.send(func);
    }
}