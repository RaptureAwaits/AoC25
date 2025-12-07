use std::collections::HashSet;
use aoc_shared::{get_input_filepath, line_iterator};

fn main() {
    let filepath = get_input_filepath();

    let mut beams: HashSet<usize> = HashSet::new();
    let mut splits: u16 = 0;
    if let Ok(rows) = line_iterator(&filepath) {
        let mut width: usize = 0;

        for row in rows.map_while(Result::ok) {
            if beams.is_empty() && let Some(start_index) = row.find('S') {
                beams.insert(start_index);
                width = row.len();
                continue;
            }

            let mut search_index: usize = 0;
            let mut new_beams: HashSet<usize> = beams.clone();
            while let Some(splitter_index) = row[search_index..].find('^') {
                let splitter_pos = search_index + splitter_index;

                if beams.contains(&splitter_pos) {
                    splits += 1;
                    new_beams.remove(&splitter_pos);
                    if splitter_pos > 0 {
                        new_beams.insert(splitter_pos - 1);
                    }
                    if splitter_pos < width - 1 {
                        new_beams.insert(splitter_pos + 1);
                    }
                }
                search_index = splitter_pos + 1
            }
            beams = new_beams;


        }
    }
    println!("The beam has been split {} times", splits);
}