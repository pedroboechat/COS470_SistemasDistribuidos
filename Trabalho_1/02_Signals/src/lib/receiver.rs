use std::process::exit;
use std::sync::Mutex;
use nix::sys::signal::{self as sigmod, Signal, SigHandler};
use nix::libc::c_int;

/// Handles received signals
/// ## Arguments
/// * `signal` - Signal received.
extern fn signal_handler(signal: c_int) {
    let signal = Signal::try_from(signal).unwrap();
    let _ = match signal {
        Signal::SIGTERM => {
            println!("SIGTERM signal received!");
            exit(0);
        },
        Signal::SIGUSR1 => {
            println!("SIGUSR1 signal received!");
        },
        Signal::SIGUSR2 => {
            println!("SIGUSR2 signal received!");
        },
        _ => (),
    };
}

/// Sets custom signal handler as default for all the supported signals
fn set_signal_handler() -> SigHandler {
    let handler = SigHandler::Handler(signal_handler);
    unsafe {
        for sig in Signal::iterator() {
            if sig == Signal::SIGKILL || sig == Signal::SIGSTOP {
                continue;
            }
            sigmod::signal(sig, handler).unwrap();
        }
    }
    return handler;
}

/// Starts a busy wait on the process, to maintain it active during signal listening
pub fn busy_wait() {
    let _handler = set_signal_handler();
    loop {};
}

/// Starts a blocking wait on the process, to maintain it active during signal listening
pub fn blocking_wait() {
    let _handler = set_signal_handler();
    let mutex = Mutex::new(-1);
    mutex.lock();
}