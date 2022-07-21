use std::collections::{HashMap, VecDeque};
use std::net::TcpStream;

use crate::lib::messages::Message;

/// Process queue struct
pub struct Queue {
    pub data: VecDeque<(Message, TcpStream)>,
    pub record: HashMap<u32, u8>,
    pub grant: bool
}

/// Process queue implementation
impl Queue {
    /// Create a new process queue
    pub fn new() -> Queue {
        Queue {
            data: VecDeque::new(),
            record: HashMap::new(),
            grant: false
        }
    }
}