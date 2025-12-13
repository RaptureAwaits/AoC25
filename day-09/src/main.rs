use::std::fmt;
use aoc_shared::{get_input_filepath, line_iterator};

#[derive(Clone, Copy, PartialEq)]
enum Orient {
    Horizontal,
    Vertical
}

#[derive(Clone, PartialEq, Eq)]
enum Direct {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Direct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Up => write!(f, "Up"),
            Self::Right => write!(f, "Right"),
            Self::Down => write!(f, "Down"),
            Self::Left => write!(f, "Left"),
        }
    }
}

#[derive(PartialEq)]
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

#[derive(Clone, PartialEq)]
struct Boundary {
    from: Point,
    to: Point,

    length: i64,
    orientation: Orient,
    direction: Direct
}

impl fmt::Display for Boundary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) -> ({}, {})", self.from.x, self.from.y, self.to.x, self.to.y)
    }
}

impl Boundary {
    fn new(from: Point, to: Point) -> Boundary {
        let length: i64;
        let orientation: Orient;
        let direction: Direct;

        if from.x == to.x {
            length = { from.y - to.y }.abs() + 1;
            orientation = Orient::Vertical;
            if from.y < to.y { direction = Direct::Up } else { direction = Direct::Down };

        } else if from.y == to.y {
            length = { from.x - to.x }.abs() + 1;
            orientation = Orient::Horizontal;
            if from.x < to.x { direction = Direct::Right } else { direction = Direct::Left };
        } else {
            panic!("A boundary must be an orthogonal vector")
        }

        Boundary { from, to, length, orientation , direction }
    }

    fn get_axis_span(&self) -> (i64, i64) {
        let min: i64;
        let max: i64;
        if self.orientation == Orient::Vertical {
            min = *[self.from.y, self.to.y].iter().min().unwrap();
            max = *[self.from.y, self.to.y].iter().max().unwrap();
        } else {
            min = *[self.from.x, self.to.x].iter().min().unwrap();
            max = *[self.from.x, self.to.x].iter().max().unwrap();
        }
        (min, max)
    }

    fn is_coord_in_span(&self, coord: i64) -> bool {
        let (min, max) = self.get_axis_span();
        min < coord && coord < max
    }

    fn has_parallel_overlap(&self, other: &Boundary) -> bool {
        if self.orientation != other.orientation {
            return false;
        }

        if self.orientation == Orient::Vertical {
            let (y1_min, y1_max) = self.get_axis_span();
            let (y2_min, y2_max) = other.get_axis_span();
            if y1_min < y2_max && y1_max > y2_min {
                return true;
            }
        } else {
            let (x1_min, x1_max) = self.get_axis_span();
            let (x2_min, x2_max) = other.get_axis_span();
            if x1_min < x2_max && x1_max > x2_min {
                return true;
            }
        }
        false
    }

