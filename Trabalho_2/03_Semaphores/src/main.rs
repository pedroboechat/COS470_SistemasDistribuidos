use std::str::FromStr;

use clap::{Command, Arg};

mod lib;
use crate::lib::orchestrator;

#[tokio::main(flavor="multi_thread")]
async fn main() {
    let matches = Command::new("Semaphores")
        .version("1.0.0")
        .author(
            "Pedro Boechat <pedroboechat@poli.ufrj.br>"
        )
        .about("Implements a producer-consumer with semaphores")
        .arg(
            Arg::new("memory")
            .exclusive(false)
            .required(true)
            .short('n')
            .long("memory")
            .value_names(&["N"])
            .help("Sets the amount of shared memory to <N>")
        )
        .arg(
            Arg::new("producer")
            .exclusive(false)
            .required(true)
            .short('p')
            .long("producer")
            .value_names(&["P"])
            .help("Sets the amount of producer threads to <P>")
        )
        .arg(
            Arg::new("consumer")
            .exclusive(false)
            .required(true)
            .short('c')
            .long("consumer")
            .value_names(&["C"])
            .help("Sets the amount of consumer threads to <C>")
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
    
    if matches.is_present("memory") {
        if matches.is_present("producer") {
            if matches.is_present("consumer") {
                let n_args: Vec<&str> = matches.values_of("memory").unwrap().collect();
                let p_args: Vec<&str> = matches.values_of("producer").unwrap().collect();
                let c_args: Vec<&str> = matches.values_of("consumer").unwrap().collect();
                let t_args: Vec<&str> = matches.values_of("times").unwrap().collect();
                let n: usize = usize::from_str(n_args[0]).unwrap();
                let p: usize = usize::from_str(p_args[0]).unwrap();
                let c: usize = usize::from_str(c_args[0]).unwrap();
                let t: usize = usize::from_str(t_args[0]).unwrap();

                println!(
                    "Producer threads = {}, Consumer threads = {}",
                    p, c
                );

                let mut elapsed_times: Vec<u128> = Vec::with_capacity(t);

                for run in 1..=t {
                    print!("Run #{} -> ", run);
                    let elapsed: u128 = orchestrator::run(n, p, c).await;
                    elapsed_times.push(elapsed);
                }

                println!("\nAll elapsed times:\n{:?}", elapsed_times);
            }
            else {
                panic!("The argument `consumer` was not specified");
            }
        }
        else {
            panic!("The argument `producer` was not specified");
        }
    } else {
        panic!("The argument `memory` was not specified");
    }
}