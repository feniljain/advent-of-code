use std::fs;

#[derive(Debug)]
struct Instr {
    quantitiy: u32,
    from: u32,
    to: u32,
}

#[derive(Debug)]
struct Crate(char);

struct Parser {
    line_char_count: usize,
}

impl Parser {
    fn new() -> Self {
        Parser { line_char_count: 1 }
    }

    fn parse_line(&mut self, line: &str) -> Vec<Option<Crate>> {
        let mut collected_crate = String::new();
        let mut crates = vec![];

        for ch in line.chars() {
            if self.line_char_count % 4 == 0 || (self.line_char_count == (line.len())) {
                if collected_crate.trim() != "" {
                    let mut chars_iter = collected_crate.chars();
                    // [
                    let opening_bracket = chars_iter.next().unwrap();
                    assert_eq!(opening_bracket, '[');
                    // crate
                    let krate = chars_iter.next().unwrap();
                    crates.push(Some(Crate(krate)));
                } else {
                    crates.push(None);
                }
                collected_crate = String::new();
            } else {
                collected_crate.push(ch);
            }

            self.line_char_count += 1;
        }

        crates
    }

    fn parse_crates(&mut self, lines: Vec<&str>) -> Vec<Vec<Option<Crate>>> {
        let mut crate_stacks = vec![];
        for line in lines {
            crate_stacks.push(self.parse_line(line));
            self.line_char_count = 1;
        }

        crate_stacks
    }
}

fn transpose<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn move_crates(mut crate_stacks: Vec<Vec<Crate>>, instr: Instr) -> Vec<Vec<Crate>> {
    let mut batched_crates = vec![];
    (0..instr.quantitiy).for_each(|_| {
        let crate_to_be_moved = crate_stacks[(instr.from - 1) as usize].pop().unwrap();
        batched_crates.push(crate_to_be_moved);
    });

    batched_crates.reverse();

    for krate in batched_crates {
        crate_stacks[(instr.to - 1) as usize].push(krate);
    }

    crate_stacks
}

fn main() {
    let mut input_str =
        fs::read_to_string("days/day5/input-day5").expect("should contain input");
    input_str = input_str.trim_end().to_string();

    let split_idx = match input_str.find("\n\n") {
        Some(x) => x,
        None => 0,
    };

    let (crate_stacks, instrs_unprocessed) = input_str.split_at(split_idx);

    // Prepocessing Crate Stacks
    let mut lines_with_numbers: Vec<&str> = crate_stacks.split("\n").collect();
    let _ = lines_with_numbers.pop();
    let lines = lines_with_numbers;

    let mut parser = Parser::new();
    let crate_stacks = parser.parse_crates(lines);

    let mut crate_stacks = transpose(crate_stacks);
    println!("Crate Stacks: {:?}", crate_stacks);

    // Prepocessing Instructions
    let instrs_unprocessed = instrs_unprocessed.trim();

    let mut instrs = vec![];
    instrs_unprocessed
        .split("\n")
        .for_each(|instr_unprocessed| {
            let instr_arr: Vec<&str> = instr_unprocessed.split_whitespace().collect();
            instrs.push(Instr {
                quantitiy: instr_arr[1].parse::<u32>().unwrap(),
                from: instr_arr[3].parse::<u32>().unwrap(),
                to: instr_arr[5].parse::<u32>().unwrap(),
            });
        });

    for instr in instrs {
        crate_stacks = move_crates(crate_stacks, instr);
    }
    println!("Crate Stacks: {:?}", crate_stacks);

    for crate_stack in crate_stacks {
        print!("{:?}", crate_stack.last().unwrap().0);
    }

    // println!("Total Count: {:?}", cnt);
    // SSCGWJCRB
}
