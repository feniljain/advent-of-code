use std::{
    cmp::{self, Ordering},
    fs,
};

use nom::{
    branch::alt, bytes::complete::tag, character::complete as cc, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

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
    let mut packets: Vec<Packet> = input_str
        .split("\n\n")
        .map(|pair_lines| {
            let (line1, line2) = pair_lines
                .split_once("\n")
                .expect("invalid input: expected pair of packets");

            let res1 = parse_packet(line1).expect("invalid input");
            let res2 = parse_packet(line2).expect("invalid input");

            vec![res1.1, res2.1]
        })
        .flatten()
        .collect();

    let packet_2 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let packet_6 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);

    packets.push(packet_2.clone());
    packets.push(packet_6.clone());

    packets.sort_by(|first, second| {
        // Here we expect directly coz it's always mentioned that we
        // need to continue computation if inputs are same
        // ( inputs = integer/list ) and None is used to represent
        // that state, so final state can never be None
        let order = check_order(first, second).expect("internal error");
        if order {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });

    let decoder_key = packets.iter().enumerate().fold(1, |acc, (i, packet)| {
        let packet_2_comparison = check_list(&vec![packet.clone()], &vec![packet_2.clone()]);
        let packet_6_comparison = check_list(&vec![packet.clone()], &vec![packet_6.clone()]);

        if None == packet_2_comparison || None == packet_6_comparison {
            acc * (i + 1)
        } else {
            acc * 1
        }
    });

    println!("Decoder Key: {:?}", decoder_key);
}
