#![feature(iter_array_chunks)]

use std::fs;

fn contains_another(x1: u32, x2: u32, y1: u32, y2: u32) -> bool {
    x1 <= y1 && x2 >= y2
}

fn contains_overlap(x1: u32, x2: u32, y1: u32, y2: u32) -> bool {
    if x1 > y1 {
        return y2 >= x1;
    } else if x1 < y1 {
        return y1 <= x2;
    } else {
        return true;
    }
}

fn main() {
    let input_str =
        fs::read_to_string("days/day4/input-day4").expect("should contain input");

    let mut cnt = 0;

    input_str.trim().split("\n").for_each(|pair_assignments| {
        let assignments: Vec<&str> = pair_assignments.split(",").collect();
        let first_assignement = assignments[0];
        let second_assignment = assignments[1];

        let first_assignment_limits: Vec<&str> = first_assignement.split("-").collect();
        let second_assigement_limits: Vec<&str> = second_assignment.split("-").collect();

        let first_assignment_lower_limit = first_assignment_limits[0].parse::<u32>().unwrap();
        let first_assignment_upper_limit = first_assignment_limits[1].parse::<u32>().unwrap();

        let second_assignment_lower_limit = second_assigement_limits[0].parse::<u32>().unwrap();
        let second_assignment_upper_limit = second_assigement_limits[1].parse::<u32>().unwrap();

        let first_contains_second = contains_another(
            first_assignment_lower_limit,
            first_assignment_upper_limit,
            second_assignment_lower_limit,
            second_assignment_upper_limit,
        );

        if first_contains_second
            || contains_another(
                second_assignment_lower_limit,
                second_assignment_upper_limit,
                first_assignment_lower_limit,
                first_assignment_upper_limit,
            )
            || contains_overlap(
                first_assignment_lower_limit,
                first_assignment_upper_limit,
                second_assignment_lower_limit,
                second_assignment_upper_limit,
            )
        {
            cnt += 1;
        }
    });

    println!("Total Count: {:?}", cnt);
}