    fn intersects(&self, other: &Boundary) -> bool {
        let (h_vec, v_vec) = match (self.orientation, other.orientation) {
            (Orient::Vertical, Orient::Vertical) => return false,
            (Orient::Vertical, Orient::Horizontal) => (other, self),
            (Orient::Horizontal, Orient::Vertical) => (self, other),
            (Orient::Horizontal, Orient::Horizontal) => return false,
        };

        h_vec.is_coord_in_span(v_vec.from.x) && v_vec.is_coord_in_span(h_vec.from.y)
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
    fn new(corner1: &Point, corner2: &Point) -> RedRect {
        let min_x = [corner1.x, corner2.x].iter().min().unwrap().clone();
        let max_x = [corner1.x, corner2.x].iter().max().unwrap().clone();
        let min_y = [corner1.y, corner2.y].iter().min().unwrap().clone();
        let max_y = [corner1.y, corner2.y].iter().max().unwrap().clone();
        RedRect {
            left: Boundary::new(Point { x: min_x, y: min_y }, Point { x: min_x, y: max_y }),
            top: Boundary::new(Point { x: min_x, y: max_y }, Point { x: max_x, y: max_y }),
            right: Boundary::new(Point { x: max_x, y: max_y }, Point { x: max_x, y: min_y }),
            bottom: Boundary::new(Point { x: max_x, y: min_y }, Point { x: min_x, y: min_y }),
            area: { max_x - min_x + 1 } * { max_y - min_y + 1 },
        }
    }

    fn is_point_in_area(&self, point: &Point) -> bool {
        if self.top.is_coord_in_span(point.x) && self.left.is_coord_in_span(point.y) {
            true
        } else {
            false
        }
    }
}

struct RectEdgeChecker {
    left: Direct,
    top: Direct,
    right: Direct,
    bottom: Direct,
}

impl RectEdgeChecker {
    fn new(left_vector_direct: Direct) -> RectEdgeChecker {
        if left_vector_direct == Direct::Up {
            RectEdgeChecker {
                left: Direct::Up,
                top: Direct::Right,
                right: Direct::Down,
                bottom: Direct::Left,
            }
        } else {
            RectEdgeChecker {
                left: Direct::Down,
                top: Direct::Left,
                right: Direct::Up,
                bottom: Direct::Right,
            }
        }
    }
}

fn main() {
    let filepath = get_input_filepath();

    if let Ok(rows) = line_iterator(&filepath) {

        let mut tiles: Vec<Point> = vec![];
        let mut boundaries: Vec<Boundary> = vec![];
        let mut rects: Vec<RedRect> = vec![];

        for row in rows.map_while(Result::ok) {
            let coords = row.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            let point = match &coords[..] {
                &[x, y] => Point::new(x, y),
                _ => panic!("Invalid co-ordinates given")
            };

            let new_tile_index = tiles.len();
            tiles.push(point);

            // Create rectangles relative to other tiles for part 1
            for existing_tile in tiles[..new_tile_index].iter() {
                rects.push(RedRect::new(&tiles[new_tile_index], &existing_tile));
            }

            // Create green area boundary for part 2
            if tiles.len() > 1 {
                boundaries.push(Boundary::new(tiles[tiles.len() - 2].clone(), tiles[tiles.len() - 1].clone()));
            }
        }
        boundaries.push(Boundary::new(tiles[0].clone(), tiles[boundaries.len()].clone()));

        rects.sort_by(|r1, r2| r1.area.cmp(&r2.area));
        rects.reverse();

        // Part 1
        let largest_area = rects[0].area;
        println!("The area of the largest rectangle is {}", largest_area);

        // Part 2
        let mut vertical_boundaries = boundaries.clone();
        vertical_boundaries.retain(|b| b.orientation == Orient::Vertical);
        vertical_boundaries.sort_by(|b1, b2| b1.from.x.cmp(&b2.from.x));  // Left -> Right

        let rec_check = RectEdgeChecker::new(vertical_boundaries[0].direction.clone());

        let mut horizontal_boundaries = boundaries.clone();
        horizontal_boundaries.retain(|b| b.orientation == Orient::Horizontal);
        horizontal_boundaries.sort_by(|b1, b2| b1.from.y.cmp(&b2.from.y));  // Bottom -> Top

        for rect in rects {
            if validate_rect(&rect, &rec_check, &vertical_boundaries, &horizontal_boundaries) {
                println!("The largest rectangle contained within the green area has area {}", rect.area);
                break;
            }
        }
    }
}

fn validate_rect(rect: &RedRect, checker: &RectEdgeChecker, v_bounds: &Vec<Boundary>, h_bounds: &Vec<Boundary>) -> bool {
    let left_bounds = v_bounds.iter()
        .filter(|b| b.from.x <= rect.left.from.x && b.has_parallel_overlap(&rect.left))
        .collect::<Vec<&Boundary>>();
    let nearest_left_bound: &Boundary;
    if let Some(b) = left_bounds.last() { nearest_left_bound = b } else {
        // println!("Failed: no valid left bound");
        return false
    };

    let top_bounds = h_bounds.iter()
        .filter(|b| b.from.y >= rect.top.from.y && b.has_parallel_overlap(&rect.top))
        .collect::<Vec<&Boundary>>();
    let nearest_top_bound: &Boundary;
    if let Some(b) = top_bounds.first() { nearest_top_bound = b } else {
        // println!("Failed: no valid top bound");
        return false
    };

    let right_bounds = v_bounds.iter()
        .filter(|b| b.from.x >= rect.right.from.x && b.has_parallel_overlap(&rect.right))
        .collect::<Vec<&Boundary>>();
    let nearest_right_bound: &Boundary;
    if let Some(b) = right_bounds.first() { nearest_right_bound = b } else {
        // println!("Failed: no valid right bound");
        return false
    };

    let bottom_bounds = h_bounds.iter()
        .filter(|b| b.from.y <= rect.bottom.from.y && b.has_parallel_overlap(&rect.bottom))
        .collect::<Vec<&Boundary>>();
    let nearest_bottom_bound: &Boundary;
    if let Some(b) = bottom_bounds.last() { nearest_bottom_bound = b } else {
        // println!("Failed: no valid bottom bound");
        return false
    };

    // println!("| x = {}", nearest_left_bound.from.x);
    // println!("|");
    // println!("v  v x = {}", rect.left.from.x);
    // println!("|-----------| <- y = {}", nearest_top_bound.from.y);
    // println!("|           |");
    // println!("|  |-----|  | <- y = {}", rect.top.from.y);
    // println!("|  |     |  |");
    // println!("|  |_____|  | <- y = {}", rect.bottom.from.y);
    // println!("|           |");
    // println!("|___________| <- y = {}", nearest_bottom_bound.from.y);
    // println!("         ^  ^");
    // println!("         |  |");
    // println!("         | x = {}", rect.right.from.x);
    // println!("            |");
    // println!("            | x = {}", nearest_right_bound.from.x);

    if nearest_left_bound.direction != checker.left {
        // println!("Failed: nearest left bound goes in the wrong direction");
        return false;
    } else if nearest_top_bound.direction != checker.top {
        // println!("Failed: nearest top bound goes in the wrong direction");
        return false;
    } else if nearest_right_bound.direction != checker.right {
        // println!("Failed: nearest right bound goes in the wrong direction");
        return false;
    } else if nearest_bottom_bound.direction != checker.bottom {
        // println!("Failed: nearest bottom bound goes in the wrong direction");
        return false;
    }


    for h_vec in h_bounds {
        // Do any horizontal boundary vectors cross the vertical rectangle edges?
        if rect.left.intersects(h_vec) {
            // println!("Failed: left edge {} intersects with horizontal boundary {}", rect.left, h_vec);
            return false
        }
        if rect.right.intersects(h_vec) {
            // println!("Failed: right edge {} intersects with horizontal boundary {}", rect.right, h_vec);
            return false
        }
    }

    for v_vec in v_bounds {
        // Do any vertical boundary vectors cross the horizontal rectangle edges?
        if rect.top.intersects(v_vec) {
            // println!("Failed: top edge {} intersects with vertical boundary {}", rect.top, v_vec);
            return false
        }
        if rect.bottom.intersects(v_vec) {
            // println!("Failed: bottom edge {} intersects with vertical boundary {}", rect.bottom, v_vec);
            return false
        }

        if rect.is_point_in_area(&v_vec.from) || rect.is_point_in_area(&v_vec.to) {
            // println!("Failed: red tile lies within rectangle");
            return false
        }
    }
    true
}