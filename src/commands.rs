use std::fs::{self, File};
use std::io::{self, Error, ErrorKind};
use std::path::Path;
use std::process;
use std::net::TcpStream;
use std::time::UNIX_EPOCH;
use std::fs::DirEntry;
use std::process::Command;
use std::env::current_dir;


// Dir color will be blue, file color will be white
const DIR_COLOR: &str = "\x1b[38;2;69;133;136m";
const FILE_COLOR: &str = "\x1b[38;2;255;255;255m";
const RESET_COLOR: &str = "\x1b[0m";
const RUST_FILE_COLOR: &str = "\x1b[38;2;255;0;0m";
pub fn execute_command(input: &str) {
    match input {
        "ex" => {
            println!("Exiting...");
            process::exit(0);
        }
        "uname" => uname(),
        "crate" => qwe(),
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

