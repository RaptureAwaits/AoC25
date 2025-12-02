use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

const START: i16 = 50;


fn main() {
    let root_dir = env::current_dir().unwrap();
    let input_file = Path::new("inputs.txt");
    let filepath = root_dir.join(input_file);

    let mut dial: i16 = START;
    let mut zeroes: i16 = 0;

    if let Ok(inputs) = line_iterator(&filepath) {
        for input in inputs.map_while(Result::ok) {
            let direct = input.chars().next().unwrap();
            let shift = input[1..].parse::<i16>().unwrap();

            let mut new_dial = dial;
            match direct {
                'L' => new_dial -= shift,
                'R' => new_dial += shift,
                _ => panic!("Invalid input")
            };
            // println!("{} -> {} -> {} ({})", dial, new_dial, new_dial.rem_euclid(100), input);

            dial = new_dial.rem_euclid(100);
            if dial == 0 {
                zeroes += 1
            }
        }
    }
    print!("Zeroes encountered in sequence: {}", zeroes)
}

fn line_iterator(filename: &Path) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
