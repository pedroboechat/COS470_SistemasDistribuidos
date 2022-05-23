use std::sync::atomic::{AtomicBool, Ordering};

/// Lock struct
pub struct Lock {
    pub held: AtomicBool
}

impl Lock {
    /// Create a new lock
    pub fn new() -> Self {
        return Self { held: AtomicBool::new(false) };
    }

    /// Tries to acquire the lock, *busy-waiting*
    pub fn acquire(&self) {
        while self.held.swap(
            true, 
            Ordering::Acquire
        ) {};
    }
    
    /// Releases the lock
    pub fn release(&self) {
        self.held.store(
            false,
            Ordering::Release
        );
    }
}


