use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::ParseIntError,
    path::Path,
};

#[derive(Debug)]
pub enum AocError {
    IoError(io::Error),
    ParseIntError(ParseIntError),
}

impl From<io::Error> for AocError {
    fn from(e: io::Error) -> Self {
        AocError::IoError(e)
    }
}

impl From<ParseIntError> for AocError {
    fn from(e: ParseIntError) -> Self {
        AocError::ParseIntError(e)
    }
}

pub fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    return reader.lines().collect();
}
