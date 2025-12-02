use std::env;
use std::path::Path;
use aoc_shared::read_delimited_file;

fn main() {
    let root_dir = env::current_dir().unwrap();
    let input_file = Path::new("inputs.txt");
    let filepath = root_dir.join(input_file);

    let mut invalid_id_sum: u64 = 0;

    for input in read_delimited_file(&filepath, ',') {
        let range = input.split('-')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let start = range[0];
        let end = range[1] + 1;

        for id in start..end {
            if is_id_invalid(id) {
                invalid_id_sum += id;
                // println!("Invalid ID: {}", id)
            }
        }
    }
    println!("The sum of invalid IDs is {}", invalid_id_sum);
}

fn is_id_invalid(id: u64) -> bool {
    let id_str = id.to_string();
    let l = id_str.chars().count();

    if l % 2 == 1 {
        return false;
    }

    let first = &id_str[0..l/2];
    let second = &id_str[l/2..];

    if first == second {
        return true;
    }

    false
}
