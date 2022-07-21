use std::fmt;
use std::str::FromStr;

/// Messages type enumeration
#[derive(Copy, Clone, Debug)]
pub enum MessageType {
    Request,
    Grant,
    Release
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MessageType::Request => write!(f, "1"),
            MessageType::Grant   => write!(f, "2"),
            MessageType::Release => write!(f, "3")
        }
    }
}

/// Messages struct
#[derive(Debug)]
pub struct Message {
    pub message_type: MessageType,
    pub process_id: u32
}

/// Messages implementation
impl Message {
    /// Create a message
    /// ## Arguments
    /// * `message_type` - MessageType for the Message.
    /// * `process_id` - Process ID (uid) for the current process.
    pub fn new(message_type: MessageType, process_id: u32) -> Message {
        Message {
            message_type,
            process_id
        }
    }

    /// Convert message to String
    pub fn to_string(&self) -> String {
        format!(
            "{:0<8}",
            format!(
                "{}|{}|",
                self.message_type,
                self.process_id
            )
        )
    }

    /// Create message from String
    /// ## Arguments
    /// * `raw_string` - Raw string to create Message.
    pub fn from_string(raw_string: String) -> Message {
        let split: Vec<&str> = raw_string.split("|").collect();
        let message_type: MessageType = match split[0] {
            "1" => MessageType::Request,
            "2" => MessageType::Grant,
            "3" => MessageType::Release,
            &_ => panic!("Invalid message type")
        };
        let process_id: u32 = u32::from_str(split[1]).unwrap();
        return Message{
            message_type,
            process_id
        }
    }
}
