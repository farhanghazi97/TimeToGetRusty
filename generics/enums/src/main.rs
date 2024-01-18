// Use of generics in enum definitions

// Reflecting on previous chapters, you might realise
// that we have seen generics used within enums already!

// Recall the following:

/*
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
*/

// Similar to structs, we can define enums to hold
// generic data types in their variants!

// PROTIP: When you recognise situations in your code with
// multiple struct or enum definitions that only differ
// in the types of the values they hold, you can avoid
// code duplication by using generic types instead!

// Let's create our own examples
use std::io::Bytes;

enum FileType {
    PDF,
    DOC,
    DOCX,
}

struct File<T> {
    file_type: FileType,
    size: u32,
    data: T,
}

impl<T> File<T> {
    fn get_content_type(&self) -> &str {
        match self.file_type {
            FileType::DOC => "application/msword",
            FileType::DOCX => {
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            }
            FileType::PDF => "application/pdf",
        }
    }
}

// Constrain methods  to specific generic types

// These methods are only available on file instances
// which have string data!
impl File<&str> {
    fn get_string_data(&self) -> &str {
        &self.data
    }
}

// These methods are only available on file instances
// which have bytes data!
impl File<&[u8]> {
    fn get_bytes_data(&self) -> &[u8] {
        self.data
    }
}

fn main() {
    let file_1 = File {
        file_type: FileType::DOC,
        size: 32,
        data: "abc",
    };

    let file_2 = File {
        file_type: FileType::PDF,
        size: 64,
        data: "hello world".as_bytes(),
    };

    let content_type_1 = file_1.get_content_type();
    println!("The content type of file 1 is: {content_type_1}");
    let data_1 = file_1.get_string_data();
    println!("The data within file 1 is: {data_1}");

    let content_type_2 = file_2.get_content_type();
    println!("The content type of file 2 is: {content_type_2}");
    let data_2 = file_2.get_bytes_data();
    println!("The data within file 2 is: {:?}", data_2);
}
