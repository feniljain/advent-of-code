use std::fs;

fn main() {
    let input_str = fs::read_to_string("days/day1/input-day1").expect("should contain input");

    let mut a: Vec<u32> = input_str
        .split("\n\n")
        .map(|x| {
            x.trim_end()
                .split('\n')
                .map(|x_str| x_str.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    a.sort();
    a.reverse();

    println!("a: {:?}", a);

    println!("a: {:?}", a[0] + a[1] + a[2]);
}
