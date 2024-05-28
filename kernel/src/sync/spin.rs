//! # Spin lock
//! ## References
//! * [Rust atomic and locks](https://www.oreilly.co.jp/books/9784814400515/) Chapter 4

use {
    core::sync::atomic::{
        AtomicBool,
        Ordering::{
            Acquire,
            Release,
        },
    },
    crate::x64,
};

pub struct Lock {
    locked: AtomicBool,
}

impl Lock {
    pub const fn new() -> Self {
        let locked = AtomicBool::new(false);
        Self {
            locked,
        }
    }

    pub fn lock(&self) {
        while self.locked.swap(true, Acquire) {
            x64::pause();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

