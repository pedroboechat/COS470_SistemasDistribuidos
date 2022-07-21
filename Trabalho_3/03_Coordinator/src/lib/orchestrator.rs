use std::sync::{Arc, Mutex};

use futures::future;

use crate::lib::{coordinator, interface, queue::Queue, socket_handler};

/// Execute the orchestrator for the coordinator and its interface
/// ## Arguments
/// * `address` - Binding address for the socket.
pub async fn run(address: String) {
    // Shared memory for the process queue
    let shared_memory: Arc<Mutex<Queue>> = Arc::new(Mutex::new(Queue::new()));

    // Await for coordinator, the socket handler and its interface
    let (_i, _c, _sh) = future::join3(
        interface::create(shared_memory.clone()),
        coordinator::create(shared_memory.clone()),
        socket_handler::create(address, shared_memory.clone()),
    ).await;
}
