use std::env;
use std::path::Path;
use aoc_shared::read_delimited_file;

fn main() {
    let root_dir = env::current_dir().unwrap();
    let input_file = Path::new("inputs.txt");
    let filepath = root_dir.join(input_file);

    let mut invalid_id_sum_p1: u64 = 0;
    let mut invalid_id_sum_p2: u64 = 0;

    for input in read_delimited_file(&filepath, ',') {
        let range = input.split('-')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let start = range[0];
        let end = range[1] + 1;

        for id in start..end {
            if is_id_invalid_p1(id) {
                invalid_id_sum_p1 += id;
                // println!("Invalid ID: {}", id)
            }

            if is_id_invalid_p2(id) {
                invalid_id_sum_p2 += id;
                // println!("Invalid ID: {}", id)
            }
        }
    }
    println!("(Part 1) The sum of invalid IDs is {}", invalid_id_sum_p1);
    println!("(Part 2) The sum of invalid IDs is {}", invalid_id_sum_p2);
}

fn is_id_invalid_p1(id: u64) -> bool {
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

fn is_id_invalid_p2(id: u64) -> bool {
    let id_str = id.to_string();
    let l = id_str.chars().count() as u8;

    let l_max = {l as f32 / 2.0}.floor() as u8 + 1;

    for substr_len in 1..l_max {
        if l % substr_len != 0 {
            continue;
        }

        let substr = &id_str[0..substr_len as usize];
        let reps = l / substr_len;
        if reps > 1 && substr.repeat(reps as usize) == id_str {
            return true;
        }
    }

    false
}
