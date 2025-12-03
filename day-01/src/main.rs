use aoc_shared::{get_input_filepath, line_iterator};

const START: i16 = 50;


fn main() {
    let filepath = get_input_filepath();

    let mut dial: i16 = START;
    let mut zeroes_p1: i16 = 0;
    let mut zeroes_p2: i16 = 0;

    if let Ok(inputs) = line_iterator(&filepath) {
        for input in inputs.map_while(Result::ok) {
            let direct = input.chars().next().unwrap();
            let mag = input[1..].parse::<i16>().unwrap();

            let turn: i16;
            match direct {
                'L' => turn = -mag,
                'R' => turn = mag,
                _ => panic!("Invalid input")
            };

            let new_dial = dial + turn;
            let new_dial_actual = new_dial.rem_euclid(100);

            zeroes_p1 += process_input_p1(new_dial_actual);
            zeroes_p2 += process_input_p2(dial, turn, new_dial, new_dial_actual);

            dial = new_dial_actual;

        }
    }
    println!("(Part 1) Zeroes encountered in sequence: {}", zeroes_p1);
    println!("(Part 2) Zeroes encountered in sequence: {}", zeroes_p2);
}

fn process_input_p1(new_dial_actual: i16) -> i16 {
    if new_dial_actual == 0 {
        1
    } else {
        0
    }
}

fn process_input_p2(current_dial: i16, turn: i16, new_dial: i16, new_dial_actual: i16) -> i16 {
    let full_turns = {turn / 100}.abs();
    let rem: i16 = turn % 100;

    let mut zeroes = full_turns;
    if current_dial != 0 && {current_dial + rem != new_dial_actual || new_dial_actual == 0} {
        zeroes += 1
    }

    println!("{} -> {} -> {} => {} zeroes", current_dial, turn, new_dial_actual, zeroes);

    zeroes
}
