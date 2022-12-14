use std::{collections::VecDeque, fs};

#[derive(Clone, Default, Debug)]
struct NodeInfo {
    ch: char,
    adj_nodes_info: Vec<AdjNodeInfo>,
}

#[derive(Clone, Copy, Debug)]
struct AdjNodeInfo {
    ch: char,
    graph_idx: usize,
}

fn get_level(ch: char) -> i32 {
    if ch == 'S' {
        'a' as i32
    } else if ch == 'E' {
        'z' as i32
    } else {
        ch as i32
    }
}

fn get_adj_nodes(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<AdjNodeInfo> {
    let mut adj_nodes = vec![];

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let curr_level = get_level(matrix[i][j]);

    let condition = |i: usize, j: usize, matrix: &Vec<Vec<char>>| -> bool {
        (curr_level - get_level(matrix[i][j])) >= -1
    };

    // upper element
    if i > 0 && condition(i - 1, j, matrix) {
        let graph_idx = calculate_graph_idx(i - 1, j, n_cols);
        adj_nodes.push(AdjNodeInfo {
            ch: matrix[i - 1][j],
            graph_idx,
        });
    }

    // lower element
    if i < n_rows - 1 && condition(i + 1, j, matrix) {
        let graph_idx = calculate_graph_idx(i + 1, j, n_cols);
        adj_nodes.push(AdjNodeInfo {
            ch: matrix[i + 1][j],
            graph_idx,
        });
    }

    // left element
    if j > 0 && condition(i, j - 1, matrix) {
        let graph_idx = calculate_graph_idx(i, j - 1, n_cols);
        adj_nodes.push(AdjNodeInfo {
            ch: matrix[i][j - 1],
            graph_idx,
        });
    }

    // right element
    if j < n_cols - 1 && condition(i, j + 1, matrix) {
        let graph_idx = calculate_graph_idx(i, j + 1, n_cols);
        adj_nodes.push(AdjNodeInfo {
            ch: matrix[i][j + 1],
            graph_idx,
        });
    }

    adj_nodes
}

fn calculate_graph_idx(i: usize, j: usize, cols_len: usize) -> usize {
    (i * (cols_len)) + j
}

fn calculate_matrix_idx(graph_idx: usize, cols_len: usize) -> (usize, usize) {
    let i = graph_idx / cols_len;
    let j = graph_idx - (i * cols_len);

    (i, j)
}

fn plot_visited(visited: Vec<bool>, rows: usize, cols: usize) {
    let mut cnt = 0;
    for _i in 0..rows {
        for _j in 0..cols {
            if visited[cnt] {
                print!("X");
            } else {
                print!(".");
            }
            cnt += 1;
        }
        println!();
    }
}

fn main() {
    let input_str =
        fs::read_to_string("days/day12/example-input-day12").expect("should contain input");
    // fs::read_to_string("days/day12/input-day12").expect("should contain input");

    let lines: Vec<&str> = input_str.split("\n").collect();

    let mut matrix = vec![vec!['a'; lines[0].len()]; lines.len() - 1];

    lines.iter().enumerate().for_each(|(i, line)| {
        line.trim().chars().enumerate().for_each(|(j, ch)| {
            matrix[i][j] = ch;
        });
    });

    let mut graph: Vec<NodeInfo> = vec![Default::default(); lines[0].len() * (lines.len() - 1)];
    let mut visited: Vec<bool> = vec![false; lines[0].len() * (lines.len() - 1)];
    let mut dist: Vec<usize> = vec![0; lines[0].len() * (lines.len() - 1)];

    let mut start_node_idx = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let adj_nodes = get_adj_nodes(&matrix, i, j);

            let idx = calculate_graph_idx(i, j, matrix[0].len());
            graph[idx] = NodeInfo {
                ch: matrix[i][j],
                adj_nodes_info: adj_nodes,
            };

            if matrix[i][j] == 'S' {
                start_node_idx = idx;
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(start_node_idx);
    visited[start_node_idx] = true;
    dist[start_node_idx] = 0;

    let mut end_goal_dist = 0;

    while let Some(node_idx) = queue.pop_front() {
        for adj_node in &graph[node_idx].adj_nodes_info {
            if !visited[adj_node.graph_idx] {
                visited[adj_node.graph_idx] = true;
                dist[adj_node.graph_idx] = dist[node_idx] + 1;
                queue.push_back(adj_node.graph_idx);

                if adj_node.ch == 'E' {
                    end_goal_dist = dist[adj_node.graph_idx];
                    break;
                }
            }
        }
    }

    println!("End Goal Distance: {end_goal_dist:?}");
    // plot_visited(visited, matrix.len(), matrix[0].len());
}
