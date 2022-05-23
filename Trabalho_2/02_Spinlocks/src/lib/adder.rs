use std::iter;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Instant;

use rand::{thread_rng, Rng};
use rand::prelude::ThreadRng;
use tokio::spawn;
use tokio::task::JoinHandle;

use crate::spinlock::Lock;

/// Execute adder with spinlocks
/// ## Arguments
/// * `n` - Amount of numbers.
/// * `k` - Amount of threads.
pub async fn run(n: usize, k: usize) -> u128 {
    // Initialize random number generator
    let mut random: ThreadRng = thread_rng();

    // Create random numbers vector
    let numbers: Vec<i32> = Vec::from_iter(
        iter::from_fn(move || {
            return Some(random.gen_range(-100..=100));
        }).take(n)
    );

    // Split numbers vector into (n / k) chunks of same size
    let chunks: Vec<Vec<i32>> = numbers.chunks(n / k).map(
        |x| x.to_vec()
    ).collect();

    // Vector to store thread handles
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(n);
    
    // Create new Lock
    let lock: Arc<Lock> = Arc::new(Lock::new());

    // Create zeroed accumulator
    let acummulator: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));

    // Initialize timer
    let timer = Instant::now();

    // For-loop to create threads
    for chunk in chunks {
        // Create new lock reference
        let lock_ref: Arc<Lock> = Arc::clone(&lock);

        // Create new acummulator reference
        let acummulator_ref: Arc<AtomicI32> = Arc::clone(&acummulator);

        // Create thread and push its handle to the handles vector
        handles.push(
            spawn(async move {
                // Create local acummulator
                let mut local_acummulator: i32 = 0;

                // Sum the numbers in the chunk
                for number in chunk {
                    local_acummulator += number;
                }

                // Acquires lock
                lock_ref.acquire();

                // Loads current value from the acumulator
                let current_value = acummulator_ref.load(
                    Ordering::Relaxed
                );

                // Stores updated value to acummulator
                acummulator_ref.store(
                     current_value + local_acummulator,
                     Ordering::Relaxed
                );

                // Releases lock
                lock_ref.release();
                return;
            })
        );
    }

    // For-loop through all the thread handles
    for handle in handles {
        // Awaits for the thread to finish running
        handle.await.unwrap();
    }

    // Gets elapsed time, in milliseconds
    let elapsed: u128 = timer.elapsed().as_millis();

    println!(
        "Final number is {}. Took {} milliseconds.",
        acummulator.load(Ordering::Relaxed),
        elapsed
    );

    return elapsed;

}
