use std::fs::File;
use std::io::{Read, Error};

pub fn open_file(filename: &str) -> Result<String, Error> {
    let mut f = File::open(filename)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}
