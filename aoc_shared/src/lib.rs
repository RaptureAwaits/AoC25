use std::fs::{File, read_to_string};
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub fn read_delimited_file(filename: &Path, delimiter: char) -> Vec<String>{
    let list_string = read_to_string(filename).unwrap();
    let list = list_string.split(delimiter).map(String::from).collect::<Vec<String>>();
    list
}

pub fn line_iterator(filename: &Path) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
