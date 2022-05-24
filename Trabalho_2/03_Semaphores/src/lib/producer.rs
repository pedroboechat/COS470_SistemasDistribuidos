use std::sync::Arc;
use std::sync::{Mutex, MutexGuard};

use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use tokio::spawn;
use tokio::task::JoinHandle;

pub async fn create(
    threads: usize,
    shared_memory:  Arc<Mutex<Vec<i32>>>
) {
    // Vector to store producer thread handles
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(threads);
    
    for _ in 0..threads {

        let mutex: Arc<Mutex<Vec<i32>>> = shared_memory.clone();

        handles.push(
            spawn(async move {

                // Initialize random number generator
                let mut random: ThreadRng = thread_rng();

                loop {
                    // Generate random number
                    let randint: i32 = random.gen_range(1..=10000000);

                    // Acquire mutex guard
                    let mut guard: MutexGuard<Vec<i32>> = mutex.lock().unwrap();

                    // Exit thread at consumer end
                    if guard[0] == -100001 {
                        return;
                    }

                    // Add new value to mutex
                    let free_index = guard.iter().position(|&i| i == 0);
                    match free_index {
                        Some(free_index) => {
                            // println!("{:?} - Free >> {}", guard, free_index);
                            guard[free_index] = randint;
                        },
                        None => ()
                    }

                    // Release mutex guard
                    drop(guard);
                }
            })
        );
    }

    println!("Waiting");

    // For-loop through all the producer thread handles
    for handle in handles {
        // Awaits for the thread to finish running
        handle.await.unwrap();
    }
}