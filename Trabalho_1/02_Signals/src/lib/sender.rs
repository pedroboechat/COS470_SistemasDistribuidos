use std::str::FromStr;
use nix::unistd::Pid;
use nix::errno::Errno;
use nix::sys::signal::{Signal, kill};

/// Sends signal to a process with given PID.
/// 
/// ## Arguments
/// * `pid` - PID to send the signal to.
/// * `signal` - Signal to be sent.
pub fn send_signal(raw_pid: String, raw_signal: String) -> Result<(), Errno> {
    
    let parsed_pid: i32 = match FromStr::from_str(raw_pid.as_str()) {
        Ok(value) => value,
        Err(_) => panic!(
            "Informed PID is not a integer."
        ),
    };
    let parsed_signal: String = raw_signal.to_uppercase();

    let pid: Pid = Pid::from_raw(parsed_pid);
    let signal: Signal = match Signal::from_str(parsed_signal.as_str()) {
        Ok(value) => value,
        Err(_) => panic!(
            "Signal {} is invalid.",
            parsed_signal
        ),
    };
    
    return match kill(pid, None) {
        Ok(_) => kill(pid, signal),
        Err(_) => panic!(
            "PID {} does not exist.",
            pid
        ),
    };
}
