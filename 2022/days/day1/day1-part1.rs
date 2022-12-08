use std::fs;

fn main() {
    let input_str = fs::read_to_string("input").expect("should contain input");
    let a: Option<u32> = input_str
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .filter_map(|x_str| {
                    if x_str == "" {
                        return None;
                    }

                    Some(x_str.parse::<u32>().unwrap())
                })
                .sum::<u32>()
        })
        .max();

    println!("a: {:?}", a);
}
