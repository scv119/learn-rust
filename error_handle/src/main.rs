use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::error::Error;

fn panic_vector() {
    let v = vec![1, 2, 3];
    v[99];
}

fn read_file() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            },
        },
    };
}

fn read_file1() {
    let file = "hello2.txt".to_string();
    let greeting_file_result = File::open(&file).expect("file should exist");
}

fn read_file2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("wat1")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() -> Result<(), Box<dyn Error>> {
    let result = read_file2()?;
    Ok(())
}

