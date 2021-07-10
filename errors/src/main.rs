use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    //cause_panic();

    use_result();
}

fn cause_panic() {
    let v = vec![1, 2, 3];
    v[99];
}

fn use_result() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opeing the file: {:?}", other_error)
            }
        },
    };

    let f2 = File::open("hello.txt").unwrap();
    let f3 = File::open("hello.txt").expect("Some error message!");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}