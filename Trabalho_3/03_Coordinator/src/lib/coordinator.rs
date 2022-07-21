use std::io::Write;
use std::sync::{Arc, Mutex, MutexGuard};

use log::info;
use tokio::spawn;
use tokio::task::JoinHandle;

use crate::lib::queue::Queue;
use crate::lib::messages::*;

/// Create the coordinator
/// ## Arguments
/// * `queue` - Process queue.
pub async fn create(
    queue: Arc<Mutex<Queue>>
) {
    // Spawn thread for the coordinator
    let coordinator_handle: JoinHandle<()> = spawn(async move {
        // Thread queue
        let thread_queue: Arc<Mutex<Queue>> = queue.clone();

        // Loop through queue
        loop {
            // Lock Mutex
            let mut guard: MutexGuard<Queue> = thread_queue.lock().unwrap();

            if !guard.data.is_empty() & !guard.grant {
                // Extract tuple
                let (message, mut stream) = guard.data.pop_front().unwrap();

                // Handle process message
                match message.message_type {
                    MessageType::Request => {
                        // Create Grant Message
                        let write_string = Message::new(
                            MessageType::Grant,
                            0
                        ).to_string();

                        // Prepare write buffer
                        let write_buffer: &[u8] = write_string.as_bytes();

                        // Flush stream
                        stream.flush().unwrap();

                        // Write to stream
                        let _w = match stream.write(&write_buffer) {
                            Ok(value) => {
                                info!("[S] {:7} - {}", "Grant", message.process_id);
                                let current_record = guard.record.get_mut(&message.process_id);
                                match current_record {
                                    Some(current_record) => {
                                        *current_record += 1;
                                    },
                                    None => {
                                        guard.record.insert(
                                            message.process_id,
                                            1
                                        );
                                    }
                                };
                                guard.grant = true;
                                value
                            },
                            Err(error) => {
                                info!("Unable to write into socket: {:?}", error);
                                panic!("Unable to write into socket: {:?}", error)
                            },
                        };
                    },
                    MessageType::Release => (), // Handled at socket handler
                    MessageType::Grant => () // Process can't send Grant
                }
            }

            // Release Mutex
            drop(guard);
        }
    });

    // Await for the end of execution of the coordinator
    coordinator_handle.await.unwrap();
}
