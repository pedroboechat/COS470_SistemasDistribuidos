use nix::libc;
use nix::unistd::{fork, ForkResult, pipe, close, read, write};
use rand::{thread_rng, Rng};

/// Checks whether a number is prime.
/// 
/// ## Arguments
/// * `x` - Number to check primeness.
fn is_prime(x: i64) -> bool {
    let last = (x as f64).sqrt() as i64 + 1;
    x > 1 && (2..last).all(|d| x % d != 0)
}

pub fn run(values: i32) {
    let mut buffer = [0; 8];
    
    let pipefd = match pipe() {
        Ok(value) => value,
        Err(error) => panic!("Couldn't create pipe: {:?}", error),
    };

    println!("👨 Parent Process 👨");

    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            let mut rng = thread_rng();
            let mut random_counter = 0;
            let mut random_number: i64 = 1;
            println!("[Parent] Created child with PID {}", child);

            match close(pipefd.0) {
                Ok(_) => (),
                Err(error) => panic!("[Parent] Couldn't close read end on pipe: {}", error),
            };

            while random_counter < values {
                random_number += rng.gen_range(1..100);
                println!(
                    "[Parent] Iteration: {:4} | Number: {:6}",
                    random_counter + 1,
                    random_number
                );
                let data = random_number.to_be_bytes();
                _ = match write(pipefd.1, &data) {
                    Ok(_) => (),
                    Err(error) => panic!("[Parent] Couldn't write on pipe: {}", error),
                };
                random_counter += 1;
            }

            println!("[Parent] Iteration has ended: sending '0'...");
            let data = (0 as i64).to_be_bytes();
            _ = match write(pipefd.1, &data) {
                Ok(_) => (),
                Err(error) => panic!("[Parent] Couldn't write on pipe: {}", error),
            };
            
            match close(pipefd.1) {
                Ok(_) => (),
                Err(error) => panic!("[Parent] Couldn't close write end on pipe: {}", error),
            };
        }
        Ok(ForkResult::Child) => {
            match close(pipefd.1) {
                Ok(_) => (),
                Err(error) => panic!("[Child]  Couldn't close write end on pipe: {}", error),
            };
            
            write(
                libc::STDOUT_FILENO,
                format!(
                        "👶 Child  Process 👶\n"
                    ).as_bytes()
            ).ok();

            loop {
                _ = match read(pipefd.0, &mut buffer[..]) {
                    Ok(_) => (),
                    Err(error) => panic!("[Child]  Couldn't read from pipe: {}", error),
                };

                let x: i64 = i64::from_be_bytes(buffer);
                if x == 0 {
                    write(
                        libc::STDOUT_FILENO,
                        format!(
                                "[Child]  Received '0': Exiting child...\n"
                            ).as_bytes()
                    ).ok();
                    break;
                }
                
                let x_is_prime: bool = is_prime(x) as i8 > 0;

                write(
                    libc::STDOUT_FILENO,
                    format!(
                            "[Child]  Number: {:7} | Is Prime: {:6}\n",
                            x,
                            x_is_prime
                        ).as_bytes()
                ).ok();
            }
            
            match close(pipefd.0) {
                Ok(_) => (),
                Err(error) => panic!("[Child]  Couldn't close read end on pipe: {}", error),
            };
            unsafe { libc::_exit(0) };
        }
        Err(_) => println!("[Parent] Fork failed..."),
    }
}