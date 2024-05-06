#![allow(unused)]
use std::error::Error;
use std::fs::File;

fn main() {
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

fn _old_main1() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

fn _old_main() {
    // let s = last_char_of_first_line("");
    // println!("{:?}", s);
    let s = last_char_of_first_line("hello world\n world hello");
    println!("{:?}", s);

    //  // gives an error
    // let greeting_file = File::open("hello.txt")?;
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// A Shortcut for Propagating Errors: the ? Operator
fn _question_operator() {
    use std::fs;
    use std::io::{self, Read};

    fn read_username_from_file_shorter() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    fn read_username_from_file_chain() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
}

// Propagating Errors
fn propagating_errors() {
    use std::io::{self, Read};

    fn _read_username_from_file() -> Result<String, io::Error> {
        let user_file_result = File::open("hello.txt");

        let mut username_file = match user_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
}

fn _expect_fn() {
    let _greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn _unwarp_fn() {
    let _greeting_file = File::open("hello.txt").unwrap();
}

use std::io::ErrorKind;

fn _old2() {
    let _greeting_file = File::open("hello,txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the firl: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn _old1() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
