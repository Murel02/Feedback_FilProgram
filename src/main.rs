use std::fs::{File, OpenOptions, remove_file, read_to_string};
use std::io::{self, Write};

fn main() {
    let filename = "data.txt";

    loop {
        println!("\nChoose an action:");
        println!("1. Create and write to file");
        println!("2. Read file contents");
        println!("3. Append text to file");
        println!("4. Clear file contents");
        println!("5. Delete file");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => write_to_file(filename),
            "2" => read_from_file(filename),
            "3" => append_to_file(filename),
            "4" => clear_file(filename),
            "5" => delete_file(filename),
            "6" => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

// Function to create (and overwrite) the file
fn write_to_file(filename: &str) {
    let mut file = File::create(filename).expect("Could not create file");
    let text = "Welcome to Rust file handling!\n";
    file.write_all(text.as_bytes()).expect("Could not write to file");
    println!("File created and text written.");
}

// Function to read and display file contents
fn read_from_file(filename: &str) {
    match read_to_string(filename) {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(_) => println!("Could not read the file. Maybe it does not exist?"),
    }
}

// Function to append text without overwriting
fn append_to_file(filename: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .expect("Could not open file");

    println!("Enter the text you want to append:");
    let mut new_text = String::new();
    io::stdin().read_line(&mut new_text).expect("Failed to read input");

    file.write_all(new_text.as_bytes()).expect("Could not append text");
    println!("Text appended.");
}

// Function to clear file contents
fn clear_file(filename: &str) {
    File::create(filename).expect("Could not clear file");
    println!("File contents have been cleared.");
}

// Function to delete the file completely
fn delete_file(filename: &str) {
    match remove_file(filename) {
        Ok(_) => println!("File deleted."),
        Err(_) => println!("Could not delete the file. Maybe it does not exist?"),
    }
}
