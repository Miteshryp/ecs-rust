use std::sync::mpsc::Sender;

use crate::ecs_base::ECSBase;
use crate::system::SystemParam;
use crate::world::World;
use ecs_macros::SystemParam;

use super::InitError;

#[derive(SystemParam)]
pub struct CommandBufferWriter {
    // pub(crate) writer_channel: Sender<CommandFunction>,
    pub(crate) writer_channel: Sender<Box<dyn FnMut(&mut World) -> ()>>,
}

impl SystemParam for CommandBufferWriter {
    fn initialise(world: &World) -> (Option<InitError>, Option<Self>)
    where
        Self: Sized,
    {
        (
            None,
            Some(Self {
                writer_channel: (*world).get_command_writer(),
            }),
        )
    }

    fn get_resource_access_type() -> hashbrown::HashSet<std::any::TypeId> {
        hashbrown::HashSet::new()
    }

    fn is_resource_access_mut() -> bool {
        false
    }
}

impl CommandBufferWriter {
    // pub fn add_command(&self, func: fn(&mut  World) -> ()) {
    //     self.writer_channel.send(func);
    // }
    pub fn add_command<Func: FnMut(&mut World) + 'static>(&self, func: Func) {
        let _ = self.writer_channel.send(Box::new(func));
    }
}
