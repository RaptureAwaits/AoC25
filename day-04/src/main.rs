use aoc_shared::{get_input_filepath, line_iterator};

fn main() {
    let filepath = get_input_filepath();

    let mut row_window: Vec<Vec<char>> = vec![vec![]];
    let mut row_len: usize = 0;


    let mut p2_tiles: Vec<Vec<char>> = vec![vec![]];

    // This block completes part 1 whilst completing one iteration for part 2
    let mut removed_papers: u32 = 0;
    if let Ok(rows) = line_iterator(&filepath) {
        for row in rows.map_while(Result::ok) {

            let mut tiles: Vec<char> = row.chars().collect::<Vec<char>>();
            row_len = tiles.len();
            tiles.insert(0, '|');
            tiles.push('|');

            if row_window.len() == 1 {
                row_window.push(vec!['-'; row_len + 2]);
                p2_tiles.push(vec!['-'; row_len + 2]);
                row_window.push(tiles);
                continue;
            }

            removed_papers += process_row_window(&mut row_window, tiles, &mut p2_tiles);
        }

        // Push one more "empty" row to count the last line of input
        removed_papers += process_row_window(&mut row_window, vec!['-'; row_len + 2], &mut p2_tiles);
        p2_tiles.push(vec!['-'; row_len + 2]);
    }
    println!("The total number of accessible papers before removal is {}", removed_papers);

    let mut removed_papers_this_iter: u32 = 1;
    while removed_papers_this_iter > 0 {
        let mut row_window: Vec<Vec<char>> = p2_tiles[0..3].to_vec();
        let mut new_tiles: Vec<Vec<char>> = vec![vec![], vec!['-'; row_len + 2]];

        removed_papers_this_iter = 0;
        for row_index in 1..p2_tiles.len() - 2 {
            removed_papers_this_iter += process_row_window(&mut row_window, p2_tiles[row_index + 2].to_vec(), &mut new_tiles)
        }
        new_tiles.push(vec!['-'; row_len + 2]);
        p2_tiles = new_tiles;
        removed_papers += removed_papers_this_iter;
        // _display_tile_grid(&p2_tiles);
        // _ = stdin().read_line(&mut String::new()).unwrap();
    }
    println!("The total number of removable papers is {}", removed_papers);
}

fn process_row_window(row_window: &mut Vec<Vec<char>>, new_row: Vec<char>, new_tile_grid: &mut Vec<Vec<char>>) -> u32 {
    row_window.remove(0);
    row_window.push(new_row);

    let (removed_papers, new_tiles) = count_middle_row(&row_window);
    new_tile_grid.push(new_tiles);

    removed_papers
}

fn count_middle_row(row_window: &Vec<Vec<char>>) -> (u32, Vec<char>) {
    let mut accessible_papers: u32 = 0;

    let mut new_tiles: Vec<char> = row_window[1].clone();
    for tile_index in 1..row_window[0].len() - 1 {
        // We don't care if empty tiles are accessible (it took me way too long to realize this...)
        let tile = row_window[1][tile_index];
        if tile != '@' {
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
            new_tiles[tile_index] = '*';
        }
    }
    (accessible_papers, new_tiles)
}

fn _display_tile_grid(tile_grid: &Vec<Vec<char>>) {
    for row in tile_grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}
