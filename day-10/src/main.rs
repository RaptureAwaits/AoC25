use std::fmt;
use aoc_shared::{get_input_filepath, line_iterator};

struct Machine {
    buttons: Vec<Vec<bool>>,
    basis_buttons: Vec<Vec<bool>>,

    state: Vec<bool>,
    target: Vec<bool>,

    jolt_state: Vec<u16>,
    jolt_target: Vec<u16>,
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut write_parts: Vec<String> = vec![];

        write_parts.push(format!(
            "[{}] ", self.target.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(",")
        ));
        for button in &self.buttons {
            write_parts.push(format!(
                "({}) ", button.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(",")
            ));
        }
        write_parts.push(format!(
            "{{{}}}", self.jolt_target.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(",")
        ));
        write!(f, "{}", write_parts.join(""))
    }
}

impl Machine {
    fn new(row: String) -> Machine {
        let splits = row.split(" ").map(|s| s.to_string());

        let light_vec: Vec<bool> = vec![];
        let button_vec: Vec<Vec<bool>> = vec![];
        let jolts_vec: Vec<u16> = vec![];

        let mut new_machine = Machine {
            buttons: button_vec,
            basis_buttons: vec![],

            state: vec![],
            target: light_vec,

            jolt_state: vec![],
            jolt_target: jolts_vec,
        };

        for split in splits {
            match split.chars().collect::<Vec<char>>()[..] {
                ['[', ..] => {
                    new_machine.target = {
                        split[1..split.len() - 1]
                            .chars()
                            .filter(|c| *c == '#' || *c == '.')
                            .map(|c| if c == '#' { true } else { false })
                            .collect::<Vec<bool>>()
                    };
                    new_machine.state = vec![false; new_machine.target.len()];
                    new_machine.jolt_state = vec![0; new_machine.jolt_target.len()];
                }

                ['(', ..] => new_machine.add_button(
                    split[1..split.len() - 1]
                        .split(",")
                        .filter(|c| !c.is_empty())
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                ),

                ['{', ..] => new_machine.jolt_target = {
                    split[1..split.len() - 1]
                        .split(",")
                        .filter(|c| !c.is_empty())
                        .map(|s| s.parse::<u16>().unwrap())
                        .collect::<Vec<u16>>()
                },

                _ => ()
            }
        }
        new_machine
    }

    fn add_button(&mut self, affects: Vec<usize>) {
        let mut button_vec = vec![false; self.target.len()];

        for a in affects {
            button_vec[a] = true;
        }

        self.buttons.push(button_vec);
    }

    fn push_button(&mut self, button_index: usize) {
        for (i, b) in self.buttons[button_index].iter().enumerate() {
            if *b {
                self.state[i] = !self.state[i];
                self.jolt_state[i] += 1;
            }
        }
    }

    fn reset(&mut self) {
        self.state = vec![false; self.target.len()];
        self.jolt_state = vec![0; self.jolt_target.len()];
    }

    fn print_state(&self) {
        let current = self.state.iter().map(|b| if *b { '#' } else { '.' }).collect::<String>();
        println!("{}", current);

        let target = self.target.iter().map(|b| if *b { '#' } else { '.' }).collect::<String>();
        println!("{}", target);

        let jolt_state = self.jolt_state.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(",");
        println!("{}", jolt_state);

        let jolt_target = self.jolt_target.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(",");
        println!("{}", jolt_target);

        println!("\n");
    }
}

fn main() {
    let filepath = get_input_filepath();

    let mut machines: Vec<Machine> = vec![];
    if let Ok(rows) = line_iterator(&filepath) {
        for row in rows.map_while(Result::ok) {
            machines.push(Machine::new(row));
        }
    }

    let mut presses: usize = 0;
    let mut jolt_presses: usize = 0;

    // Part 1
    for mut machine in &mut machines {
        let c: u32 = { 2 as u32 }.pow(machine.buttons.len() as u32);
        let mut combinations = { 0..c }
            .map(|u| format!("{:0>pad$b}", u, pad = machine.buttons.len())
                .chars()
                .enumerate()
                .filter(|(_, b)| *b == '1')
                .map(|(i, _)| i)
                .collect::<Vec<usize>>()
            )
            .collect::<Vec<Vec<usize>>>();
        combinations.sort_by(|a, b| a.len().cmp(&b.len()));

        for combo_index in 0..combinations.len() {
            for button_index in &combinations[combo_index] {
                machine.push_button(*button_index);
            }

            if machine.state == machine.target {
                presses += combinations[combo_index].len();
                break;
            }

            machine.reset();
        }
    }
    println!("The fewest presses to activate every machine is {}", presses);

    // Part 2
    for mut machine in machines {
        machine.reset();

        // Combination logic for multiple presses
        // Calculate "basis buttons" to eliminate redundant buttons

        if machine.jolt_state == machine.jolt_target {
            break;
        }
    }
    println!("The fewest presses to configure every machine's joltage is {}", jolt_presses);
}