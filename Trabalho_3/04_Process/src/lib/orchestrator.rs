use std::time::Duration;

use tokio::spawn;
use tokio::task::JoinHandle;

use crate::lib::process;

/// Execute the orchestrator for the coordinator and its interface
/// ## Arguments
/// * `address` - Binding address for the socket.
/// * `n`       - Number of processes.
/// * `k`       - Sleep time.
/// * `r`       - Repetitions.
pub async fn run(address: String, n: usize, k: usize, r: usize) {
    // Convert `k` to Duration for the sleep function
    let sleep_time: Duration = Duration::from_secs(k.try_into().unwrap());

    // Vector to store producer thread handles
    let mut handles: Vec<JoinHandle<_>> = Vec::with_capacity(n);
    
    // Create `n` threads with unique process IDs
    for process_id in 1 ..= u32::try_from(n).unwrap() {
        let thread_address: String = address.clone();
        handles.push(
            spawn(async move {
                process::create(thread_address, process_id, sleep_time, r)
            })
        );
    }

    // For-loop through all the producer thread handles
    for handle in handles {
        // Awaits for the thread to finish running
        handle.await.unwrap().await;
    }
}
