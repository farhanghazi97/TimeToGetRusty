use std::fs::File;
use std::io::{self, Read};

// Pay attention to the return type of this function!
// It is of type Result<String, Error> i.e Ok(String), Err(io:Error)!
fn read_username_file(file_path: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(file_path);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        // If we fail to read the file to begin with, then we
        // immediately return the error and exit the function
        // Note the explicit return here!

        // THIS IS WHERE WE ARE PROPAGATING THE ERROR BACK TO
        // THE CALLING CODE! THIS IS ERROR PROPAGATION IN ACTION!
        Err(error) => return Err(error),
    };
    // If we made it to this point, then we must have opened
    // the file successfully. Nice!
    // Let's create a mutable variable to contain our username
    // (should we find it)
    let mut username = String::new();
    // Just like file open, trying to read the file contents could
    // be successful or produce and error. Thus, we handle either
    // scenario using a "match" statement
    // If we are able to read the file, we return the contents of
    // the file (via Ok()). If there is an error, we return the error
    // via the Err()!
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        // THIS IS WHERE WE ARE PROPAGATING THE ERROR BACK TO
        // THE CALLING CODE! THIS IS ERROR PROPAGATION IN ACTION!
        Err(e) => Err(e),
    }
}

fn main() {
    // The "read_username_from_file()" function demonstrates how we can
    // propagate library errors upto the code that calls it so that the calling
    // code can handle the underlying error.
    let result = read_username_file("./src/hello.txt");
    // Based on the return from "read_username_file()", we then decide how to handle
    // the success / error
    match result {
        Ok(username) => println!("Username read from file: {username}"),
        Err(error) => panic!("Failed to read username from file: {error}"),
    }
}
