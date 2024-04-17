pub mod component_manager;
pub mod handles;

use crate::ecs_base::ECSBase;

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

