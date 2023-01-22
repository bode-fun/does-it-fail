use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    // You can check if a function returns an Error and act accordingly
    if let Err(e) = print_file_line_by_line("../README.md") {
        println!("Error: {}", e);
    }
}

fn print_file_line_by_line(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file and create a buffered reader
    // Try to remove the ? operator and see what happens
    // The program will not compile if a possible error is not handled
    // The ? is used for returning early if an error is encountered
    // It works on the Result type, Rust's way of representing errors
    let f = File::open(file)?;
    let mut file = BufReader::new(&f);

    // Read the file into a string
    let mut file_contents = String::new();
    // Here we use the ? operator again.
    file.read_to_string(&mut file_contents)?;
    
    // I could rewrite the above line as:
    // match file.read_to_string(&mut file_contents) {
    //     Ok(_) => (), // Do nothing if everything went well
    //     Err(e) => return Err(Box::new(e)), // Return early if an error is encountered
    // }
    // But the ? operator is much more concise

    // Split the string into lines
    let lines = file_contents.split("\n").collect::<Vec<&str>>();
    // Print each line
    for line in lines {
        println!("{line}");
    }

    // Return Ok if everything went well
    Ok(())
}
