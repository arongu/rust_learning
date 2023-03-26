use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::{fs, io};

const HELLO_TEXT: &str = "/home/aron/hello.txt";

pub fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open(HELLO_TEXT);

    let mut f: File = match f {
        Ok(file) => file,
        Err(err) => Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut f = File::open(HELLO_TEXT)?; // if ok returns file, if not returns the error

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(HELLO_TEXT)?.read_to_string(&mut s)?; // chaining methods
    Ok(s)
}

pub fn read_username_from_file3() -> String {
   fs::read_to_string(HELLO_TEXT)?
}