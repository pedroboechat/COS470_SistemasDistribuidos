use std::io::{Write, Read};
use std::net::{TcpStream, Shutdown};
use std::str;
use std::time::Duration;

use log::info;
use tokio::time::sleep;

use crate::lib::messages::*;

/// Create the process
/// ## Arguments
/// * `address` - Binding address for the socket.
/// * `sleep_time` - Sleep time.
/// * `repetitions` - Repetitions.
pub async fn create(
    address: String,
    process_id: u32,
    sleep_time: Duration,
    repetitions: usize
) {
    // Create TcpStream with coordinator socket
    let mut stream = match TcpStream::connect(address.as_str()) {
        Ok(value) => value,
        Err(error) => {
            info!("Unable to bind into {}: {:?}", address.as_str(), error);
            panic!("Unable to bind into {}: {:?}", address.as_str(), error)
        },
    };

    // Create Request Message
    let request_msg: String = Message::new(MessageType::Request, process_id).to_string();
    
    // Prepare Request write buffer
    let request_write_buffer: &[u8] = request_msg.as_bytes();
    
    // Create Release Message
    let release_msg: String = Message::new(MessageType::Release, process_id).to_string();
    
    // Prepare Release write buffer
    let release_write_buffer: &[u8] = release_msg.as_bytes();

    // Loop for each repetition
    for _ in 0..repetitions {
        info!("{:?}", process_id);

        // Initialize read buffer
        let mut read_buffer: [u8; 8] = [0; 8];

        // Flush stream
        stream.flush().unwrap();

        // Write Request Message to stream
        let _w_rq = match stream.write(&request_write_buffer) {
            Ok(value) => value,
            Err(error) => {
                info!("Unable to write into socket: {:?}", error);
                panic!("Unable to write into socket: {:?}", error)
            },
        };

        // Read Grant Message from stream
        let _r = match stream.read(&mut read_buffer[..]) {
            Ok(value) => value,
            Err(error) => {
                info!("Unable to read from socket: {:?}", error);
                panic!("Unable to read from socket: {:?}", error)
            },
        };

        // Convert Buffer to Message raw string
        let raw_string: String = str::from_utf8(&read_buffer).unwrap().to_string();

        // Convert Message raw string to Message
        let message: Message = Message::from_string(raw_string);

        // Check if Grant
        match message.message_type {
            MessageType::Request => {
                info!("Process can't receive Request Message");
                panic!("Process can't receive Request Message")
            },
            MessageType::Release => {
                info!("Process can't receive Release Message");
                panic!("Process can't receive Release Message")
            },
            MessageType::Grant => ()
        }

        // Flush stream
        stream.flush().unwrap();

        // Write Release Message to stream
        let _w_rl = match stream.write(&release_write_buffer) {
            Ok(value) => value,
            Err(error) => {
                info!("Unable to write into socket: {:?}", error);
                panic!("Unable to write into socket: {:?}", error)
            },
        };

        // Sleep for a while
        sleep(sleep_time).await;
    }

    stream.shutdown(Shutdown::Both).unwrap();
}
