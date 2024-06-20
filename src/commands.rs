use std::fs::{self, File};
use std::io::{self, Error, ErrorKind};
use std::path::Path;
use std::process;
use std::net::TcpStream;
use std::time::UNIX_EPOCH;
use std::fs::DirEntry;
use std::process::Command;
use std::env::current_dir;
use std::env;

// Dir color will be blue, file color will be white

pub fn execute_command(input: &str) {
    match input {
        "ex" => {
            println!("Exiting...");
            process::exit(0);
        }
        "uname" => uname(),
        "crate" => qwe(),
        "ls" => list_file(),
        cmd if cmd.starts_with("cd") => change_directory(cmd),
        cmd if cmd.starts_with("vi") => open_vi(),
        _ => println!("???"),
        

    }
}
// Keep the rest of your functions here
fn uname() {
    println!("kern-arm64 0.0.1");
    println!("made by lex-studio");
}
fn qwe() {
    println!("CONFIG crate");
    println!("crate kern-arm64")
}
fn list_file() {
    let path = current_dir().unwrap();
    let mut entries = fs::read_dir(path).unwrap();
    while let Some(entry) = entries.next() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            println!("\x1b[34m{}\x1b[0m", entry.file_name().to_str().unwrap());
        } else {
            println!("\x1b[37m{}\x1b[0m", entry.file_name().to_str().unwrap());
        }
    }
}
fn change_directory(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.len() < 2 {
        println!("Usage: cd <directory>");
        return;
    }

    let dir = parts[1];

    // Handle the special case of `cd ..`
    if dir == ".." {
        let current_dir = std::env::current_dir().unwrap();
        let restricted_path = Path::new("/home/lex");

        if current_dir == restricted_path {
            println!("You are already in root directory");
        } else {
            if let Some(parent) = current_dir.parent() {
                if std::env::set_current_dir(parent).is_ok() {
                    // Optionally print the new directory
                    // println!("{}", parent.display()); 
                } else {
                    println!("Failed");
                }
            } else {
                println!("Failed");
            }
        }
    } else { // Handle other directories
        let target_path = Path::new(dir);
        let allowed_prefix = Path::new("/home/lex");
        let is_absolute = target_path.is_absolute();
        
        if is_absolute && !target_path.starts_with(allowed_prefix) {
            println!("directory doesnt exist");
        } else {
            if std::env::set_current_dir(dir).is_ok() {
                // Optionally print the new directory
                // println!("{}", dir); 
            } else {
                println!("Failed to change directory to {}", dir);
            }
        }
    }
}
// vi command that start vi editor with path of file after vi Command


fn open_vi() {
    // Attempt to open vi directly (no path restrictions)
    match Command::new("vi").spawn() {
        Ok(mut child) => {
            // Optionally wait for the child process to finish
            child.wait().expect("Failed to wait on child"); 
        }
        Err(err) => {
            // If vi is not found, try to run it with `whereis vi`
            match Command::new("sh").arg("-c").arg("whereis vi | cut -d' ' -f2 | xargs vi").spawn() {
                Ok(mut child) => {
                    child.wait().expect("Failed to wait on child");
                }
                Err(err) => {
                    println!("Error: could not find or execute vi: {}", err);
                }
            }
        }
    }
}
