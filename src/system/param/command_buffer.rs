use std::sync::mpsc::Sender;

use ecs_macros::SystemParam;
use crate::component::Component;
use crate::system::SystemParam;
use crate::ecs_base::ECSBase;
use crate::world::command_type::{CommandFunction, CommandType};
use crate::world::World;



// CommandBufferReader

#[derive(SystemParam)]
pub struct CommandBufferWriter {
    pub(crate) writer_channel: Sender<CommandFunction>
}

impl SystemParam for CommandBufferWriter {
    fn initialise(world: *mut crate::world::World) -> Option<Self> where Self: Sized {
        unsafe {
            Some(Self {
                writer_channel: (*world).get_command_writer()
            })
        }
    }
}

impl CommandBufferWriter {
    pub fn add_command(&self, func: fn(&mut  World) -> ()) {
        self.writer_channel.send(func);
    }
}