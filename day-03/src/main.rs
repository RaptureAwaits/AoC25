use aoc_shared::{get_input_filepath, line_iterator};

fn main() {
    let filepath = get_input_filepath();

    let mut max_joltage_sum_p1: u64 = 0;
    let mut max_joltage_sum_p2: u64 = 0;
    if let Ok(inputs) = line_iterator(&filepath) {
        for bank in inputs.map_while(Result::ok) {
            let jolts = bank.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>();

            let max_joltage_p1 = get_max_joltage(&jolts, 2);
            // println!("{} => {}", bank, max_joltage_p1);
            max_joltage_sum_p1 += max_joltage_p1;

            let max_joltage_p2 = get_max_joltage(&jolts, 12);
            // println!("{} => {}", bank, max_joltage_p2);
            max_joltage_sum_p2 += max_joltage_p2;
        }

        println!("The maximum safe joltage across all banks is {}", max_joltage_sum_p1);
        println!("The maximum unsafe joltage across all banks is {}", max_joltage_sum_p2);
    }
}

fn get_max_joltage(jolts: &Vec<u8>, n_digits: usize) -> u64 {
    let bank_length = jolts.len();

    let mut digits_by_jolt_index = {0..n_digits}.collect::<Vec<usize>>();

    let mut jolt_index: usize = 0;
    while jolt_index < bank_length {
        let joltage = jolts[jolt_index];
        let batteries_to_right = jolts.len() - 1 - jolt_index;

        let mut digit_index: usize = 0;
        while digit_index < n_digits {
            let dji = digits_by_jolt_index[digit_index];
            if dji >= jolt_index {
                break;
            }

            let digit = jolts[dji];
            let digits_to_right = n_digits - 1 - digit_index;

            if joltage > digit && batteries_to_right >= digits_to_right {
                let mut new_digit_indices = digits_by_jolt_index[0..digit_index].to_vec();
                new_digit_indices.extend({jolt_index..jolt_index + 1 + digits_to_right}.collect::<Vec<usize>>());
                digits_by_jolt_index = new_digit_indices;
                break;
            }

            digit_index += 1;
        }
        jolt_index += 1;
    }

    let max_joltage = digits_by_jolt_index
        .iter()
        .map(|u| jolts[*u].to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    max_joltage
}
