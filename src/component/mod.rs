pub mod component_manager;
pub mod handles;


use crate::ECSBase;
use ecs_macros::ECSBase;

/// ### Description
/// 
/// The trait declaration which detects [Component] types
/// across the ECS system
/// 
/// This trait must be implemented for all user-defined 
/// components using the [ecs_macros::Component] derive macro
/// 
// #[derive(ECSBase)]
pub trait Component: ECSBase {
    fn get_name() -> String
    where
        Self: Sized;
}

