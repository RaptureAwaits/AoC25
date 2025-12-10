use aoc_shared::{get_input_filepath, line_iterator};

type TileIndex = usize;

struct Rect {
    corner1: TileIndex,
    corner2: TileIndex,
    area: i64
}

impl Rect {
    fn new(corner1: &RedTile, corner2: &RedTile) -> Rect {
        Rect {
            corner1: corner1.index,
            corner2: corner2.index,
            area: { { corner1.x - corner2.x }.abs() + 1 } * { { corner1.y - corner2.y }.abs() + 1 }
        }
    }
}


struct RedTile {
    index: TileIndex,
    x: i64,
    y: i64,
}


fn main() {
    let filepath = get_input_filepath();

    if let Ok(rows) = line_iterator(&filepath) {

        let mut tiles: Vec<RedTile> = vec![];
        let mut rects: Vec<Rect> = vec![];

        for row in rows.map_while(Result::ok) {
            let coords = row.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            let (x, y) = match &coords[..] {
                &[x, y] => (x, y),
                _ => panic!("Invalid co-ordinates given")
            };

            let new_tile_index = tiles.len();
            tiles.push(RedTile { index: new_tile_index, x, y });
            for existing_tile in tiles[..new_tile_index].iter() {
                rects.push(Rect::new(&tiles[new_tile_index], &existing_tile));
            }
        }
        let largest_area = rects.iter().map(|r| r.area).max().unwrap();
        println!("The area of the largest rectangle is {}", largest_area);
    }
}