use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // The operation below attempts to open a file.
    // Note the return type of the open() method.
    // It is of type Result<File, Error>

    // As you might recall, the Result enum looks like this:
    // enum Result<T, E> {
    //      Ok(T),
    //      Err(E),
    // }

    // The generic parameter T has been filled by the implementation
    // of File::open with the type of the success value,
    // std::fs::File, which is a file handle forwarded to the
    // Ok() variant of the Result.

    // The generic parameter of E has been provided by std::io::Error and
    // is forwarded to the Err() variant of the Result.

    // Based on the result of the file open operation, we can take different
    // actions by utilisting the familiar "match" statement!
    let file_result_1 = File::open("hello.txt");
    let _file_1 = match file_result_1 {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    // The above scenario will panic! no matter why the File::open failed.
    // However, in reality, it could have failed for a number of reasons
    // such as the file not existing or the user not having permissions
    // to open the file. Hence, we can drill down further into the reason
    // for failure and take actions based on failure reasons!
    // Let's take a look!
    let file_result_2 = File::open("world.txt");
    let _file_2 = match file_result_2 {
        Ok(file) => file,
        // The standard library provides std::io::Error which has the ErrorKind enum
        // defined, which has a list of variants capturing general categories of
        // I/O errors!
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("word.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    // The above is pretty cool and the "match" works well enough but it can
    // be a bit verbose and dosen't always communicate intent well.
    // A shorthand way of doing the above is to leverage the .expect() method
    // allows us to choose the panic! error message.
    let _file_result_3 = File::open("Rusty.txt").expect("Could not open the file!");
    // In production-quality code, most Rustaceans choose expect() rather than
    // unwrap() to give more context about why the operation is expected to always
    // succeed.
}
