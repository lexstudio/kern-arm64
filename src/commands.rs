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
        cmd if cmd.starts_with("vi") => {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            let file_path = if parts.len() > 1 { Some(parts[1]) } else { None };
            open_vi(file_path); // Pass the file path to open_vi
        }
        cmd if cmd.starts_with("mkd") => create_directory(cmd),
        cmd if cmd.starts_with("mkf") => create_file(cmd),
        cmd if cmd.starts_with("rmv") => remove_item(cmd),
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

fn open_vi(file_path: Option<&str>) {
    let mut command = Command::new("vi");

    if let Some(path) = file_path {
        command.arg(path); // Add the file path as an argument
    }

    match command.spawn() {
        Ok(mut child) => {
            child.wait().expect("Failed to wait on child");
        }
        Err(err) => {
            match Command::new("sh")
                .arg("-c")
                .arg("whereis vi | cut -d' ' -f2 | xargs vi")
                .spawn()
            {
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

fn create_directory(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.len() < 2 {
        println!("Usage: mkd <directory_name>");
        return;
    }

    let dir_name = parts[1];
    let current_dir = env::current_dir().unwrap();  // Get current directory
    let new_dir_path = current_dir.join(dir_name);

    // Check if directory already exists
    if new_dir_path.exists() {
        println!("Directory '{}' already exists.", dir_name);
        return;
    }

    // Create directory with error handling
    match fs::create_dir(&new_dir_path) {
        Ok(_) => println!("Directory '{}' created successfully.", dir_name),
        Err(err) => match err.kind() {
            std::io::ErrorKind::PermissionDenied => {
                println!("Error: Permission denied to create directory.");
            }
            _ => println!("Error creating directory: {}", err),
        },
    }
}

// ... (other imports)


// ... (other functions: open_vi, change_directory, create_directory)

fn create_file(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.len() < 2 {
        println!("Usage: mkf <file_name>");
        return;
    }

    let file_name = parts[1];
    let current_dir = env::current_dir().unwrap();
    let new_file_path = current_dir.join(file_name);

    // Check if file already exists
    if new_file_path.exists() {
        println!("File '{}' already exists.", file_name);
        return;
    }

    // Create file with error handling
    match File::create(&new_file_path) {
        Ok(_) => println!("File '{}' created successfully.", file_name),
        Err(err) => match err.kind() {
            std::io::ErrorKind::PermissionDenied => {
                println!("Error: Permission denied to create file.");
            }
            _ => println!("Error creating file: {}", err),
        },
    }
}

fn remove_item(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.len() < 2 {
        println!("Usage: rm <file_or_directory_name>");
        return;
    }

    let item_name = parts[1];
    let current_dir = env::current_dir().unwrap();
    let item_path = current_dir.join(item_name);

    // Check if item exists
    if !item_path.exists() {
        println!("Item '{}' does not exist.", item_name);
        return;
    }

    // Remove file or directory based on type
    if item_path.is_file() {
        match fs::remove_file(&item_path) {
            Ok(_) => println!("File '{}' removed successfully.", item_name),
            Err(err) => println!("Error removing file: {}", err),
        }
    } else if item_path.is_dir() {
        match fs::remove_dir_all(&item_path) {
            Ok(_) => println!("Directory '{}' removed successfully.", item_name),
            Err(err) => println!("Error removing directory: {}", err),
        }
    } else {
        println!("Invalid item type.");
    }
}
