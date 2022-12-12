#![feature(fn_traits)]

use std::{fmt::Debug, fs};

struct Monkey {
    items_worry_levels: Vec<u128>,
    operation: Box<dyn Fn(u128) -> u128>,
    test: Box<dyn Fn(u128) -> usize>,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("starting_items", &self.items_worry_levels)
            .field("operation", &(self.operation)(2u128))
            .field("test", &(self.test)(2u128))
            .finish()
    }
}

fn make_operation_closure(operation: &str, var: &str) -> Box<dyn Fn(u128) -> u128> {
    let var_res = var.parse::<u128>();
    if let Ok(var) = var_res {
        match operation {
            "+" => Box::new(move |x| var + x),
            "-" => Box::new(move |x| var - x),
            "*" => Box::new(move |x| var * x),
            "/" => Box::new(move |x| (var as f32 / x as f32).round() as u128),
            _ => panic!("incorrect operation"),
        }
    } else {
        match operation {
            "+" => Box::new(|x| x + x),
            "-" => Box::new(|_| 0),
            "*" => Box::new(|x| x * x),
            "/" => Box::new(|_| 1),
            _ => panic!("incorrect operation"),
        }
    }
}

// fn print_monkey_state(monkeys: &Vec<Monkey>) {
//     // println!("========");
//     for (i, monkey) in monkeys.iter().enumerate() {
//         println!("Monkey {}: {:?}", i, monkey.items_worry_levels);
//     }
//     // println!("========");
// }

// fn print_round_state(monkeys: &Vec<Monkey>, inspection_count: &Vec<u128>) {
//     for (i, _monkey) in monkeys.iter().enumerate() {
//         println!("Monkey {i} inspected items {} times", inspection_count[i]);
//         // println!("Monkey {}: {:?}", i, monkey.items_worry_levels);
//     }
// }

const TOTAL_ROUNDS: u32 = 10000;

fn main() {
    let input_str =
        fs::read_to_string("days/day11/input-day11").expect("should contain input");

    let mut lcm = 1;

    let mut monkeys = input_str
        .trim()
        .split("\n\n")
        .map(|monkey_info_lines| {
            let lines = monkey_info_lines.split("\n").skip(1).collect::<Vec<&str>>();

            // starting items
            let (_, numbers_str_list) = lines[0]
                .split_once(":")
                .expect("invalid input: ':' separation not present");
            let numbers_list: Vec<u128> = numbers_str_list
                .trim()
                .split(",")
                .map(|number_str| {
                    number_str
                        .trim()
                        .parse::<u128>()
                        .expect("invalid input: expected a number")
                })
                .collect::<Vec<u128>>();

            // operation
            let (_, operation_str) = lines[1]
                .split_once(":")
                .expect("invalid input, ':' separation not present");
            let (_, expr_str) = operation_str
                .split_once("=")
                .expect("invalid input, '=' separation not present");

            let expr: Vec<&str> = expr_str.trim().split_ascii_whitespace().skip(1).collect();

            let operation = make_operation_closure(expr[0], expr[1]);

            // test
            let (_, divisible_by_str) = lines[2]
                .split_once("Test: divisible by ")
                .expect("invalid input, given string not present");

            let divisible_by = divisible_by_str
                .parse::<u128>()
                .expect("expected divisible_by integer");

            lcm*=divisible_by;

            let true_case = lines[3]
                .split_once("If true: throw to monkey ")
                .expect("invalid input, given string not present");

            let true_monkey_idx = true_case.1.parse::<usize>().expect("expected an u128");

            let false_case = lines[4]
                .split_once("If false: throw to monkey ")
                .expect("invalid input, given string not present");

            let false_monkey_idx = false_case.1.parse::<usize>().expect("expected an u128");

            let monkey = Monkey {
                items_worry_levels: numbers_list,
                operation,
                test: Box::new(move |x| {
                    if x % divisible_by == 0 {
                        true_monkey_idx
                    } else {
                        false_monkey_idx
                    }
                }),
            };

            monkey
        })
        .collect::<Vec<Monkey>>();

    let n_monkeys = monkeys.len();

    let mut inspection_count = vec![0u128; n_monkeys];

    for _round in 0..TOTAL_ROUNDS {
        for i in 0..n_monkeys {
            let monkey = &monkeys[i];
            let mut items_to_pass = vec![];

            monkey
                .items_worry_levels
                .iter()
                .for_each(|item_worry_level| {
                    let worry_level_after_hold = (monkey.operation)(*item_worry_level) % lcm;

                    // let worry_level_after_leave = (worry_level_after_hold as f32 / 3.0) as u128;

                    let pass_to_monkey_idx = (monkey.test)(worry_level_after_hold) as usize;

                    items_to_pass.push((pass_to_monkey_idx, worry_level_after_hold % lcm));
                    inspection_count[i] += 1;
                });

            monkeys[i].items_worry_levels = vec![];

            for item_to_pass in items_to_pass {
                monkeys[item_to_pass.0]
                    .items_worry_levels
                    .push(item_to_pass.1);
            }
        }

        // print_monkey_state(&monkeys);
        // print_inspection_state(&monkeys);

        // println!("After round: {}", _round+1);
        // print_round_state(&monkeys, &inspection_count);
    }

    inspection_count.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!(
        "Level Of Monkey Business: {}",
        inspection_count[0] * inspection_count[1]
    );
}
