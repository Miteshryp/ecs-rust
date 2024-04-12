use super::World;

pub enum CommandType {
    AddEntityWithComponents,
    AddEntity,
    RemoveEntity,
    RemoveComponentFromEntity,
    AddResource,
    RemoveResource
}

pub type CommandFunction = fn (&mut World) -> ();
