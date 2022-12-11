use std::fs;

#[derive(Debug, Clone, Copy, Default)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Operation {
    Noop,
    Addx(i32),
}

fn update_screen(screen: &mut Vec<Vec<char>>, sprite_start_pos: Coordinate, crt_pos: Coordinate) {
    if crt_pos.x <= sprite_start_pos.x + 2 && crt_pos.x >= sprite_start_pos.x {
        screen[crt_pos.y as usize][crt_pos.x as usize] = '#';
        // visualize_screen(&screen);
    }
}

fn visualize_screen(screen: &Vec<Vec<char>>) {
    screen.iter().for_each(|line| {
        line.iter().for_each(|pixel| {
            print!("{pixel}");
        });
        println!();
    });
    println!();
}

fn get_crt_pos(cycles_completed: i32) -> Coordinate {
    let y = (cycles_completed as f32 / 40f32).floor() as i32;
    Coordinate {
        y,
        x: cycles_completed - (y * 40),
    }
}

fn main() {
    let input_str =
        fs::read_to_string("days/day10/input-day10").expect("should contain input");

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

    let mut value: i32 = 1;

    let mut cycles_completed: i32 = 0;

    let mut screen = vec![vec!['.'; 40]; 6];
    let mut sprite_start_pos = Coordinate::default();

    operations.iter().enumerate().for_each(|(_i, operation)| {
        let mut crt_pos = get_crt_pos(cycles_completed);

        let offset = match operation {
            Operation::Noop => {
                cycles_completed += 1;
                update_screen(&mut screen, sprite_start_pos, crt_pos);
                0
            }
            Operation::Addx(val) => {
                cycles_completed += 1;
                update_screen(&mut screen, sprite_start_pos, crt_pos);

                crt_pos = get_crt_pos(cycles_completed);

                cycles_completed += 1;
                update_screen(&mut screen, sprite_start_pos, crt_pos);

                *val
            }
        };

        value += offset;

        sprite_start_pos.x = value - 1;
    });

    visualize_screen(&screen);
}
