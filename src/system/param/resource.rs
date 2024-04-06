use super::SystemParam;
use crate::{component::resource::Resource, world::World};
use std::{
    marker::PhantomData,
    sync::{RwLockReadGuard, RwLockWriteGuard},
};

///
///     @NOTE: We get a mutable pointer to the world as a input to the initialise method
///     This is to facilitate RwLockGuard acquisition from the world for Resource
///     handles. &mut World makes rust believe the returned guard does not have
///     lifetime which lives long enough, so we fool the borrow checker this way
///     to make it think that the reference is static.
pub struct ResourceHandle<R: Resource + 'static>
where
    R: Resource,
{
    inner_guard_box: Box<RwLockReadGuard<'static, Box<dyn Resource>>>,
    _marker: PhantomData<R>,
}

impl<'a, R: Resource + 'static> SystemParam for ResourceHandle<R> {
    fn initialise(world: *mut World) -> Option<Self> {
        // @SAFETY:
        // 1. The world does not go out of scope (Otherwise we wouldn't be executing this function)
        // 2. The guard is only returned by the world only if no other
        //      mutable access guard to the resource is alive
        unsafe {
            match (*world).get_resource_ref::<R>() {
                Some(guard_box) => Some(Self {
                    inner_guard_box: guard_box,
                    _marker: std::marker::PhantomData,
                }),
                None => None,
            }
        }
    }
}

impl<R: Resource + 'static> ResourceHandle<R> {
    pub fn get_resource(&mut self) -> &R {
        self.inner_guard_box.as_any().downcast_ref::<R>().unwrap()
    }
}

pub struct MutResourceHandle<'a, R: Resource + 'static>
where
    R: Resource,
{
    inner_guard_box: Box<RwLockWriteGuard<'a, Box<dyn Resource>>>,
    pub(crate) _marker: PhantomData<R>,
}

impl<'a, R: Resource + 'static> SystemParam for MutResourceHandle<'a, R> {
    fn initialise(world: *mut World) -> Option<Self> {
        // @SAFETY:
        // 1. See safety description in [`ResourceHandle`]
        // 2. The lock is only returned by the world only when the Resource is
        //      not mutably or immutably borrowed by some process in the system.
        unsafe {
            match (*world).get_resource_mut::<R>() {
                Some(guard_box) => Some(Self {
                    inner_guard_box: guard_box,
                    _marker: std::marker::PhantomData,
                }),
                None => None,
            }
        }
    }
}

impl<'a, R: Resource + 'static> MutResourceHandle<'a, R> {
    pub fn get_resource_mut(&mut self) -> &mut R {
        self.inner_guard_box
            .as_any_mut()
            .downcast_mut::<R>()
            .unwrap()
    }
}
