use std::str::FromStr;

use clap::{Command, Arg};

mod lib;
use crate::lib::pipe;

fn main() {
    let matches = Command::new("Pipes")
        .version("1.0.0")
        .author(
            "Henrique Chaves <henriquechaves@poli.ufrj.br>\nPedro Boechat <pedroboechat@poli.ufrj.br>"
        )
        .about("Implements a producer and a consumer that comunicate through pipes")
        .arg(
            Arg::new("values")
            .exclusive(true)
            .short('v')
            .long("values")
            .value_names(&["VALUES"])
            .help("Starts the program for <VALUES> numbers")
        )
        .arg_required_else_help(true)
        .get_matches();
    
    if matches.is_present("values") {
        let args: Vec<&str> = matches.values_of("values").unwrap().collect();
        pipe::run(FromStr::from_str(args[0]).unwrap());
    }
}
