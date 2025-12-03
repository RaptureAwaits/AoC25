use std::fs::{File, read_to_string};
use std::{env, io};
use std::io::{BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

pub fn get_input_filepath() -> PathBuf {
    let root_dir = env::current_dir().unwrap();
    let input_file = Path::new("inputs.txt");
    let filepath = root_dir.join(input_file);
    filepath
}

pub fn read_delimited_file(filename: &Path, delimiter: char) -> Vec<String>{
    let list_string = read_to_string(filename).unwrap();
    let list = list_string.split(delimiter).map(String::from).collect::<Vec<String>>();
    list
}

pub fn line_iterator(filename: &Path) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
