use std::{
    fs,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn get_coords_between(start_coord: Coordinate, end_coord: Coordinate) -> Vec<Coordinate> {
    let diff = start_coord - end_coord;

    let mut coords_between = vec![];

    let sign;
    if diff.x == 0 {
        sign = diff.y.signum() * -1;
        let mut coord = start_coord;
        // Here we only go till -1 as
        // the last coord is not an
        // in "between" element
        for _ in 0..diff.y.abs() - 1 {
            coord = coord + Coordinate { x: 0, y: sign };
            coords_between.push(coord);
        }
    } else {
        sign = diff.x.signum() * -1;
        let mut coord = start_coord;
        for _ in 0..diff.x.abs() - 1 {
            coord = coord + Coordinate { x: sign, y: 0 };
            coords_between.push(coord);
        }
    }

    coords_between
}

#[derive(Debug, Clone, Copy)]
struct ChangeCoordinate {
    _max_x: i32,
    min_x: i32,
    _max_y: i32,
    min_y: i32,
    d_buffer: i32,
}

impl ChangeCoordinate {
    fn new(max_x: i32, min_x: i32, max_y: i32, min_y: i32, d_buffer: i32) -> Self {
        Self {
            _max_x: max_x,
            min_x,
            _max_y: max_y,
            min_y,
            d_buffer,
        }
    }

    fn change_coord_system(&self, coord: Coordinate) -> (usize, usize) {
        assert!(coord.x >= self.min_x);
        assert!(coord.y >= self.min_y);

        let x = coord.x - self.min_x;

        // As y is already in perfect
        // coord system
        //
        // y denotes rows, and
        // x denotes columns
        (coord.y as usize, (x + self.d_buffer) as usize)
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    for i in 0..rows {
        for j in 0..cols {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}

fn check_limits(coord: (usize, usize), rows: usize, cols: usize) -> bool {
    return coord.0 < rows - 1 && coord.1 < cols - 1;
}

fn drop_sand(
    grid: &Vec<Vec<char>>,
    drop_coord: (usize, usize),
    rows: usize,
    cols: usize,
) -> Option<(usize, usize)> {
    let mut sand_coord = drop_coord;

    // drop vertically
    while check_limits(sand_coord, rows, cols) {
        let next_char = grid[sand_coord.0 + 1][sand_coord.1];

        if next_char != '#' && next_char != 'o' {
            if sand_coord.0 == rows - 2 {
                panic!("invalid state");
            }

            sand_coord.0 += 1;
            continue;
        }

        let left_diagonal_char = grid[sand_coord.0 + 1][sand_coord.1 - 1];

        if left_diagonal_char != '#' && left_diagonal_char != 'o' {
            if sand_coord.0 == rows - 2 {
                panic!("invalid state");
            }

            sand_coord.0 += 1;

            sand_coord.1 -= 1;
            continue;
        }

        let right_diagonal_char = grid[sand_coord.0 + 1][sand_coord.1 + 1];

        if right_diagonal_char != '#' && right_diagonal_char != 'o' {
            if sand_coord.0 == rows - 2 {
                panic!("invalid state");
            }

            sand_coord.0 += 1;

            sand_coord.1 += 1;
            continue;
        }

        break;
    }

    return Some(sand_coord);
}

fn main() {
    let input_str =
        // fs::read_to_string("days/day14/example-input-day14").expect("should contain input");
    fs::read_to_string("days/day14/input-day14").expect("should contain input");

    let mut min_x = i32::MAX;
    let min_y = 0;

    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    let coords: Vec<Coordinate> = input_str
        .trim()
        .split("\n")
        .map(|pair_lines| {
            let mut previous_coord_opt = None;
            let coords: Vec<Coordinate> = pair_lines
                .split(" -> ")
                .map(|coord_str| {
                    let (x_str, y_str) = coord_str
                        .split_once(",")
                        .expect("expected comma separated coords");

                    let curr_coord = Coordinate {
                        x: x_str.parse::<i32>().expect("expected an i32"),
                        y: y_str.parse::<i32>().expect("expected an i32"),
                    };

                    let mut coords_between = match previous_coord_opt {
                        Some(previous_coord) => get_coords_between(previous_coord, curr_coord),
                        None => vec![],
                    };

                    previous_coord_opt = Some(curr_coord);
                    coords_between.push(curr_coord);

                    if curr_coord.x < min_x {
                        min_x = curr_coord.x;
                    }

                    if curr_coord.x > max_x {
                        max_x = curr_coord.x;
                    }

                    if curr_coord.y > max_y {
                        max_y = curr_coord.y;
                    }

                    coords_between
                })
                .flatten()
                .collect();

            coords
        })
        .flatten()
        .collect();

    let rows = max_y - min_y + 1;
    let cols = max_x - min_x + 1;

    let mut rows = rows as usize;
    let mut cols = cols as usize;

    let change_coord = ChangeCoordinate::new(max_x, min_x, max_y, min_y, 0);

    let mut grid = vec![vec!['.'; cols]; rows];

    for coord in coords {
        let (x, y) = change_coord.change_coord_system(coord);
        grid[x][y] = '#';
    }

    let (source_x, source_y) = change_coord.change_coord_system(Coordinate { x: 500, y: 0 });
    grid[source_x][source_y] = '+';

    // Two observations we make:
    // - First, max triangle base can
    // be calculated using AP series
    // formula with n = height, this
    // can give us how big the base
    // would be, for example it's
    // 1 + (11 - 1) * 2 = 21
    //
    // - Second, all obstacles in
    // input will be in range of max
    // triangle the salt can make,
    // this frees us from handling
    // the case where obstacles are
    // longer than triangle base

    // calculate base
    let base_len = 1 + rows * 2;

    // calculate required buffer space
    let needed_space_on_each_side = (base_len - 1) / 2;
    let left_buffer_len = needed_space_on_each_side - (source_y + 1) + 2;
    let right_buffer_len = needed_space_on_each_side - (cols - (source_y + 1));

    // make left buffer space
    for i in 0..rows {
        let mut left_buffer = vec!['.'; left_buffer_len];
        left_buffer.append(&mut grid[i]);
        grid[i] = left_buffer;
    }

    // make right buffer space
    for i in 0..rows {
        let mut right_buffer = vec!['.'; right_buffer_len];
        grid[i].append(&mut right_buffer);
    }

    cols += left_buffer_len + right_buffer_len;

    // add rock floor after a row
    grid.push(vec!['.'; cols]);
    grid.push(vec!['#'; cols]);

    rows += 2;

    let change_coord = ChangeCoordinate::new(max_x, min_x, max_y, min_y, (left_buffer_len) as i32);
    let (source_x, source_y) = change_coord.change_coord_system(Coordinate { x: 500, y: 0 });

    let mut cnt = 0;
    while let Some((x, y)) = drop_sand(&grid, (source_x, source_y), rows as usize, cols as usize) {
        cnt += 1;

        grid[x][y] = 'o';

        if x == source_x && y == source_y {
            break;
        }
    }

    // print_grid(&grid);

    println!("sand count: {:?}", cnt);
}
// 329
