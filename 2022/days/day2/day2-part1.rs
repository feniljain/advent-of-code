use std::fs;

fn get_game_score(opponent_choice: &str, your_choice: &str) -> usize {
    match (opponent_choice, your_choice) {
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
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

fn main() {
    let input_str = fs::read_to_string("days/day2/example-input-day2").expect("should contain input");

    let mut total_score = 0;

    // let a: Vec<Vec<&str>> =
    input_str.trim().split("\n").for_each(|x| {
        let strs: Vec<&str> = x.trim_end().split(' ').collect();
        total_score += get_game_score(strs[0], strs[1]) + get_move_score(strs[1]);
    });

    println!("Total Score: {:?}", total_score);
}
