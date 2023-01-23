use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    // To handle errors in Rust, one uses the Result enum.
    // The Result enum is an enum with two type variants: Ok and Err.
    // Unlike other languages, Rust does not have exception throwing.
    // A function which can fail, returns the Result enum,
    // one has to check the type of the returned Result and act accordingly.
    // Checking the type is usually done with pattern matching or an if let statement.
    // Rust forces one indirectly to handle errors. This means that one will not be able
    // to use the value returned by a function without unpacking the Result enum first.
    // Therefore, you can have unhandled errors, but you will not be able to use
    // the value returned by the function and the compiler will produce a warning.

    // Try to modify the code and see how the compiler reacts.
    // You might not be able to compile the code anymore.
    // Try the following:
    // 1. Comment out the if let statement and
    //    comment in the match statement below.
    // 2. Comment in the print_file_line_by_line function call below.
    // 3. Put the ? operator after the print_file_line_by_line function call
    //    and see if you can make the code compile again.
    //    Hint: You will have to change the return type of the function
    //    and add a Ok(()) at the end of the function.

    // This is a simple check if the type which is returned by print_file_line_by_line
    // is of type Err. If it is, the error is printed to the console.
    // The same will work with Ok.
    // Another way to do this is with pattern matching. Which is done below.
    if let Err(e) = print_file_line_by_line("./README.md") {
        // Print the error.
        println!("Error: {}", e);
    }

    // This is a example of pattern matching.
    // It is more verbose than the if let statement above.
    // If the print_file_line_by_line function returns Ok, the program will print
    // "Everything went well". If it returns Err, the program will print the error.

    // match print_file_line_by_line("../README.md") {
    //     Ok(_) => println!("Everything went well"),
    //     Err(e) => println!("Error: {}", e),
    // };

    // This will work as well, but the compiler will produce a warning
    // print_file_line_by_line("../README.md");

    // This example shows only a few ways handle errors.
    // The main takeaway is that compared to other languages,
    // Rust forces one to handle errors at compile time.
    // This increases the reliability of the code and adds safety.
}

fn print_file_line_by_line(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Don't worry about the Box<dyn std::error::Error> part.
    // It just means that the function can return any type of error and
    // put it in a box, which lives on the heap.
    // If that means nothing for you, just ignore it.
    // To understand it better, you can just think of the return type as:
    // Result<void, Error>

    // Try to remove the ? operator and see what happens.
    // One will not be able to use the value returned by the function
    // if the Result enum is not handled first.
    // The ? is used for returning early if an error is encountered
    // It only works on the Result enum.
    // It is a shorthand for the match statement below.

    // Open the file and prepare it for reading.
    let f = File::open(file)?; // <-- This is the ? operator.
    let mut file = BufReader::new(&f);

    // Read the file into a string.
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?; // <-- Here is another ? operator.
                                              // One could rewrite the above line as:

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

    // Return the Ok variant of the Result enum.
    // The "()" is the unit type. It is like void in other languages.
    Ok(())
}
