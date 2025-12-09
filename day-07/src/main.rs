use std::collections::HashSet;
use aoc_shared::{get_input_filepath, line_iterator};

struct BeamTree {
    beams: Vec<Beam>,
}

impl BeamTree {
    fn new(start: usize) -> BeamTree {
        BeamTree { beams: vec![Beam::new(0, start)] }
    }

    fn new_node(&mut self, x: usize) -> usize {
        let new_beam_index = self.beams.len();
        self.beams.push(Beam::new(new_beam_index, x));
        new_beam_index
    }

    fn get_node(&mut self, index: usize) -> &mut Beam {
        &mut self.beams[index]
    }

    // fn display_node(&self, index: usize, path: String) {
    //     if self.beams[index].left.is_some() {
    //         print!("{}", self.beams[index].left.unwrap());
    //     }
    //     print!(" <- {} ({}) -> ", index, path);
    //     if self.beams[index].right.is_some() {
    //         print!("{}", self.beams[index].right.unwrap());
    //     }
    //     println!("\n");
    // }

    fn count_branches(&mut self, beam_index: usize, path: String) -> u64 {
        if self.beams[beam_index].branches > 0 {
            return self.beams[beam_index].branches;
        } else {
            // self.display_node(beam_index, path.clone());
        }

        if self.beams[beam_index].left.is_none() && self.beams[beam_index].right.is_none() {
            self.beams[beam_index].branches = 1;
            return 1;
        }

        let mut branches = 0;
        if self.beams[beam_index].left.is_some() {
            branches += self.count_branches(self.beams[beam_index].left.unwrap(), path.clone() + "L");
        }

        if self.beams[beam_index].right.is_some() {
            branches += self.count_branches(self.beams[beam_index].right.unwrap(), path.clone() + "R");
        }

        self.beams[beam_index].branches = branches;
        branches
    }
}


struct Beam {
    index: usize,
    branches: u64,

    x: usize,
    active: bool,

    left: Option<usize>,
    right: Option<usize>,
}

impl Beam {
    fn new(index: usize, x: usize) -> Beam {
        Beam {index, branches: 0, x, active: true, left: None, right: None }
    }
}


fn main() {
    let filepath = get_input_filepath();
    if let Ok(mut rows) = line_iterator(&filepath) {
        let mut width: usize = 0;
        let mut start_index: usize = 0;

        // Part 1
        let mut splits: usize = 0;

        // Initialize grid property variables, beam vector for part 1, and splitter BST for part 2
        if let Ok(first_row) = rows.next().unwrap() {
            if let Some(s) = first_row.find('S') {
                start_index = s;
                width = first_row.len();
            }
        }

        // Create root beam
        let mut beam_tree = BeamTree::new(start_index);

        for row in rows.map_while(Result::ok) {
            // Find positions of all splitters in row
            let mut search_index: usize = 0;
            let mut splitter_positions: HashSet<usize> = HashSet::new();
            while let Some(splitter_pos) = row[search_index..].find('^') {
                splitter_positions.insert(search_index + splitter_pos);
                search_index += splitter_pos + 1;
            }

            // Check if any beams hit splitters
            let active_beams = beam_tree.beams
                .iter()
                .filter(|b| b.active)
                .map(|b| b.index)
                .collect::<Vec<usize>>()
                .clone();

            let mut beam_positions = beam_tree.beams
                .iter()
                .filter(|b| b.active)
                .map(|b| b.x)
                .collect::<Vec<usize>>()
                .clone();
            beam_positions.sort();

            draw_row(row, beam_positions.clone());
            // println!("Active beams: {} => {}\n", active_beams.len(), beam_positions.clone().iter().map(|u| u.to_string() + ", ").collect::<String>());
            // println!("{}", splits);

            // let s = &mut String::new();
            // let _ = stdin().read_line(s);

            if splitter_positions.is_empty() {
                continue;
            }

            let unique_beam_positions: HashSet<usize> = HashSet::from_iter(beam_positions.clone().iter().map(|u| *u));
            // println!("Unique beam positions: {}", unique_beam_positions.iter().map(|u| u.to_string() + ", ").collect::<String>());
            splits += splitter_positions.intersection(&unique_beam_positions).count();

            let mut pos_to_index_map: Vec<Option<usize>> = vec![None; width];
            for incident_beam_index in active_beams {
                let inc_x: usize;
                {
                    let beam = beam_tree.get_node(incident_beam_index);
                    inc_x = beam.x;
                }

                if splitter_positions.contains(&inc_x) {
                    {
                        let beam = beam_tree.get_node(incident_beam_index);
                        beam.active = false;
                    }

                    if inc_x > 0 {
                        let new_beam_pos = inc_x - 1;
                        let left_beam_index = pos_to_index_map[new_beam_pos];
                        match left_beam_index {
                            Some(_) => {
                                beam_tree.get_node(incident_beam_index).left = left_beam_index;
                            },
                            None => {
                                let new_beam_index = beam_tree.new_node(new_beam_pos);
                                beam_tree.get_node(incident_beam_index).left = Some(new_beam_index);
                                pos_to_index_map[new_beam_pos] = Some(new_beam_index);
                            },
                        }
                    }

                    if inc_x < width - 1 {
                        let new_beam_pos = inc_x + 1;
                        let right_beam_index = pos_to_index_map[new_beam_pos];
                        match right_beam_index {
                            Some(_) => {
                                beam_tree.get_node(incident_beam_index).right = right_beam_index;
                            },
                            None => {
                                let new_beam_index = beam_tree.new_node(new_beam_pos);
                                beam_tree.get_node(incident_beam_index).right = Some(new_beam_index);
                                pos_to_index_map[new_beam_pos] = Some(new_beam_index);
                            },
                        }
                    }
                }
            }

            // println!("{}", pos_to_index_map
            //     .iter()
            //     .map(|u| { if u.is_some() { u.unwrap().to_string() } else { "X".to_string() } })
            //     .collect::<String>());
        }

        // Part 1
        println!("The beam has been split {} times", splits);

        // Part 2
        println!("There are {} possible timelines for the beam", beam_tree.count_branches(0, String::new()))
    }
}

fn draw_row(row: String, beam_positions: Vec<usize>) {
    let mut drawn_row = row.clone();
    for pos in beam_positions {
        if drawn_row.chars().nth(pos).unwrap() != '^' {
            drawn_row.replace_range(pos..pos + 1, "|");
        }
    }
    println!("{}", drawn_row);
}