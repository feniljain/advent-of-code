use std::{cmp, fs};

use nom::{
    branch::alt, bytes::complete::tag, character::complete as cc, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

#[derive(Debug, Clone)]
struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, Clone)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

fn parse_packet(line: &str) -> IResult<&str, Packet> {
    alt((
        map(
            delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")),
            Packet::List,
        ),
        map(cc::u32, Packet::Integer),
    ))(line)
}

// Some(true) -> correct order
// Some(false) -> incorrect order
// None -> both are same
fn check_integer_order(int_left: u32, int_right: u32) -> Option<bool> {
    if int_left == int_right {
        None
    } else {
        Some(int_left < int_right)
    }
}

fn check_list(list_left: &Vec<Packet>, list_right: &Vec<Packet>) -> Option<bool> {
    let min_len = cmp::min(list_left.len(), list_right.len());

    for i in 0..min_len {
        match check_order(&list_left[i], &list_right[i]) {
            Some(result) => {
                return Some(result);
            }
            None => continue,
        }
    }

    // We transversed list till have same number
    // of elements and we did not get any results
    // i.e. both have exact same integers till now

    if list_left.len() == min_len && list_right.len() == min_len {
        return None;
    }

    return Some(list_left.len() == min_len);
}

fn check_order(left_packet: &Packet, right_packet: &Packet) -> Option<bool> {
    match (left_packet, right_packet) {
        (Packet::List(list_left), Packet::List(list_right)) => check_list(&list_left, &list_right),
        (Packet::Integer(int_left), Packet::Integer(int_right)) => {
            check_integer_order(*int_left, *int_right)
        }
        (Packet::List(list_left), Packet::Integer(_int_right)) => {
            check_list(&list_left, &vec![right_packet.clone()])
        }
        (Packet::Integer(_int_left), Packet::List(list_right)) => {
            check_list(&vec![left_packet.clone()], list_right)
        }
    }
}

fn main() {
    let input_str =
        // fs::read_to_string("days/day13/example-input-day13").expect("should contain input");
    fs::read_to_string("days/day13/input-day13").expect("should contain input");
    let pairs: Vec<Pair> = input_str
        .split("\n\n")
        .map(|pair_lines| {
            let (line1, line2) = pair_lines
                .split_once("\n")
                .expect("invalid input: expected pair of packets");

            let res1 = parse_packet(line1).expect("invalid input");
            let res2 = parse_packet(line2).expect("invalid input");

            Pair {
                left: res1.1,
                right: res2.1,
            }
        })
        .collect();

    let n_pairs = pairs.len();
    let mut sum_of_indices = 0;
    for i in 0..n_pairs {
        let order = check_order(&pairs[i].left, &pairs[i].right).expect("internal error");
        if order {
            sum_of_indices += i + 1;
        }
    }

    println!("Sum Of Indices: {:?}", sum_of_indices);
}
