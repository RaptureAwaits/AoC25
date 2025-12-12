use aoc_shared::{get_input_filepath, line_iterator};

type TileIndex = usize;

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point::new(self.x.clone(), self.y.clone())
    }
}

struct RedTile {
    index: TileIndex,
    pos: Point,
}

struct Boundary {
    from: Point,
    to: Point,

    length: i64,
    orientation: String,
    direction: String
}

impl Boundary {
    fn new(from: Point, to: Point) -> Boundary {
        let length: i64;
        let orientation: String;
        let direction: String;

        if from.x == to.x {
            length = { from.y - to.y }.abs() + 1;
            orientation = "vertical".to_string();
            if from.y < to.y { direction = "up".to_string() } else { direction = "down".to_string() };

        } else if from.y == to.y {
            length = { from.x - to.x }.abs() + 1;
            orientation = "horizontal".to_string();
            if from.x < to.x { direction = "right".to_string() } else { direction = "left".to_string() };
        } else {
            panic!("A boundary must be an orthogonal vector")
        }

        Boundary { from, to, length, orientation , direction }
    }
}

struct RedRect {
    left: Boundary,
    top: Boundary,
    right: Boundary,
    bottom: Boundary,

    area: i64
}

impl RedRect {
    fn new(corner1: &RedTile, corner2: &RedTile) -> RedRect {
        let min_x = [corner1.pos.x, corner2.pos.x].iter().min().unwrap().clone();
        let max_x = [corner1.pos.x, corner2.pos.x].iter().max().unwrap().clone();
        let min_y = [corner1.pos.y, corner2.pos.y].iter().min().unwrap().clone();
        let max_y = [corner1.pos.y, corner2.pos.y].iter().max().unwrap().clone();
        RedRect {
            left: Boundary::new(Point { x: min_x, y: min_y }, Point { x: min_x, y: max_y }),
            top: Boundary::new(Point { x: min_x, y: max_y }, Point { x: max_x, y: max_y }),
            right: Boundary::new(Point { x: max_x, y: max_y }, Point { x: max_x, y: min_y }),
            bottom: Boundary::new(Point { x: max_x, y: min_y }, Point { x: min_x, y: min_y }),
            area: { max_x - min_x + 1 } * { max_y - min_y + 1 },
        }
    }
}


fn main() {
    let filepath = get_input_filepath();

    if let Ok(rows) = line_iterator(&filepath) {

        let mut tiles: Vec<RedTile> = vec![];
        let mut boundaries: Vec<Boundary> = vec![];
        let mut rects: Vec<RedRect> = vec![];

        for row in rows.map_while(Result::ok) {
            let coords = row.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            let point = match &coords[..] {
                &[x, y] => Point::new(x, y),
                _ => panic!("Invalid co-ordinates given")
            };

            let new_tile_index = tiles.len();
            tiles.push(RedTile { index: new_tile_index, pos: point });

            // Create rectangles relative to other tiles for part 1
            for existing_tile in tiles[..new_tile_index].iter() {
                rects.push(RedRect::new(&tiles[new_tile_index], &existing_tile));
            }

            // Create green area boundary for part 2
            if tiles.len() > 0 {
                boundaries.push(Boundary::new(tiles[new_tile_index - 1].pos.clone(), tiles[new_tile_index].pos.clone()));
            }
        }
        boundaries.push(Boundary::new(tiles[0].pos.clone(), tiles[boundaries.len()].pos.clone()));

        rects.sort_by(|r1, r2| r1.area.cmp(&r2.area));
        rects.reverse();

        // Part 1
        let largest_area = rects[0].area;
        println!("The area of the largest rectangle is {}", largest_area);

        // Part 2
        let leftmost_direction = boundaries.iter().filter(|b| b.orientation == "vertical").min_by(|b1, b2| b1.from.x.cmp(&b2.from.x)).unwrap().direction.clone();

        // This vector tells us which side the green area lies on for each direction of vector.
        // For example, if this is "down", then all down boundaries have the green area to their immediate right.
        // Ergo, the next boundary in the sequence must point right (a left vector here would imply a boundary with a lower x value), and the green area must lie above it.
        // Up and left boundaries naturally have the opposite property, with the green area lying to the left and below, respectively.

        // If we look out from each edge of our red rectangle and the first parallel boundary we see in each direction has green on the rectangle's side,
        // and no perpendicular boundaries totally intersect with the edge itself, then we know the rectangle lies wholly within a green area.
    }
}