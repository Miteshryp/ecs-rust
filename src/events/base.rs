use std::{any::TypeId};

use crate::ecs_base::ECSBase;


/// Base event type. To be included using a derive macro
/// 
/// This must be implemented by all user-defined events 
/// using the [ecs_macros::Event] derive macro
pub trait Event: ECSBase {
    ///
    /// Interface for getting the type id of the 
    /// underlying event. 
    ///
    /// Useful for dynamically dispatched events in the 
    /// [crate::events::event_manager::EventManager]
    /// 
    fn event_type_id(&self) -> std::any::TypeId;
}