use std::any::TypeId;

use crate::ecs_base::ECSBase;

/// 
/// ### Description
/// 
/// A [Resource] is a singleton component in an ECS world which is not 
/// attached to any entity. A Resource lives independently in the world
/// and can be accessed by any component in the world (synchronously)
/// 
/// All user-defined resources must implement this trait using the
/// [ecs_macros::Resource] derive macro
pub trait Resource: ECSBase {
}

pub type ResourceId = TypeId;