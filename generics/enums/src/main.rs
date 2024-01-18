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
#[derive(Debug)]
enum MSDocType {
    DOC,
    DOCX,
    XLSX,
}

#[derive(Debug)]
enum FileTypes {
    Microsoft(MSDocType),
    PDF,
}

struct File<T> {
    file_type: FileTypes,
    size: u32,
    data: T,
}

impl<T> File<T> {
    fn get_content_type(&self) -> &str {
        match &self.file_type {
            FileTypes::Microsoft(file_type) => match file_type {
                MSDocType::DOC => "application/msword",
                MSDocType::DOCX => {
                    "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                }
                MSDocType::XLSX => {
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                }
            },
            FileTypes::PDF => "application/pdf",
        }
    }
}

// Constrain methods to specific generic types
// By specifying the explicit type, we can make certain
// methods only available for those types!

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
    let files = vec![
        File {
            file_type: FileTypes::Microsoft(MSDocType::DOC),
            data: "",
            size: 200,
        },
        File {
            file_type: FileTypes::Microsoft(MSDocType::DOCX),
            data: "",
            size: 680,
        },
        File {
            file_type: FileTypes::PDF,
            data: "",
            size: 1024,
        },
    ];
    for file in &files {
        let content_type = file.get_content_type();
        println!(
            "The content type for {:?} is {content_type}",
            file.file_type
        );
    }
}
