pub mod component_manager;
pub mod handles;


/// ### Description
/// 
/// The trait declaration which detects [Component] types
/// across the ECS system
/// 
/// This trait must be implemented for all user-defined 
/// components using the [ecs_macros::Component] derive macro
pub trait Component {
    fn get_name() -> String
    where
        Self: Sized;
}

