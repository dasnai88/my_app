use std::fs;
use std::io;

fn main() {
    let file_name = "out.txt";

    /*match write_to_file(file_name, "Hello Rust") {
        Ok(()) => println!("File written"),
        Err(e) => println!("Errors {}", e),
    }*/

    match file_read(file_name) {
        Ok(text) => println!("Text: {}", text),
        Err(e) => println!("Error: {}", e),
    }
}

fn file_read(file_patch: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_patch)
}

fn write_to_file(file_patch: &str, content: &str) -> Result<(), io::Error> {
    fs::write(file_patch, content)
}
