use std::env;
use std::path::Path;
use aoc_shared::line_iterator;

const START: i16 = 50;


fn main() {
    let root_dir = env::current_dir().unwrap();
    let input_file = Path::new("inputs.txt");
    let filepath = root_dir.join(input_file);

    let mut dial: i16 = START;
    let mut clicks: i16 = 0;

    if let Ok(inputs) = line_iterator(&filepath) {
        for input in inputs.map_while(Result::ok) {
            let direct = input.chars().next().unwrap();

            let turn = input[1..].parse::<i16>().unwrap();
            let full_turns = turn / 100;
            let shift = turn % 100;

            let mut offset = dial;
            match direct {
                'L' => offset -= shift,
                'R' => offset += shift,
                _ => panic!("Invalid input")
            };

            let new_pos = offset.rem_euclid(100);

            let mut new_clicks = full_turns;
            if dial != 0 && { offset != new_pos || new_pos == 0} {
                new_clicks += 1
            }
            // println!("{} -> {} -> {} ({}) => {} clicks", dial, offset, new_pos, input, new_clicks);
            dial = new_pos;
            clicks += new_clicks;
        }
    }
    print!("Clicks through 0 encountered in sequence: {}", clicks)
}
