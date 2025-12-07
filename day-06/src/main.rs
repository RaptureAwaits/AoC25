use aoc_shared::{get_input_filepath, line_iterator};

fn main() {
    let filepath = get_input_filepath();

    let mut operands: Vec<Vec<u64>> = vec![];
    if let Ok(rows) = line_iterator(&filepath) {

        let mut row_len: usize = 0;
        let mut result_sum: u64 = 0;
        let mut operations: Vec<char> = vec![];
        for row in rows.map_while(Result::ok) {
            if row.contains('+') || row.contains('*') {
                operations = row.split_whitespace()
                    .into_iter()
                    .map(|s| s.to_string().chars().nth(0).unwrap())
                    .collect::<Vec<char>>();
                break;


            }

            let row_nums: Vec<u64> = row.split_whitespace()
                .into_iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            if operands.is_empty() {
                row_len = row_nums.len();
                operands = vec![vec![]; row_len];
            }

            for index in 0..row_len {
                operands[index].push(row_nums[index]);
            }
        }

        for i in 0..row_len {
            let result: u64;

            match operations[i] {
                '+' => {
                    result = operands[i].iter().sum();
                },
                '*' => {
                    result = operands[i].iter().fold(1, |acc, x| acc * x);
                },
                _ => panic!("Invalid operation")
            }
            // println!("{} {} = {}", operands[i].iter().map(|u| u.to_string() + " ").collect::<String>(), operations[i], result);
            result_sum += result;
        }
        println!("The sum of all problem results is {}", result_sum);
    }
}