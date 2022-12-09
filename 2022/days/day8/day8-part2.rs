use std::fs;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .filter_map(|n| n.next())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn build_max_vec(heights: &Vec<i32>, debug: bool) -> Vec<u32> {
    let mut scenic_scores = vec![0];

    let len = heights.len();

    heights
        .into_iter()
        .enumerate()
        .skip(1)
        .for_each(|(i, curr_height)| {
            if i == len - 1 {
                scenic_scores.push(0);
                return;
            }

            let mut scenic_score = 0;

            for j in (0..i).rev() {
                if &heights[j] < curr_height {
                    if debug {
                        println!(
                            "Element: {:?} at idx: {} incrementing scenic score due to idx: {}",
                            curr_height, i, j
                        );
                    }
                    scenic_score += 1;
                } else if &heights[j] >= curr_height {
                    scenic_score += 1;
                    break;
                }
            }

            if debug {
                println!(
                    "Element: {:?} at idx: {} with scenic score: {}",
                    curr_height, i, scenic_score
                );
            }

            scenic_scores.push(scenic_score);
        });

    scenic_scores
}

fn main() {
    let input_str =
        fs::read_to_string("days/day8/input-day8").expect("should contain input");

    let matrix = input_str
        .trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|x| (x.to_digit(10).expect("invalid input") as i32))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut scenic_score_matrix = vec![];

    let row_len = matrix[1].len() - 1;
    let col_len = matrix.len() - 1;

    scenic_score_matrix.push(vec![0; col_len + 1]);

    // left to right
    (&matrix)
        .into_iter()
        .enumerate()
        .skip(1)
        .for_each(|(i, heights_row)| {
            let scenic_scores;
            if i == row_len {
                scenic_scores = vec![0; col_len + 1];
            } else {
                scenic_scores = build_max_vec(heights_row, false);
            }

            scenic_score_matrix.push(scenic_scores);
        });

    // right to left
    (&matrix)
        .into_iter()
        .enumerate()
        .skip(1)
        .for_each(|(i, heights_row)| {
            let mut heights_row = heights_row.clone();
            heights_row.reverse();

            let scenic_scores;
            if i == row_len {
                scenic_scores = vec![0; col_len + 1];
            } else {
                scenic_scores = build_max_vec(&heights_row, false);
            }

            for (j, scenic_score) in scenic_scores.iter().rev().enumerate() {
                scenic_score_matrix[i][j] = scenic_score_matrix[i][j] * scenic_score;
            }
        });

    let transposed_matrix = transpose(matrix);

    // top to bottom
    (&transposed_matrix)
        .into_iter()
        .enumerate()
        .skip(1)
        .for_each(|(i, heights_row)| {
            let scenic_scores;
            if i == row_len {
                scenic_scores = vec![0; col_len + 1];
            } else {
                scenic_scores = build_max_vec(&heights_row, false);
            }

            for (j, scenic_score) in scenic_scores.iter().enumerate() {
                scenic_score_matrix[j][i] = scenic_score_matrix[j][i] * scenic_score;
            }
        });

    // bottom to top
    (&transposed_matrix)
        .into_iter()
        .enumerate()
        .skip(1)
        .for_each(|(i, heights_row)| {
            let mut heights_row = heights_row.clone();
            heights_row.reverse();

            let scenic_scores;
            if i == row_len {
                scenic_scores = vec![0; col_len + 1];
            } else {
                scenic_scores = build_max_vec(&heights_row, false);
            }

            for (j, scenic_score) in scenic_scores.iter().rev().enumerate() {
                scenic_score_matrix[j][i] = scenic_score_matrix[j][i] * scenic_score;
            }
        });

    // println!("Scenic Scores Matrix: {:?}", scenic_score_matrix);

    let mut max_scenic_score = 0;
    scenic_score_matrix.iter().for_each(|scenic_scores| {
        scenic_scores.iter().for_each(|scenic_score| {
            if scenic_score > &max_scenic_score {
                max_scenic_score = *scenic_score;
            }
        });
    });

    println!("Max: {:?}", max_scenic_score);
}
