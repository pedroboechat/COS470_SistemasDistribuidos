use std::str::FromStr;

use clap::{Command, Arg};

mod lib;
use crate::lib::{spinlock, adder};

#[tokio::main(flavor="multi_thread")]
async fn main() {
    let matches = Command::new("Spinlocks")
        .version("1.0.0")
        .author(
            "Pedro Boechat <pedroboechat@poli.ufrj.br>"
        )
        .about("Implements an adder with spinlocks")
        .arg(
            Arg::new("numbers")
            .exclusive(false)
            .required(true)
            .short('n')
            .long("numbers")
            .value_names(&["N"])
            .help("Sets the amount of random numbers to <N>")
        )
        .arg(
            Arg::new("threads")
            .exclusive(false)
            .required(true)
            .short('k')
            .long("threads")
            .value_names(&["K"])
            .help("Sets the amount used threads to <K>")
        )
        .arg(
            Arg::new("times")
            .exclusive(false)
            .required(false)
            .short('t')
            .long("times")
            .value_names(&["T"])
            .default_value("1")
            .help("Runs the program <T> times")
        )
        .arg_required_else_help(true)
        .get_matches();
    
    if matches.is_present("numbers") {
        if matches.is_present("threads") {
            let n_args: Vec<&str> = matches.values_of("numbers").unwrap().collect();
            let k_args: Vec<&str> = matches.values_of("threads").unwrap().collect();
            let t_args: Vec<&str> = matches.values_of("times").unwrap().collect();
            let n: usize = usize::from_str(n_args[0]).unwrap();
            let k: usize = usize::from_str(k_args[0]).unwrap();
            let t: usize = usize::from_str(t_args[0]).unwrap();

            println!(
                "N = {}, K = {}",
                n, k
            );

            let mut elapsed_times: Vec<u128> = Vec::with_capacity(t);

            for run in 1..=t {
                print!("Run #{} -> ", run);
                let elapsed: u128 = adder::run(n, k).await;
                elapsed_times.push(elapsed);
            }

            println!("\nAll elapsed times:\n{:?}", elapsed_times);
        }
        else {
            panic!("The argument `threads` was not specified");
        }
    } else {
        panic!("The argument `numbers` was not specified");
    }
}