#![feature(iter_array_chunks)]

use std::{collections::HashSet, fs};

fn main() {
    let mut input_str =
        fs::read_to_string("days/day6/input-day6").expect("should contain input");
    input_str = input_str.trim().to_string();

    let mut chars_iter = input_str.chars();
    let mut sliding_buffer = vec![];

    const DISTINCT_CHARS_TO_MATCH: usize = 14;

    let mut idx_cnt = 0;
    chars_iter.find(|ch| {
        idx_cnt += 1;

        sliding_buffer.push(*ch);

        if sliding_buffer.len() == DISTINCT_CHARS_TO_MATCH {
            let set: HashSet<char> = HashSet::from_iter(sliding_buffer.clone().into_iter());

            if set.len() == DISTINCT_CHARS_TO_MATCH {
                println!("{:?}", set);
                return true;
            }

            sliding_buffer.remove(0);
        }

        return false;
    });

    println!("Position: {:?}", idx_cnt);
}
