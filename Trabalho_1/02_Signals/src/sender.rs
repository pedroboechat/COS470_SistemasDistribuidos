#![crate_name = "sender"]

use nix::unistd::Pid;
use nix::sys::signal::{Signal, kill};
use std::io;
use std::str::FromStr;

/// Checks for the existence of a process with a given PID.
/// 
/// ## Arguments
/// * `pid` - PID to be checked.
fn check_pid(pid: Pid) -> bool {
    return match kill(pid, None) {
        Ok(_) => true,
        Err(_) => false,
    };
}

/// Sends signal to a process with given PID.
/// 
/// ## Arguments
/// * `pid` - PID to send the signal to.
/// * `signal` - Signal to be sent.
fn send_signal(pid: Pid, signal: Signal) {
    return match kill(pid, signal) {
        Ok(_) => println!(
            "Signal sent with success!"
        ),
        Err(error) => panic!(
            "Couldn't send signal: {:?}",
            error
        ),
    };
}

fn main() {
    println!("📤 Signal sender 📤");
    
    println!("PID: ");
    let parsed_pid = 6699;

    println!("Signal: ");
    let parsed_signal = "siGteRm".to_uppercase();

    let pid = Pid::from_raw(parsed_pid);
    let signal = match Signal::from_str(parsed_signal.as_str()) {
        Ok(value) => value,
        Err(_) => panic!(
            "Signal {} is invalid.",
            parsed_signal
        ),
    };

    if check_pid(pid) {
        send_signal(pid, signal);
    } else {
        panic!(
            "PID {} does not exist.",
            pid
        );
    }
}
