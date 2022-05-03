use std::io::{Write, Read};
use std::net::{TcpStream, TcpListener, Shutdown};
use std::process::exit;

fn is_prime(x: i64) -> bool {
    let last = (x as f64).sqrt() as i64 + 1;
    x > 1 && (2..last).all(|d| x % d != 0)
}

fn handler(mut stream: &TcpStream) -> bool {
    let mut buffer = [0; 8];

    let _r = match stream.read(&mut buffer[..]) {
        Ok(value) => value,
        Err(error) => panic!("Unable to read from socket: {:?}", error),
    };

    let x: i64 = i64::from_be_bytes(buffer);

    if x == 0 {
        return false;
    }

    let x_is_prime: i8 = is_prime(x) as i8;

    let response = x_is_prime.to_be_bytes();

    stream.flush().unwrap();

    let _w = match stream.write(&response) {
        Ok(value) => value,
        Err(error) => panic!("Unable to write into socket: {:?}", error),
    };

    println!(
        "Number: {:6} | Is Prime: {:6}\n",
        x,
        x_is_prime > 0
    );

    return true;
}

pub fn run(address: String) {
    let listener = match TcpListener::bind(address.as_str()) {
        Ok(value) => value,
        Err(error) => panic!("Unable to bind into {}: {:?}", address.as_str(), error),
    };

    let (stream, _address) = match listener.accept() {
        Ok(value) => value,
        Err(error) => panic!("Unable to set listener to accept connections: {:?}", error),
    };

    match stream.set_read_timeout(None) {
        Ok(_) => (),
        Err(error) => panic!("Unable to set read timeout: {:?}", error),
    }

    loop {
        if handler(&stream) == false {
            break;
        }
    }

    stream.shutdown(Shutdown::Both).unwrap();
    exit(0);
}
