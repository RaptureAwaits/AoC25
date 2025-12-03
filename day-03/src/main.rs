use aoc_shared::{get_input_filepath, line_iterator};

fn main() {
    let filepath = get_input_filepath();

    let mut max_joltage_sum: u32 = 0;
    if let Ok(inputs) = line_iterator(&filepath) {
        for bank in inputs.map_while(Result::ok) {
            let mut d1: u8 = 0;
            let mut d2: u8 = 0;
            let mut d1_pend: Option<u8> = None;

            let jolts = bank.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>();

            for joltage in jolts {
                if d1_pend.is_some() {
                    d1 = d1_pend.unwrap();
                    d2 = joltage;
                    d1_pend = None;
                } else if joltage > d2 {
                    d2 = joltage;
                }

                if joltage > d1 {
                    d1_pend = Some(joltage);
                }
            }

            let max_joltage = 10 * d1 + d2;
            // println!("{} => {}", bank, max_joltage);
            max_joltage_sum += max_joltage as u32;
        }

        println!("The maximum achievable joltage across all banks is {}", max_joltage_sum);
    }
}
