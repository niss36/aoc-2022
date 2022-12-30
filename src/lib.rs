use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    return reader.lines().collect();
}

pub fn to_lines(data: &str) -> Vec<String> {
    data.lines().map(|s| s.to_owned()).collect()
}
