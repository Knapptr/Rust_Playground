#![allow(unused)]

use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let int = return_err().unwrap();
}
fn return_err() -> Result<i32, String> {
    Err("Error".to_string())
}
//
// Using ? with options
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
// Propagating errors with ? operator declaratively (uses chains for brevity)
fn read_username_from_file_op_chain() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
// Propagating errors with ? operator declaratively (the same as above but more verbose)
fn read_username_from_file_op() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

//Returning Results to function caller - imperitavely
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
// Result methods
// use std::fs::File;
// fn main() {
// explicitly state panic! message
// let file_handle = File::open("a_non_existant_file.txt")
//     .expect("Problem opening the file");
// just call a panic message with no explicit message
// let file_handle = File::open("a_non_existant_file.txt").unwrap();
// }
// Recoverable errors
// use std::{fs::File, io::ErrorKind};
// fn main() {
//     let file_open_result = File::open("hello.txt");

//     let file = match file_open_result {
//         Ok(f) => f,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(f) => f,
//                 Err(e) => panic!("Error creating file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error)
//             }
//         },
//     };
// }
// Trace test
// call this with RUST_BACKTRACE=1 cargo run
//
// fn main() {
//     let v = vec![1, 2];
//     v[99];
// }
//
// Panic
// fn main() {
//     panic!("-Crash!-");
// }
