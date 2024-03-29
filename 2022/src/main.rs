use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::{Add, Range, Sub},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i64,
    y: i64,
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

fn parse_x_y(line: Vec<&str>, pos_x: usize, pos_y: usize) -> Coordinate {
    let (x_info, _) = line[pos_x]
        .split_once(",")
        .expect("expected comma at the end");

    let (_, x) = x_info
        .split_once("=")
        .expect("invalid input: expected '=' separated string");

    let x = x.parse::<i64>().expect("expected i64");

    let (_, y) = line[pos_y]
        .trim()
        .split_once("=")
        .expect("invalid input: expected '=' separated string");

    let y = y.parse::<i64>().expect("expected i64");

    Coordinate { x, y }
}

#[derive(Debug, Copy, Clone)]
struct Line {
    m: f64,
    b: f64,
}

impl Line {
    fn new(first_coord: Coordinate, second_coord: Coordinate) -> Self {
        let dy = (second_coord.y - first_coord.y) as f64;
        let dx = (second_coord.x - first_coord.x) as f64;

        let m = dy / dx;

        let b = (second_coord.y as f64) - (m * second_coord.x as f64);
        Line { m: dy / dx, b }
    }

    fn direction_of_coord(&self, coord: &Coordinate, debug: bool) -> f64 {
        let direction =
            // (coord.y as f64 * self.dx) - ((self.dy * coord.x as f64) + (self.b * self.dx));
            (coord.y as f64) - ((self.m * coord.x as f64) + (self.b));

        if debug {
            // println!(
            //     "line: y = {}*x + {:?} - direction: {direction}",
            //     self.m, self.b
            // );
        }

        if direction == 0.0 {
            return direction;
        }

        direction.signum()
    }

    fn calculate_x(&self, y: i64) -> i64 {
        ((y as f64 - self.b) / self.m) as i64
    }
}

#[derive(Debug)]
struct MaxMin {
    max_x: i64,
    min_x: i64,
    min_y: i64,
    max_y: i64,
}

impl MaxMin {
    fn new() -> Self {
        Self {
            max_x: i64::MIN,
            min_x: i64::MAX,
            min_y: i64::MAX,
            max_y: i64::MIN,
        }
    }

