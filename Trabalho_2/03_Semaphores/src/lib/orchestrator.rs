use std::iter;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use futures::future;

use crate::lib::{producer, consumer};

/// Execute the orchestrator for the producer-consumer
/// ## Arguments
/// * `n` - Amount of shared memory.
/// * `p` - Amount of producer threads.
/// * `c` - Amount of consumer threads.
pub async fn run(n: usize, p: usize, c: usize) -> u128 {

    // Shared memory vector
    let shared_memory: Arc<Mutex<Vec<i32>>> = Arc::new(
        Mutex::new(
            Vec::from_iter(
                iter::from_fn(move || {
                    return Some(0);
                }).take(n + 1)
            )
        )
    );

    // Set counter to -1
    let mut guard = shared_memory.lock().unwrap();
    guard[0] = -1;
    drop(guard);

    // Initialize timer
    let timer: Instant = Instant::now();

    // Await for producer and consumer
    let (_p, _c) = future::join(
        producer::create(
            p,
            shared_memory.clone()
        ),
        consumer::create(
            c,
            shared_memory.clone()
        )
    ).await;

    // Gets elapsed time, in milliseconds
    let elapsed: u128 = timer.elapsed().as_millis();

    println!(
        "Took {} milliseconds.",
        elapsed
    );

    return elapsed;

}
