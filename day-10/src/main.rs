use std::fmt;
use aoc_shared::{get_input_filepath, line_iterator};

struct Machine {
    buttons: Vec<Vec<bool>>,
    joltages: Vec<u16>,

    state: Vec<bool>,
    target: Vec<bool>,
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
            "{{{}}}", self.joltages.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(",")
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
            target: light_vec,
            buttons: button_vec,
            joltages: jolts_vec,
            state: vec![],
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
                }

                ['(', ..] => new_machine.add_button(
                    split[1..split.len() - 1]
                        .split(",")
                        .filter(|c| !c.is_empty())
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                ),

                ['{', ..] => new_machine.joltages = {
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
        for i in 0..self.target.len() {
            self.state[i] = self.state[i] ^ self.buttons[button_index][i];
        }
    }

    fn reset(&mut self) {
        self.state = vec![false; self.target.len()];
    }

    fn print_state(&self) {
        let current = self.state.iter().map(|b| if *b { '#' } else { '.' }).collect::<String>();
        println!("{}", current);

        let target = self.target.iter().map(|b| if *b { '#' } else { '.' }).collect::<String>();
        println!("{}", target);

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
    for mut machine in machines {
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
}