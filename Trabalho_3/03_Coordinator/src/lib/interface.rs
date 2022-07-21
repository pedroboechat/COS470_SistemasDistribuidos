use std::io::{self, Write};
use std::process::exit;
use std::sync::{Arc, Mutex, MutexGuard};

use log::info;
use tokio::spawn;
use tokio::task::JoinHandle;

use crate::lib::queue::Queue;

/// Create the coordinator's CLI interface
/// ## Arguments
/// * `queue` - Process queue.
pub async fn create(
    queue: Arc<Mutex<Queue>>
) {
    // Spawn thread for the CLI interface
    let cli_handle: JoinHandle<()> = spawn(async move {
        // Thread queue
        let thread_queue: Arc<Mutex<Queue>> = queue.clone();

        loop {
            // Print command options
            print!(
                "\n\nENTER A COMMAND:\n{}\n{}\n{}\n{}",
                "1 - Print queue.",
                "2 - Print process count.",
                "3 - Finish coordinator.",
                "> "
            );

            // Flush STDOUT
            let _ = io::stdout().flush();
    
            // Create STDIN buffer
            let mut command: String = String::new();
            io::stdin().read_line(&mut command).unwrap();
    
            match command.trim() {
                "1" => {
                    // Get current queue without locking
                    let current_queue: MutexGuard<Queue> = thread_queue.lock().unwrap();
                    
                    // Print queue to STDOUT
                    println!("{:?}", current_queue.data);

                    // Flush STDOUT
                    let _ = io::stdout().flush();

                    drop(current_queue);
                },
                "2" => {
                    // Get current queue without locking
                    let current_queue: MutexGuard<Queue> = thread_queue.lock().unwrap();
                    
                    // Print queue record to STDOUT
                    println!("{:?}", current_queue.record);

                    // Flush STDOUT
                    let _ = io::stdout().flush();

                    drop(current_queue);
                },
                "3" => {
                    // Exit with code 0
                    info!("Exiting orchestrator...");
                    exit(0);
                },
                &_ => {
                    println!("Invalid command...");
                }
            }
        }
    });

    // Await for the end of execution of the CLI interface
    cli_handle.await.unwrap();
}
