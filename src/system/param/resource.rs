use std::{marker::PhantomData, ops::{Deref, DerefMut}, ptr::{null, null_mut}, sync::{RwLockReadGuard, RwLockWriteGuard}};
use crate::{component::resource::Resource, world::World};
use super::SystemParam;


pub struct ResourceHandle<'a, R: Resource + 'static>
where
    R: Resource,
{
    world: *const World,
    inner_guard: *mut RwLockReadGuard<'a, Box<dyn Resource>>,
    _marker: PhantomData<R>,
}

impl<'a, R: Resource + 'static> SystemParam for ResourceHandle<'a, R> {
    fn initialise(world: &mut World) -> Self {
        Self {
            world: world,
            inner_guard: null_mut(), // Pointer to owned read guard
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, R: Resource + 'static>  ResourceHandle<'a, R> {
    pub fn get_resource(&mut self) -> &R {
        // Acquiring owned read guard from the world
        //
        // @SAFETY: The guard is only returned by the world only  if no other
        //      mutable access guard to the resource is alive
        let lock = unsafe {
            let lock_box = (*self.world).get_resource_ref::<R>();
            self.inner_guard = Box::into_raw(lock_box);
            &mut (*self.inner_guard)
        };

        lock.as_any().downcast_ref::<R>().unwrap()
    }
}

impl<'a, R: Resource + 'static> Drop for ResourceHandle<'a, R> {
    fn drop(&mut self) {
        unsafe {
            // Dropping owned read guard
            if !self.inner_guard.is_null() {
                let _ = Box::from_raw(self.inner_guard);
            }

            self.inner_guard = null_mut();
        }
    }
}






pub struct MutResourceHandle<'a, R: Resource + 'static>
where
    R: Resource,
{
    world: *mut World,
    inner_guard: *mut RwLockWriteGuard<'a, Box<dyn Resource>>,
    pub(crate) _marker: PhantomData<R>,
}

impl<'a, R: Resource + 'static> SystemParam for MutResourceHandle<'a, R> {
    fn initialise(world: &mut World) -> Self {
        Self {
            world: world,
            inner_guard: null_mut(),
            _marker: std::marker::PhantomData
        }
    }
}

impl<'a, R: Resource + 'static> MutResourceHandle<'a, R> {
    pub fn get_resource_mut(&mut self) -> &mut R {
        // Acquiring owned write guard to the resource from the world
        //
        // @SAFETY: The lock is only returned by the world only when the Resource is
        // not mutably or immutably borrowed by some process in the system.
        let lock = unsafe { 
            let lock_box = (*self.world).get_resource_mut::<R>();
            self.inner_guard = Box::into_raw(lock_box);
            &mut (*self.inner_guard)
        };

        lock.as_any_mut().downcast_mut::<R>().unwrap()
    }
}


impl<'a, R: Resource + 'static> Drop for MutResourceHandle<'a, R> {
    fn drop(&mut self) {
        unsafe {
            // Getting rid of the owned guard
            if !self.inner_guard.is_null() {
                let _ = Box::from_raw(self.inner_guard);
            }

            self.inner_guard = null_mut();
        }
    }
}