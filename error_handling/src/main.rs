use std::fs::File;
use std::error::Error as StdError;
use std::io::{Error, Read, ErrorKind};

fn main() -> Result<(), Box<dyn StdError>> {
    let f1 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    let f2 = File::open("hello.txt").unwrap(); // unwrap will return value if no error occurred or use panic! macro if error occurred
    let f3 = File::open("hello.txt").expect("Failed to open hello.txt"); // will test is error massage matches pattern provided in expect body
    let file_or_error = read_username_from_file();
    return Ok(());
}

fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_v2() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?; // ? in the end of the line means if error occurred than stop method and return occurred error
    Ok(s)
}

// ? operator can be used only objects which implements std::ops::Try interface, for example Result, Option
fn optional_chaining()-> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?; // if File::open will fail Error will be returned. If FIle open was successful but read_to_string occurred and error than method will return error
    return Ok(s);
}
