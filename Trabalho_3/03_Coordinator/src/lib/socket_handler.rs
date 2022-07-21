use std::io::Read;
use std::net::{TcpStream, TcpListener};
use std::str;
use std::sync::{Arc, Mutex, MutexGuard};

use log::info;
use tokio::spawn;
use tokio::task::JoinHandle;

use crate::lib::queue::Queue;
use crate::lib::messages::*;

fn handler(mut stream: TcpStream, thread_queue: Arc<Mutex<Queue>>) {
    // Deactivating read timeout
    let _ = stream.set_read_timeout(None);

    // Loop until stream shutdown
    loop {
        // Initialize read buffer
        let mut read_buffer: [u8; 8] = [0; 8];

        // Blocking read from stream into buffer
        let _r = match stream.read(&mut read_buffer[..]) {
            Ok(value) => {
                if value == 0 {
                    break;
                }
                else {
                    value
                }
            },
            Err(error) => {
                info!("Unable to read from socket: {:?}", error);
                panic!("Unable to read from socket: {:?}", error)
            },
        };

        // Convert Buffer to Message raw string
        let raw_string: String = str::from_utf8(&read_buffer).unwrap().to_string();

        // Convert Message raw string to Message
        let message: Message = Message::from_string(raw_string);

        // Clone stream for the queue
        let queue_stream: TcpStream = stream.try_clone().unwrap();

        // Lock Mutex
        let mut guard: MutexGuard<Queue> = thread_queue.lock().unwrap();

        // Check for message type
        match message.message_type {
            MessageType::Request => {
                info!("[R] {:7} - {}", "Request", message.process_id);

                // Add (Message, TcpStream) to queue
                guard.data.push_back((
                    message,
                    queue_stream
                ));
            },
            MessageType::Release => {
                info!("[R] {:7} - {}", "Release", message.process_id);

                // Release Grant
                guard.grant = false;
            },
            MessageType::Grant => () // Process can't send Grant
        }

        

        // Release Mutex
        drop(guard);
    }
}

/// Function that creates the coordinator
pub async fn create(
    address: String,
    queue: Arc<Mutex<Queue>>
) {
    // Create and bind listener
    let listener = match TcpListener::bind(address.as_str()) {
        Ok(value) => value,
        Err(error) => {
            info!(
                "Unable to bind into {}: {:?}",
                address.as_str(),
                error
            );
            panic!(
                "Unable to bind into {}: {:?}",
                address.as_str(),
                error
            )
        },
    };

    // Vector to store producer thread handles
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // Listen for incoming streams
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let thread_queue: Arc<Mutex<Queue>> = queue.clone();
                handles.push(
                    spawn(async move { handler(stream, thread_queue) } )
                );
            },
            Err(error) => {
                info!(
                    "Unable to set listener to accept connections: {:?}",
                    error
                );
                panic!(
                    "Unable to set listener to accept connections: {:?}",
                    error
                )
            },
        };
    }

    // For-loop through all the producer thread handles
    for handle in handles {
        // Awaits for the thread to finish running
        handle.await.unwrap();
    }
}
