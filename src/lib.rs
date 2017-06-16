#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "std")]
extern crate core;

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

pub type TryLockResult<Guard> = Result<Guard, TryLockError<Guard>>;

pub enum TryLockError<T> {
    Poisoned(PoisonError<T>),
    WouldBlock,
}

pub struct PoisonError<T> {
    _type: PhantomData<T>,
}

pub trait Mutex<T: Sized> {
    /// try locking the Mutex, possibly receiving a `MutexGuard`
    fn try_lock(&mut self) -> TryLockResult<T>;

    /// Set this mutex to lock. This is used by the MutexGuard
    /// to manage the lock status.
    /// 
    /// panics if lock has already been set
    unsafe fn set_lock(&self);

    /// clear this mutex's lock. This is used by the MutexGuard
    /// to manage the lock status.
    /// 
    /// panics if lock has already been cleared
    unsafe fn clear_lock(&self);

    /// get the reference to the underlying data
    /// 
    /// Note that `ref_mut` uses an immutable reference,
    /// so the data is not guaranteed to be locked by
    /// these methos.
    unsafe fn get_ref(&self) -> &T;

    /// get a mutable reference to the underlying data
    /// 
    /// Note that `ref_mut` uses an immutable reference,
    /// so the data is not guaranteed to be locked by
    /// these methos.
    unsafe fn get_ref_mut(&self) -> &mut T;
}

pub struct MutexGuard<'a, T: Sized + 'a> {
    __lock: &'a Mutex<T>,
}

impl<'a, T: Sized + 'a> MutexGuard<'a, T> {
    /// create a new MutexGuard from a Mutex
    pub fn new<M>(mutex: &'a M) -> MutexGuard<'a, T> 
    where M: Mutex<T> {
        MutexGuard {
            __lock: mutex,
        }
    }
}

impl<'a, T: Sized + 'a> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe { self.__lock.clear_lock() }
    }
}

impl<'a, T: Sized + 'a> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.__lock.get_ref() }
    }
}

impl<'a, T: Sized + 'a> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.__lock.get_ref_mut() }
    }
}
