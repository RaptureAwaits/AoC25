use transpose;
use aoc_shared::{get_input_filepath, line_iterator};

fn main() {
    let filepath = get_input_filepath();

    let mut operands: Vec<Vec<u64>> = vec![];
    if let Ok(rows) = line_iterator(&filepath) {

        let mut row_len: usize = 0;

        let mut operations: Vec<char> = vec![];

        let mut text_array: Vec<char> = vec![];
        let mut text_cols: usize = 0;
        let mut text_rows: usize = 0;

        for row in rows.map_while(Result::ok) {
            if row.contains('+') || row.contains('*') {
                operations = row.split_whitespace()
                    .into_iter()
                    .map(|s| s.to_string().chars().nth(0).unwrap())
                    .collect::<Vec<char>>();
                break;
            }

            text_rows += 1;
            text_array.extend(row.chars());

            let row_nums: Vec<u64> = row.split_whitespace()
                .into_iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();


            if operands.is_empty() {
                text_cols = row.len();
                row_len = row_nums.len();
                operands = vec![vec![]; row_len];
            }

            for index in 0..row_len {
                operands[index].push(row_nums[index]);
            }
        }

        // Part 1
        let result_sum = calculate(operands, &operations);
        println!("The sum of all problem results is {}", result_sum);

        // Part 2
        let mut transpose_array: Vec<char> = vec![' '; text_array.len()];
        transpose::transpose(&text_array, &mut transpose_array, text_cols, text_rows);

        let mut transpose_operands: Vec<Vec<u64>> = vec![];

        let mut op_block: Vec<u64> = vec![];
        let transpose_row_chunks = transpose_array.chunks(text_rows);
        let chunk_strings = transpose_row_chunks
            .map(|c| c.to_vec()
                .iter()
                .collect::<String>()
            ).collect::<Vec<String>>();

        for chunk_text in chunk_strings {
            let trim_text = chunk_text.trim();
            if trim_text.is_empty() {
                transpose_operands.push(op_block);
                op_block = vec![];
            } else {
                op_block.push(trim_text.parse::<u64>().unwrap())
            }
        }

        if !op_block.is_empty() {
            transpose_operands.push(op_block);
        }

        let result_sum_p2 = calculate(transpose_operands, &operations);
        println!("The sum of all problem results in cephalopod math is {}", result_sum_p2);
    }
}

fn calculate(operands: Vec<Vec<u64>>, operations: &Vec<char>) -> u64 {
    let mut result_sum: u64 = 0;
    for i in 0..operations.len() {
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
        // println!("({}) {} = {}", operations[i], operands[i].iter().map(|u| u.to_string() + " ").collect::<String>(), result);
        result_sum += result;
    }

    result_sum
}