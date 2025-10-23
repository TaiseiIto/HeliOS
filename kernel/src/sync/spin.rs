//! # Spin lock
//! ## References
//! * [Rust atomic and locks](https://www.oreilly.co.jp/books/9784814400515/) Chapter 4

use {
    crate::x64,
    core::{
        cell::UnsafeCell,
        ops::{Deref, DerefMut},
        sync::atomic::{
            AtomicBool,
            Ordering::{Acquire, Release},
        },
    },
};

#[derive(Debug)]
pub struct Guard<'a, T> {
    lock: &'a Lock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}

unsafe impl<T> Send for Guard<'_, T> where T: Send {}

unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

#[derive(Debug)]
pub struct Lock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> Lock<T> {
    pub const fn new(value: T) -> Self {
        let locked = AtomicBool::new(false);
        let value = UnsafeCell::new(value);
        Self { locked, value }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            x64::pause();
        }
        let lock = self;
        Guard { lock }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

unsafe impl<T> Sync for Lock<T> where T: Send {}
