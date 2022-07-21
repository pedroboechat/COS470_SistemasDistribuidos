use std::str::FromStr;

use clap::{Command, Arg};
use dotenv;
use log4rs::Handle as LogHandle;

mod lib;
use crate::lib::{orchestrator, logger};

#[tokio::main(flavor="multi_thread")]
async fn main() {
    _ = match dotenv::from_filename("./bindings.conf") {
        Ok(value) => value,
        Err(error) => panic!(
            "Couldn't read binding configuration: {:?}",
            error
        ),
    };
    let binding_address: String = match dotenv::var("BINDING_ADDRESS") {
        Ok(value) => value,
        Err(error) => panic!(
            "Couldn't read BINDING_ADDRESS binding configuration: {:?}",
            error
        ),
    };
    
    let matches = Command::new("Process")
        .version("1.0.0")
        .author(
            "Pedro Boechat <pedroboechat@poli.ufrj.br>"
        )
        .about("Implements the process for a distributed mutual exclusion example with a centralized algorithm")
        .arg(
            Arg::new("logfile")
            .exclusive(false)
            .required(false)
            .short('l')
            .long("logfile")
            .value_names(&["L"])
            .default_value("resultado")
            .help("Sets the name of the log filename to <L>")
        )
        .arg(
            Arg::new("processes")
            .exclusive(false)
            .required(false)
            .short('n')
            .long("processes")
            .value_names(&["N"])
            .default_value("1")
            .help("Sets the number of processes to <N>")
        )
        .arg(
            Arg::new("sleep")
            .exclusive(false)
            .required(false)
            .short('k')
            .long("sleep")
            .value_names(&["K"])
            .default_value("1")
            .help("Sets the sleep time to <K> seconds")
        )
        .arg(
            Arg::new("repetitions")
            .exclusive(false)
            .required(false)
            .short('r')
            .long("repetitions")
            .value_names(&["R"])
            .default_value("1")
            .help("Sets the number of repetitions to <R>")
        )
        .arg(
            Arg::new("default")
            .exclusive(false)
            .required(false)
            .short('d')
            .long("default")
            .help("Run with default options")
        )
        .arg_required_else_help(true)
        .get_matches();
    
    if matches.is_present("logfile") {        
        let logfile_args: Vec<&str> = matches.values_of("logfile").unwrap().collect();
        let n_args: Vec<&str> = matches.values_of("processes").unwrap().collect();
        let k_args: Vec<&str> = matches.values_of("sleep").unwrap().collect();
        let r_args: Vec<&str> = matches.values_of("repetitions").unwrap().collect();
        let log_filename: &str = logfile_args[0];
        let n: usize = usize::from_str(n_args[0]).unwrap();
        let k: usize = usize::from_str(k_args[0]).unwrap();
        let r: usize = usize::from_str(r_args[0]).unwrap();

        let _log_handle: LogHandle = logger::create_logger(log_filename);

        orchestrator::run(binding_address, n, k, r).await;
    } else {
        panic!("The argument `resultfile` was not specified");
    }
}