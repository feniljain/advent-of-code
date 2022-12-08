use std::fs;

fn get_winning_move(opponent_choice: &str) -> &str {
    match opponent_choice {
        "A" => "Y",
        "B" => "Z",
        "C" => "X",
        _ => "",
    }
}

fn get_losing_move(opponent_choice: &str) -> &str {
    match opponent_choice {
        "A" => "Z",
        "B" => "X",
        "C" => "Y",
        _ => "",
    }
}

fn get_tie_move(opponent_choice: &str) -> &str {
    match opponent_choice {
        "A" => "X",
        "B" => "Y",
        "C" => "Z",
        _ => "",
    }
}

fn get_score_from_instr(instr: &str) -> usize {
    match instr {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => usize::MAX,
    }
}

fn get_move_score(your_choice: &str) -> usize {
    match your_choice {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => usize::MAX,
    }
}

fn get_move<'a, 'b: 'a>(desired_round_outcome: &'b str, opponent_choice: &'b str) -> &'a str {
    match get_score_from_instr(desired_round_outcome) {
        0 => get_losing_move(opponent_choice),
        3 => get_tie_move(opponent_choice),
        6 => get_winning_move(opponent_choice),
        _ => "",
    }
}

fn main() {
    let input_str =
        fs::read_to_string("days/day2/input-day2").expect("should contain input");

    let mut total_score = 0;

    input_str.trim().split("\n").for_each(|x| {
        let strs: Vec<&str> = x.trim_end().split(' ').collect();
        total_score += get_score_from_instr(strs[1]) + get_move_score(get_move(strs[1], strs[0]));
    });

    println!("Total Score: {:?}", total_score);
}
