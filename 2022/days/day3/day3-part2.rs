#![feature(iter_array_chunks)]

use std::fs;

fn get_list_idx(char: char) -> usize {
    if char.is_ascii_uppercase() {
        return (char as usize) - 65;
    } else if char.is_ascii_lowercase() {
        return (char as usize) - 71;
    }

    usize::MAX
}

fn get_score(char: char) -> usize {
    if char.is_ascii_uppercase() {
        return (char as usize) - 38;
    } else if char.is_ascii_lowercase() {
        return (char as usize) - 96;
    }

    usize::MAX
}

fn find_overlap_char(first_str: &str, second_str: &str, third_str: &str) -> char {
    let mut chars_cnt = vec![(0, 0); 52];
    first_str.chars().for_each(|ch| {
        let chars_idx = get_list_idx(ch);
        chars_cnt[chars_idx].0 = chars_cnt[chars_idx].0 + 1;
    });

    second_str.chars().for_each(|ch| {
        let chars_idx = get_list_idx(ch.clone());
        if chars_cnt[chars_idx].0 > 0 {
            chars_cnt[chars_idx].1 += 1;
        }
    });

    let overlap_char_opt = third_str.chars().find(|ch| {
        let chars_idx = get_list_idx(*ch);
        if chars_cnt[chars_idx].0 > 0 && chars_cnt[chars_idx].1 > 0 {
            return true;
        }

        false
    });

    match overlap_char_opt {
        Some(x) => x,
        None => '\n',
    }
}

fn main() {
    let input_str = fs::read_to_string("days/day3/input-day3").expect("should contain input");

    let mut total_score = 0;

    input_str.trim().split("\n").array_chunks::<3>().for_each(
        |[first_str, second_str, third_str]| {
            let overlap_char = find_overlap_char(first_str, second_str, third_str);
            total_score += get_score(overlap_char);
        },
    );

    println!("Total Score: {:?}", total_score);
}
