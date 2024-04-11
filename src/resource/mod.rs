use std::any::TypeId;

use crate::ecs_base::ECSBase;

/// A Resource is a singleton component in an ECS world which is not 
/// attached to any entity. A Resource lives independently in the world
/// and can be accessed by any component in the world (synchronously)
pub trait Resource: ECSBase {
    fn get_type() -> TypeId where Self: Sized + 'static {
        TypeId::of::<Self>()
    }

    fn get_name(&self) -> String;
}

pub type ResourceId = TypeId;