use crate::commands::execute_command;
use std::io::{self, Write};
use rand::Rng;  // Import the Rng trait to use random number generation methods.
use std::process::Command;
use std::thread::sleep;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::thread;

mod commands; // This line includes the commands module you've just created.

 // Import the function to execute commands.
pub fn sys() {
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape codes for clearing screen
    // if up arrow key is pressed, the previous command is displayed
    // if down arrow key is pressed, the next command is displayed
    
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\nError: Ctrl+C pressed! Restarting...");
        // No change to any running flag, just an informative message.
    }).expect("Error setting Ctrl+C handler");

    while running.load(Ordering::SeqCst) {
        print!("kern-arm64# ");
        io::stdout().flush().unwrap();  // Ensure the prompt is displayed immediately.

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim(); // Trim newline characters.
                execute_command(input); // Execute the command using the function from commands.rs
            },
            Err(error) => println!("Error reading line: {}", error),
        }
    }
}
fn main() {
    println!("\x1B[2J\x1B[1;1H"); // ANSI escape codes for clearing screen
    println!("BIOS: Power on self test...");
    sleep(std::time::Duration::from_millis(010));

    println!("BIOS: Checking hardware integrity...");
    sleep(std::time::Duration::from_millis(010));

    println!("BIOS: Detecting storage devices...");
    sleep(std::time::Duration::from_millis(010));

    println!("BIOS: Hardware check passed. Loading bootloader...");
    sleep(std::time::Duration::from_millis(010));
    println!("Bootloader: Initializing...");
    sleep(std::time::Duration::from_millis(010));  
    println!("Bootloader: Checking file system...");
    sleep(std::time::Duration::from_millis(010));
    println!("Bootloader: Loading kernel into memory...");
    sleep(std::time::Duration::from_millis(010));
    println!("Kernel: Starting system services...");
    sleep(std::time::Duration::from_millis(010));
   
    println!("Kernel: Mounting file systems...");
    sleep(std::time::Duration::from_millis(010));
    println!("interface");
    sleep(std::time::Duration::from_millis(010));
    println!("kern load");
    sleep(std::time::Duration::from_millis(010));
    println!("kern hello!");
    sleep(std::time::Duration::from_millis(010));
       sys();
}
