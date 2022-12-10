use std::{collections::HashSet, fs, ops::Add};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Instr<'a> {
    direction: &'a str,
    steps: usize,
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn get_offset_from_current_coordinate(direction: &str) -> Coordinate {
    match direction {
        "R" => Coordinate { x: 1, y: 0 },
        "L" => Coordinate { x: -1, y: 0 },
        "U" => Coordinate { x: 0, y: 1 },
        "D" => Coordinate { x: 0, y: -1 },
        _ => panic!("incorrect direction passed"),
    }
}

// fn print_grid(grid: Vec<Vec<char>>) {
//     for line in grid {
//         for char in line {
//             print!("{}", char);
//         }
//         println!();
//     }
// }

// fn make_grid(curr_coords: [Coordinate; TOTAL_KNOTS]) {
//     let mut grid: Vec<Vec<char>> = vec![];
//     for _ in 0..TOTAL_KNOTS + 6 {
//         grid.push(vec!['.'; TOTAL_KNOTS + 6]);
//     }

//     for (i, coord) in curr_coords.iter().enumerate() {
//         println!("x: {:?} y: {:?}", coord.x, coord.y);
//         let mut coord_x = coord.x;
//         let mut coord_y = coord.y;

//         // if coord_x < 0 {
//         // coord_x = coord.x + 16;
//         // }

//         // if coord_y < 0 {
//         // coord_y = coord.y + 16;
//         // }

//         println!("after: x: {:?} y: {:?}", coord_x, coord_y);

//         grid[coord_x as usize][coord_y as usize] = char::from_digit(i as u32, 10).unwrap();
//     }

//     print_grid(grid);
// }

const TOTAL_KNOTS: usize = 10;

fn main() {
    let input_str =
        fs::read_to_string("days/day9/input-day9").expect("should contain input");

    let instrs = input_str
        .trim()
        .split("\n")
        .map(|line| {
            let (direction, steps) = line.split_once(" ").expect("invalid input");
            Instr {
                direction,
                steps: steps.parse::<usize>().expect("invalid steps input"),
            }
        })
        .collect::<Vec<Instr>>();

    let mut positions_visited: HashSet<Coordinate> = HashSet::new();
    positions_visited.insert(Coordinate { x: 0, y: 0 });

    let mut curr_coords = [Coordinate { x: 0, y: 0 }; TOTAL_KNOTS];

    instrs.iter().for_each(|instr| {
        for _step in 0..instr.steps {
            // update head
            let offset = get_offset_from_current_coordinate(instr.direction);
            curr_coords[0] = curr_coords[0] + offset;

            // update remaining knots
            for i in 1..TOTAL_KNOTS {
                let dx = curr_coords[i - 1].x - curr_coords[i].x;
                let dy = curr_coords[i - 1].y - curr_coords[i].y;

                if dx.abs() >= 2 || dy.abs() >= 2 {
                    if dx.abs() > 1 || dy.abs() > 1 {
                        curr_coords[i].x += dx.signum();
                        curr_coords[i].y += dy.signum();
                    }
                }
            }

            positions_visited.insert(curr_coords[TOTAL_KNOTS - 1]);
        }
    });

    println!("Positions Visited: {:?}", positions_visited.len());
}
