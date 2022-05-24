use std::sync::Arc;
use std::sync::{Mutex, MutexGuard};

use tokio::spawn;
use tokio::task::JoinHandle;

use crate::lib::prime::is_prime;

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

                loop {

                    // Acquire mutex guard
                    let mut guard: MutexGuard<Vec<i32>> = mutex.lock().unwrap();

                    // Exit thread at consumer end
                    if guard[0] == -100001 {
                        println!("Ending");
                        return;
                    }
                    else {
                        guard[0] -= 1;
                    }

                    // Add new value to mutex
                    let value_index = guard.iter().position(|&i| i > 0);
                    match value_index {
                        Some(value_index) => {
                            let value: i32 = guard[value_index];
                            guard[value_index] = 0;
                            println!("{} = {:?}", value, is_prime(value));
                        },
                        None => ()
                    }

                    // Release mutex guard
                    drop(guard);
                }
            })
        );
    }

    // For-loop through all the producer thread handles
    for handle in handles {
        // Awaits for the thread to finish running
        handle.await.unwrap();
    }
}