    fn update(&mut self, coord: &Coordinate) {
        if coord.x > self.max_x {
            self.max_x = coord.x;
        }

        if coord.y > self.max_y {
            self.max_y = coord.y;
        }

        if coord.x < self.min_x {
            self.min_x = coord.x;
        }

        if coord.y < self.min_y {
            self.min_y = coord.y;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SensorInfo {
    _coord: Coordinate,
    upper_left_line: Line,
    upper_right_line: Line,
    lower_left_line: Line,
    lower_right_line: Line,
    upper_coord: Coordinate,
    lower_coord: Coordinate,
    left_coord: Coordinate,
    right_coord: Coordinate,
    beacon_coord: Coordinate,
}

impl SensorInfo {
    fn find_collinear_point(&self, y_to_check: i64) -> [Coordinate; 4] {
        let x = self.upper_left_line.calculate_x(y_to_check);
        let first_intersection = Coordinate { x, y: y_to_check };
        let x = self.lower_left_line.calculate_x(y_to_check);
        let second_intersection = Coordinate { x, y: y_to_check };
        let x = self.lower_right_line.calculate_x(y_to_check);
        let third_intersection = Coordinate { x, y: y_to_check };
        let x = self.upper_right_line.calculate_x(y_to_check);
        let fourth_intersection = Coordinate { x, y: y_to_check };

        [
            first_intersection,
            second_intersection,
            third_intersection,
            fourth_intersection,
        ]
    }

    fn lies_in_coverage(&self, coord: &Coordinate, debug: bool) -> bool {
        let direction1 = self.upper_left_line.direction_of_coord(coord, debug);
        let direction2 = self.upper_right_line.direction_of_coord(coord, debug);
        let direction3 = -1_f64 * self.lower_left_line.direction_of_coord(coord, debug);
        let direction4 = -1_f64 * self.lower_right_line.direction_of_coord(coord, debug);

        if debug {
            // println!("Directions: {direction1} {direction2} {direction3} {direction4} for coord: {coord:?}");
        }

        // println!("Sensor Coords: {:?}", self.coord);
        if direction1 == 0.0 {
            if direction2 == 1.0 && direction3 == 1.0 && direction4 == 1.0 {
                return true;
            }
        }

        if direction2 == 0.0 {
            if direction1 == 1.0 && direction3 == 1.0 && direction4 == 1.0 {
                return true;
            }
        }

        if direction3 == 0.0 {
            if direction1 == 1.0 && direction2 == 1.0 && direction4 == 1.0 {
                return true;
            }
        }

        if direction4 == 0.0 {
            if direction1 == 1.0 && direction2 == 1.0 && direction3 == 1.0 {
                return true;
            }
        }

        if direction1 + direction2 + direction3 + direction4 == 4.0 {
            return true;
        }

        if *coord == self.upper_coord
            || *coord == self.lower_coord
            || *coord == self.left_coord
            || *coord == self.right_coord
        {
            return true;
        }

        return false;
    }
}

fn calculate_manhattan_distance(coord1: Coordinate, coord2: Coordinate) -> i64 {
    (coord1.x - coord2.x).abs() + (coord1.y - coord2.y).abs()
}

fn compute(
    sensor_info: &SensorInfo,
    mut ranges: Vec<Range<i64>>,
    // beacons: &HashMap<Coordinate, ()>,
) -> (Vec<Range<i64>>, bool) {
    let coords = sensor_info.find_collinear_point(ROW_TO_CHECK).to_vec();
    let mut first_coord = None;
    let mut second_coord = None;

    for coord in coords.clone() {
        let condition = sensor_info.lies_in_coverage(&coord, true);
        // println!("coord: {coord:?} - condition: {condition}");
        if condition {
            // println!("coord in range: {:?}", coord);
            if first_coord == None {
                first_coord = Some(coord);
                continue;
            }

            second_coord = Some(coord);
        }
    }

    // match (first_coord, second_coord) {
    //     (Some(first_coord), Some(second_coord)) => {
    //         // println!("first_coord: {first_coord:?} - second_coord: {second_coord:?}");
    //         for i in first_coord.x..=second_coord.x {
    //             // println!(
    //             //     "Coord1: {:?}",
    //             //     Coordinate {
    //             //         x: i,
    //             //         y,
    //             //     }
    //             // );
    //             let coord = Coordinate { x: i, y };
    //             if !beacons.contains_key(&coord) {
    //                 coords_set.insert(coord);
    //             }
    //         }
    //     }
    //     _ => (),
    // }

    //     let first_coord = first_coord.expect("invalid first coord");
    //     let second_coord = second_coord.expect("invalid second coord");

    match (first_coord, second_coord) {
        (Some(first_coord), Some(second_coord)) => {
            let calc_range = first_coord.x..(second_coord.x + 1);
            let beacon_overlaps = calc_range.contains(&sensor_info.beacon_coord.x);

            let l = ranges.len();
            if l == 0 {
                ranges.push(calc_range.clone());
                return (vec![calc_range], beacon_overlaps);
            }

            // let mut pos_to_remove = vec![];
            let mut stack = vec![ranges[0].clone()];
            println!("===");
            println!("ranges: {:?} - l: {l}", ranges);

            let mut i = 1;
            while i < l {
                let mut top = stack[0].clone();

                if top.end < ranges[i].start {
                    stack.push(ranges[i].clone());
                } else if top.end < ranges[i].end {
                    top.end = ranges[i].end;
                    stack.pop();
                    stack.push(top.clone());
                }

                i += 1;

                //                 let range = &ranges[i];
                //                 println!(
                //                     "Result from {:?} and {:?}: {:?}",
                //                     range,
                //                     calc_range,
                //                     range_intersect(range, &calc_range)
                //                 );

                //                 match range_intersect(&range, &calc_range) {
                //                     Some(new_range) => {
                //                         // calc_range = range;
                //                         ranges[i] = new_range.clone();
                //                         calc_range = new_range;
                //                         // pos_to_remove.push(i);
                //                         break;
                //                     }
                //                     None => {
                //                         insert = true;
                //                         // ranges.push(calc_range.clone());
                //                     }
                //                 }
            }

            return (ranges, beacon_overlaps);
        }
        _ => return (ranges, false),
    }
}

fn range_intersect(first_range: &Range<i64>, second_range: &Range<i64>) -> Option<Range<i64>> {
    let covers_completely = |first_range: &Range<i64>, second_range: &Range<i64>| -> bool {
        first_range.start <= second_range.start && first_range.end >= second_range.end
    };

    if covers_completely(first_range, second_range) {
        return Some(first_range.clone());
    } else if covers_completely(second_range, first_range) {
        return Some(second_range.clone());
    }

    // start is partially intersecting
    if first_range.start >= second_range.start && first_range.start <= second_range.end {
        return Some(second_range.start..first_range.end);
    }

    // end is partially intersecting
    if first_range.end >= second_range.start && first_range.end <= second_range.end {
        return Some(first_range.start..second_range.end);
    }

    return None;
}

// const ROW_TO_CHECK: i64 = 2000000;
const ROW_TO_CHECK: i64 = 10;

fn main() {
    let input_str =
        fs::read_to_string("days/day15/example-input-day15").expect("should contain input");
    // fs::read_to_string("days/day15/input-day15").expect("should contain input");

    // For this question, our convention of converting given
    // coordinate to indexable numbers is:
    // x -> col it belongs to
    // y -> row it belongs to

    let mut max_min = MaxMin::new();

    let mut sensors = vec![];
    let mut beacons: HashMap<Coordinate, ()> = HashMap::new();

    // let points_to_test: Vec<Coordinate> =
    input_str.trim().split("\n").for_each(|sensor_beacon_info| {
        let (sensor_info, beacon_info) = sensor_beacon_info
            .split_once(":")
            .expect("invalid input: expected ':' separated string");

        // Calculate Sensor Information
        let sensor_info: Vec<&str> = sensor_info.split(" ").collect();

        let sensor_coord = parse_x_y(sensor_info, 2, 3);

        // lower diagonals for sensor -> -1
        // upper diagonals for sensor -> 1

        // Calculate Beacon Information
        let beacon_info: Vec<&str> = beacon_info.split(" ").collect();

        let beacon_coord = parse_x_y(beacon_info, 5, 6);

        beacons.insert(beacon_coord, ());

        max_min.update(&sensor_coord);
        max_min.update(&beacon_coord);

        let dist_from_beacon = calculate_manhattan_distance(sensor_coord, beacon_coord);

        let upper_coord = sensor_coord
            + Coordinate {
                x: 0,
                y: -1 * dist_from_beacon,
            };
        let lower_coord = sensor_coord
            + Coordinate {
                x: 0,
                y: dist_from_beacon,
            };
        let left_coord = sensor_coord
            + Coordinate {
                x: -1 * dist_from_beacon,
                y: 0,
            };
        let right_coord = sensor_coord
            + Coordinate {
                x: dist_from_beacon,
                y: 0,
            };

        // println!("==========");
        // println!("Sensor Coords: {sensor_coord:?} - dist: {dist_from_beacon}");
        // println!("{upper_coord:?} {lower_coord:?} {left_coord:?} {right_coord:?}");

        // max_min.update(&upper_coord);
        // max_min.update(&lower_coord);
        // max_min.update(&left_coord);
        // max_min.update(&right_coord);

        let upper_left_line = Line::new(upper_coord, left_coord);
        let upper_right_line = Line::new(upper_coord, right_coord);
        let lower_left_line = Line::new(lower_coord, left_coord);
        let lower_right_line = Line::new(lower_coord, right_coord);

        // println!("{upper_left_line:?}");
        // println!("{upper_right_line:?}");
        // println!("{lower_left_line:?}");
        // println!("{lower_right_line:?}");

        // println!("direction: {direction:?}");

        let sensor_info = SensorInfo {
            _coord: sensor_coord,
            upper_left_line,
            upper_right_line,
            lower_left_line,
            lower_right_line,
            upper_coord,
            lower_coord,
            left_coord,
            right_coord,
            beacon_coord,
        };

        sensors.push(sensor_info.clone());

        // println!("==========");

        // coords_to_consider
    });
    // .flatten()
    // .collect();

    // let mut coords_lying_on_row: HashSet<Coordinate> = HashSet::new();
    let mut cnt = 0;
    // let mut a = vec![];
    // let ranges: Vec<Range<i64>> = vec![];

    let ranges = sensors.iter().fold(vec![], |ranges, sensor_info| {
        let (ranges, beacon_overlaps) = compute(sensor_info, ranges);
        if beacon_overlaps {
            cnt += 1;
        }

        ranges
    });
    // .collect();

    let a: i64 = ranges.iter().map(|range| range.end - range.start).sum();
    println!("ranges: {ranges:?} - {:?}", a - cnt);

    // for range in ranges {
    // }

    // .filter_map(|range| {
    //     range.and_then(|range| {
    //         println!("range: {:?}", range);
    //         // range.chain
    //         a.extend(range.clone());
    //         return Some(range.end - range.start);
    //     })
    // });
    // .fold(Range::new, f)
    // .sum();

    // println!("{:?}", sum - cnt);

    // println!("len: {:?}", coords_lying_on_row.len());
}
