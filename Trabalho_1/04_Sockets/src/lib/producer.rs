use std::io::{Write, Read};
use std::net::{TcpStream, Shutdown};
use rand::{thread_rng, Rng};

pub fn run(address: String, values: i32) {
    let mut stream = match TcpStream::connect(address.as_str()) {
        Ok(value) => value,
        Err(error) => panic!("Unable to bind into {}: {:?}", address.as_str(), error),
    };

    match stream.set_read_timeout(None) {
        Ok(_) => (),
        Err(error) => panic!("Unable to set read timeout: {:?}", error),
    }

    let mut rng = thread_rng();
    let mut random_counter = 0;
    let mut random_number: i64 = 1;
    
    while random_counter < values {
        let mut buffer = [0; 2];
        
        random_number += rng.gen_range(1..100);
        
        let data = random_number.to_be_bytes();
        
        stream.flush().unwrap();
        
        let _w = match stream.write(&data) {
            Ok(value) => value,
            Err(error) => panic!("Unable to write into socket: {:?}", error),
        };
        
        let _r = match stream.read(&mut buffer[..]) {
            Ok(value) => value,
            Err(error) => panic!("Unable to read from socket: {:?}", error),
        };

        let is_prime: bool = i16::from_be_bytes(buffer) > 0;
        
        println!(
            "Iteration: {:4} | Number: {:6} | Is Prime: {:6}\n",
            random_counter + 1,
            random_number,
            is_prime
        );

        random_counter += 1;
    }

    stream.shutdown(Shutdown::Both).unwrap();
}