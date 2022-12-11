use std::fs;

#[derive(Debug)]
enum Operation {
    Noop,
    Addx(i32),
}

const START_CYCLE: i32 = 20;
const INCREMENT_FACTOR: i32 = 40;

fn main() {
    let input_str = fs::read_to_string("days/day10/example-input-day10").expect("should contain input");

    let operations = input_str
        .trim()
        .split("\n")
        .map(|line| {
            if line.contains("addx") {
                let (_, x) = line.split_once(" ").expect("invalid input");
                Operation::Addx(x.parse::<i32>().expect("invalid i32 input"))
            } else {
                Operation::Noop
            }
        })
        .collect::<Vec<Operation>>();

    let mut cycle_to_check = START_CYCLE;

    let mut value: i32 = 1;

    let mut cycles_completed = 0;

    let mut value_acc = 0;

    operations.iter().enumerate().for_each(|(_i, operation)| {
        let (new_value, condition) = match operation {
            Operation::Noop => {
                cycles_completed += 1;

                (value, cycle_to_check == cycles_completed)
            }
            Operation::Addx(val) => {
                cycles_completed += 2;
                let new_value = value + *val;

                let d_cycles = cycles_completed - cycle_to_check;
                (new_value, d_cycles == 0 || d_cycles == 1)
            }
        };

        if condition {
            let offset = cycle_to_check * value;
            value_acc += offset;
            cycle_to_check += INCREMENT_FACTOR;
        }

        value = new_value;
    });

    println!("Val Acc: {value_acc}");
}
