use std::str::FromStr;

use clap::{Command, Arg};
use dotenv;

mod lib;
use crate::lib::{producer, consumer};

fn main() {
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

    let matches = Command::new("Sockets")
        .version("1.0.0")
        .author(
            "Henrique Chaves <henriquechaves@poli.ufrj.br>\nPedro Boechat <pedroboechat@poli.ufrj.br>"
        )
        .about("Implements a socket producer and socket consumer")
        .arg(
            Arg::new("producer")
            .exclusive(true)
            .short('p')
            .long("producer")
            .value_names(&["VALUES"])
            .help("Starts the producer program for <VALUES> numbers")
        )
        .arg(
            Arg::new("consumer")
            .exclusive(true)
            .short('c')
            .long("consumer")
            .help("Starts the consumer program")
        )
        .arg_required_else_help(true)
        .get_matches();
    
    if matches.is_present("producer") {
        let args: Vec<&str> = matches.values_of("producer").unwrap().collect();
        println!("📤 Producer Program 📤");
        producer::run(binding_address, FromStr::from_str(args[0]).unwrap());
    } else {
        println!("📤 Consumer Program 📤");
        consumer::run(binding_address);
    }
}
