use std::fs::File;
use std::io::ErrorKind;

// enum Result <T,E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    
    // we don't necessarily need to panic, in every case, sometimes we can handle it more gracefully
    let open_file = File::open("hello.txt");

    let open = match open_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create the file")
            },
            _ => {
                panic!("Unknown error type")
            }
        }
    };


    // alternate method:
    let greeting_file = File::open("bye.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // unwrap and expect

    let greeting_file_2 = File::open("hello.txt").unwrap(); // crashes if hello.txt doesn't exist
    let greeting_file_3 = File::open("hello.txt").expect("hello.txt should be included in this project"); // returns the printed statement if the file doesn't exist

    // the ? operator : if the returning value is Ok(T), then it returns it properly, else if it's Err(E), it straigtaway returns the error to the function, instead of panicing
         // this operator can only be used on functions which are returning a Result type
   
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    /*
    Rust’s design encourages using Result for recoverable errors and panic! for unrecoverable or bug-related scenarios. You default to Result, but you can make targeted use of panic! for testing, prototyping, logically certain conditions, or enforcing critical invariants. Leveraging Rust’s type system to enforce correctness (e.g., via custom types) brings safety, clarity, and reliability to your code.

     */
}
