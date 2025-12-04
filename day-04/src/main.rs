use aoc_shared::{get_input_filepath, line_iterator};

fn main() {
    let filepath = get_input_filepath();

    let mut row_window: Vec<Vec<char>> = vec![vec![]];
    let mut row_len: usize = 0;

    let mut accessible_papers: u32 = 0;
    if let Ok(rows) = line_iterator(&filepath) {


        for row in rows.map_while(Result::ok) {

            let mut tiles: Vec<char> = row.chars().collect::<Vec<char>>();
            row_len = tiles.len();
            tiles.insert(0, '|');
            tiles.push('|');

            if row_window.len() == 1 {
                row_window.push(vec!['-'; row_len + 2]);
                row_window.push(tiles);
                continue;
            }

            // Shift the window down a row
            row_window.remove(0);
            row_window.push(tiles);
            accessible_papers += count_middle_row(&row_window);
        }

        // Push one more "empty" row to count the last line of input
        row_window.remove(0);
        row_window.push(vec!['-'; row_len + 2]);
        accessible_papers += count_middle_row(&row_window);
    }
    println!("The total number of accessible papers is {}", accessible_papers);
}

fn count_middle_row(row_window: &Vec<Vec<char>>) -> u32 {
    let mut accessible_papers: u32 = 0;


    for tile_index in 1..row_window[0].len() - 1 {
        // We don't care if empty tiles are accessible (it took me way too long to realize this...)
        let tile = row_window[1][tile_index];
        if tile == '.' {
            continue;
        }

        // Gather values for surrounding tiles into one vector
        let window_top = &row_window[0][tile_index - 1..tile_index + 2].to_vec();
        let window_mid = vec![row_window[1][tile_index - 1], row_window[1][tile_index + 1]];
        let window_bottom = &row_window[2][tile_index - 1..tile_index + 2].to_vec();
        // print!(
        //     "\n\n{}\n{}X{}\n{}",
        //     window_top.iter().collect::<String>(),
        //     window_mid[0],
        //     window_mid[1],
        //     window_bottom.iter().collect::<String>()
        // );

        let mut window_contents: Vec<char> = vec![];
        window_contents.extend(window_top);
        window_contents.extend(window_mid);
        window_contents.extend(window_bottom);

        // Count paper tiles in window vector
        if window_contents.into_iter().filter(|&c| c == '@').count() < 4 {
            accessible_papers += 1;
            // print!(" => accessible\n")
        }

    }
    accessible_papers
}
