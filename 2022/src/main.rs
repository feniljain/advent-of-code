use std::{collections::HashMap, fs};

#[derive(Debug)]
struct DirInfo<'a> {
    dir_size: u64,
    outputs: Vec<Output<'a>>,
}

#[derive(Debug, Copy, Clone)]
enum Output<'a> {
    DIR { dir_name: &'a str },
    FILE { size: u64 },
}

#[derive(Debug)]
enum Command<'a> {
    CD { dir_name: &'a str },
    LS(Vec<Output<'a>>),
}

#[derive(Debug)]
struct Parser<'a> {
    lines: Vec<&'a str>,
    idx: usize,
}

impl<'a> Parser<'a> { fn new(input: Vec<&'a str>) -> Self {
        Parser {
            lines: input,
            idx: 0,
        }
    }

    fn parse_ls_cmd_output(line: &'a str) -> Output<'a> {
        let split_output: Vec<&str> = line.split_ascii_whitespace().collect();
        match split_output[0] {
            "dir" => Output::DIR {
                dir_name: split_output[1],
            },
            file_size_str => Output::FILE {
                size: file_size_str
                    .parse::<u64>()
                    .expect("incorrect file size param"),
            },
        }
    }

    fn parse_command(line: &'a str) -> Command {
        let split_command: Vec<&str> = line.split_ascii_whitespace().collect();
        let _dollar_sign = split_command[0];
        let command_name = split_command[1];

        match command_name {
            "cd" => Command::CD {
                dir_name: split_command[2],
            },
            "ls" => Command::LS(vec![]),
            _ => panic!("wrong command"),
        }
    }

    fn parse(&'a mut self) -> Vec<Command<'a>> {
        let mut commands = vec![];

        let mut output_lines: Vec<Output> = vec![];

        while let Some(line) = self.advance() {
            let mut chars = line.chars();
            if chars
                .next()
                .and_then(|ch| (ch == '$').then_some(ch))
                .is_some()
            {
                if output_lines.len() > 0 {
                    commands.push(Command::LS(output_lines));
                    output_lines = vec![];
                }

                let cmd = Parser::parse_command(line);
                if let Command::CD { dir_name: _ } = cmd {
                    commands.push(cmd);
                }
            } else {
                output_lines.push(Parser::parse_ls_cmd_output(line));
            }
        }

        if output_lines.len() > 0 {
            commands.push(Command::LS(output_lines));
        }

        commands
    }

    fn advance(&mut self) -> Option<&'a str> {
        let ele: Option<&str> = self.lines.get(self.idx).copied();
        self.idx += 1;
        ele
    }
}

struct Stack {
    data: Vec<String>,
    size: usize,
}

impl Stack {
    fn new() -> Self {
        Self {
            size: 0,
            data: vec![],
        }
    }

    fn push(&mut self, name: String) {
        self.data.push(name);
        self.size += 1;
    }

    fn pop(&mut self) -> String {
        self.size -= 1;
        self.data.remove(self.size as usize)
    }

    // fn peek(&mut self) -> Option<&'a str> {
    //     self.data.last().copied()
    // }

    fn construct_path(&self) -> String {
        if self.size > 1 {
            "/".to_string() + &self.data[1..].join("/")
        } else {
            "-".to_string()
        }
    }
}

fn calculate_dir_size(
    outputs: &Vec<Output>,
    dir_infos: &HashMap<String, DirInfo>,
    curr_dir_name: String,
) -> u64 {
    let mut dir_size = 0;

    for output in outputs {
        match output {
            Output::FILE { size } => {
                dir_size += size;
            }
            Output::DIR { dir_name } => {
                let path = if curr_dir_name == "-" {
                    "/".to_string() + dir_name
                } else {
                    curr_dir_name.clone() + "/" + dir_name
                };

                let size = dir_infos.get(&path)
                    .expect(
                        &format!(
                            "inconsistent state: expected dir_info for dir_name {:?} to be present after .. of {:?}",
                            dir_name,
                            curr_dir_name
                        )
                    ).dir_size;
                dir_size += size;
            }
        }
    }

    dir_size
}

fn main() {
    let input_str = fs::read_to_string("days/day7/input-day7").expect("should contain input");
    let lines: Vec<&str> = input_str.trim().split("\n").collect();

    let mut parser = Parser::new(lines);
    let cmds = parser.parse();

    // Here we make an assumption that no files/dirs will have same
    // name
    //
    // Fk this assumption, it costed my so many debugging hours
    let mut dir_infos: HashMap<String, DirInfo> = HashMap::new();

    let mut stack = Stack::new();

    for cmd in cmds {
        match cmd {
            Command::CD { dir_name } => {
                if dir_name == ".." {
                    let path = stack.construct_path();

                    match dir_infos.get(&path) {
                        Some(dir_info) => {
                            if dir_info.dir_size == u64::MAX {
                                let dir_size =
                                    calculate_dir_size(&dir_info.outputs, &dir_infos, path.clone());
                                // update it in hashmap
                                dir_infos.insert(
                                    path,
                                    DirInfo {
                                        dir_size,
                                        outputs: dir_info.outputs.clone(),
                                    },
                                );
                            }
                        }
                        None => {
                            panic!(
                                "invalid state: dir_infos should contain dir_name: {:?}",
                                dir_name
                            );
                        }
                    }
                    stack.pop();
                } else {
                    if dir_name == "/" {
                        stack.push(dir_name.to_string());
                    } else {
                        stack.push(dir_name.to_string());
                    }
                }
            }
            Command::LS(outputs) => {
                // This also covers the case where there are no files in a dir
                let only_files = (&outputs).into_iter().all(|x| {
                    if let Output::DIR { dir_name: _ } = x {
                        return false;
                    }

                    return true;
                });

                let path = stack.construct_path();

                if only_files {
                    let mut dir_size = 0;
                    for output in &outputs {
                        match output {
                            Output::FILE { size } => {
                                dir_size += size;
                            }
                            _ => (),
                        }
                    }
                    let dir_size = calculate_dir_size(&outputs, &dir_infos, path.clone());
                    dir_infos.insert(path, DirInfo { dir_size, outputs });
                } else {
                    dir_infos.insert(
                        path,
                        DirInfo {
                            dir_size: u64::MAX,
                            outputs,
                        },
                    );
                }
            }
        }
    }

    while stack.size > 0 {
        let path = stack.construct_path();
        let _name = stack.pop();

        let dir_info = dir_infos
            .get(&path)
            .expect("inconsistent state: expected dir to be present");

        let dir_size = calculate_dir_size(&dir_info.outputs, &dir_infos, path.clone());
        dir_infos.insert(
            path,
            DirInfo {
                dir_size,
                outputs: dir_info.outputs.clone(),
            },
        );
    }

    let root_dir_info = dir_infos
        .get("-")
        .expect("inconsistent state: expected dir to be present");

    let unused_space = 70000000 - root_dir_info.dir_size;

    let offset_from_goal = 30000000 - unused_space;

    println!("{}", offset_from_goal);

    let mut min: u64 = u64::MAX;

    for (_, dir_info) in dir_infos {
        if dir_info.dir_size >= offset_from_goal {
            if dir_info.dir_size < min {
                min = dir_info.dir_size;
            }
        }
    }

    println!("Min: {:?}", min);
}

// fn print_map(dir_infos: &HashMap<String, DirInfo>) {
//     for (name, dir_info) in dir_infos {
//         println!("name: {} : dir_size: {}", name, dir_info.dir_size);
//     }
// }
