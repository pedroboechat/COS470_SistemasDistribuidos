use std::process::{exit, id};
use clap::{Command, Arg};

mod lib;
use crate::lib::{sender, receiver};

fn main() {
    let matches = Command::new("Signals")
        .version("1.0.0")
        .author(
            "Henrique Chaves <henriquechaves@poli.ufrj.br>\nPedro Boechat <pedroboechat@poli.ufrj.br>"
        )
        .about("Implements a signal sender and signal receiver")
        .arg(
            Arg::new("sender")
            .exclusive(true)
            .short('s')
            .long("sender")
            .value_names(&["PID", "SIGNAL"])
            .help("Starts the sender program, sending a <SIGNAL> to the process with <PID>")
        )
        .arg(
            Arg::new("busy")
            .exclusive(true)
            .short('z')
            .long("busy-receiver")
            .help("Starts the receiver program with a busy wait")
        )
        .arg(
            Arg::new("blocking")
            .exclusive(true)
            .short('x')
            .long("blocking-receiver")
            .help("Starts the receiver program with a blocking wait")
        )
        .arg_required_else_help(true)
        .get_matches();
    
    if matches.is_present("sender") {
        println!("📤 Signal sender 📤");

        let args: Vec<&str> = matches.values_of("sender").unwrap().collect();

        _ = match sender::send_signal(args[0].to_string(), args[1].to_string()) {
            Ok(_) => println!(
                "Signal sent with success!"
            ),
            Err(error) => panic!(
                "Couldn't send signal: {:?}",
                error
            ),
        };

        exit(0);
        
    } else if matches.is_present("busy") {
        println!("📥 Signal receiver (busy) 📥");
        println!("Receiver's PID: {}", id());

        receiver::busy_wait();
    } else {
        println!("📥 Signal receiver (blocking) 📥");
        println!("Receiver's PID: {}", id());

        receiver::blocking_wait();
    }
}