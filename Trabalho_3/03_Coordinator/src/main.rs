use std::path::PathBuf;
use std::env::current_exe;

use clap::{Command, Arg};
use dotenv;
use log::info;
use log4rs::Handle as LogHandle;

mod lib;
use crate::lib::{orchestrator, logger};

#[tokio::main(flavor="multi_thread")]
async fn main() {
    let mut conf_path: PathBuf = current_exe().unwrap();
    conf_path.pop();
    conf_path.pop();
    conf_path.pop();
    conf_path.push("bindings");
    conf_path.set_extension("conf");

    _ = match dotenv::from_path(conf_path) {
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
    
    let matches = Command::new("Coordinator")
        .version("1.0.0")
        .author(
            "Pedro Boechat <pedroboechat@poli.ufrj.br>"
        )
        .about("Implements the coordinator for a distributed mutual exclusion example with a centralized algorithm")
        .arg(
            Arg::new("logfile")
            .exclusive(false)
            .required(false)
            .short('l')
            .long("logfile")
            .value_names(&["L"])
            .default_value("coordenador")
            .help("Sets the name of the log filename to <L>")
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
        let log_filename: &str = logfile_args[0];
        let _log_handle: LogHandle = logger::create_logger(log_filename);
        info!("Running orchestrator...");
        orchestrator::run(binding_address).await;
    } else {
        panic!("The argument `logfile` was not specified");
    }
}