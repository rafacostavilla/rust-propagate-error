use std::fs::File;
use std::io::{self, Read};

fn main() {
    let result = read_username_from_file();
    match result {
        Ok(username) => println!("{username}"),
        Err(error) => panic!("Error is: {error:?}"),
    }
}


fn read_username_from_file() -> Result<String, io::Error>{
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}