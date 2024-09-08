use std::fs;
use std::io;

fn main() {
    let result = read_username_from_file();
    match result {
        Ok(username) => println!("{username}"),
        Err(error) => panic!("Error is: {error:?}"),
    }
}


fn read_username_from_file() -> Result<String, io::Error>{
    //Short form of doing the same as below using '?' mark

    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(error) => Err(error),
    // }

    // Shorter vesion of this code:
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // The shortest way to do this:
    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    fs::read_to_string("hello.txt")

}