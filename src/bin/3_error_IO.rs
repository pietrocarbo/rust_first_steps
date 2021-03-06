use std::fs::File;
use std::io::{self, Read};
use std::num;
use std::path::Path;

#[derive(Debug)]
enum CustomError {
    Io(io::Error), // we could add more meaningful data
    Parse(num::ParseIntError),
}

fn file_parse_verbose<P: AsRef<Path>>(file_path: P) -> Result<i32, CustomError> {
    let mut file = File::open(file_path).map_err(CustomError::Io).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(CustomError::Io).unwrap();
    let n: i32 = contents.trim().parse().map_err(CustomError::Parse).unwrap();
    Ok(n)
}

// provide an implemention for converting a io::Error to a CustomError
impl From<io::Error> for CustomError {
    fn from(err: io::Error) -> CustomError {
        CustomError::Io(err)
    }
}

impl From<num::ParseIntError> for CustomError {
    fn from(err: num::ParseIntError) -> CustomError {
        CustomError::Parse(err)
    }
}

fn file_parse<P: AsRef<Path>>(file_path: P) -> Result<i32, CustomError> {
    let mut file = File::open(file_path)?;  // here can have a io::Error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;    // here can have a io::Error
    let n: i32 = contents.trim().parse()?;  // here can have a num::ParseIntError
    Ok(n)
}

fn main() {
    let n: i32 = file_parse("/home/pietro/Desktop/Emerging Programming Paradigms/rust_projects/presentation/resources/3_number.txt").unwrap_or(0);
    println!("parsed number is {}", n)
}