use aoc_shared::{get_input_filepath, line_iterator};

fn main() {
    let filepath = get_input_filepath();
    let mut fresh_count: u16 = 0;

    if let Ok(rows) = line_iterator(&filepath) {
        let mut read_ranges = true;

        let mut ranges: Vec<(u64, u64)> = vec![];
        for row in rows.map_while(Result::ok) {
            if row.is_empty() {
                // Blank line separating ranges and IDs
                read_ranges = false;
                println!("Collated {} distinct ranges from input", ranges.len());
                condense_ranges(&mut ranges);
                continue;
            }

            // Get all ranges into sorted vector via insertion sort
            if read_ranges {
                insert_new_range(row, &mut ranges)
            } else {
                if is_fresh(row, &ranges) {
                    fresh_count += 1;
                }
            }
        }
        println!("There are {} fresh ingredients in stock", fresh_count);
    }
}

fn insert_new_range(range_str: String, ranges: &mut Vec<(u64, u64)>) {
    let mid_index = range_str.find('-').unwrap();
    let start = range_str[..mid_index].parse::<u64>().unwrap();
    let end = range_str[mid_index + 1..].parse::<u64>().unwrap();

    // Init range vector
    if ranges.len() == 0 {
        ranges.push((start, end));
        return;
    }

    for range_index in 0..ranges.len() {
        let (rs, re) = ranges[range_index];

        if start > re {
            continue;
        }

        if start <= rs {
            if end > re {
                ranges[range_index] = (start, end);
            } else {
                ranges.insert(range_index, (start, end));
            }
            break;
        }
    }

}

fn condense_ranges(ranges: &mut Vec<(u64, u64)>) {
    // If adjacent ranges have no space between them, combine them into one range to optimize future iteration
    let mut range_index: usize = 0;
    let mut length: usize = ranges.len();
    while range_index < length - 1 {
        let (s, e) = ranges[range_index];
        let (ns, ne) = ranges[range_index + 1];

        if ns - 1 <= e && e <= ne {
            ranges[range_index] = (s, ne);
            ranges.remove(range_index + 1);
        } else {
            range_index += 1;
        }

        length = ranges.len();
    }
    println!("Condensed to {} non-adjacent ranges", length);
}

fn is_fresh(id_string: String, ranges: &Vec<(u64, u64)>) -> bool {
    let id: u64 = id_string.parse::<u64>().unwrap();

    for (start, end) in ranges {
        if id < *start {
            return false;
        } else if *start <= id && id <= *end {
            return true;
        }
    }
    false
}