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

// direction_filter:
// 0000 -> {left to right}{top to bottom}{right to left}{bottom to top}

fn build_max_vec(heights: &Vec<i32>, direction_filter: u32) -> Vec<u32> {
    let mut visibility = vec![];

    let mut max_height = -1;

    heights.into_iter().for_each(|height| {
        if height > &max_height {
            max_height = *height;
            visibility.push(0b1111 & direction_filter);
        } else {
            visibility.push(0b0000 & direction_filter);
        }
    });

    visibility
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

    let mut visibility_matrix = vec![];

    // left to right
    (&matrix).into_iter().for_each(|heights_row| {
        let visibility_row = build_max_vec(heights_row, 0b1000);
        visibility_matrix.push(visibility_row);
    });

    // right to left
    for (i, heights_row) in (&matrix).iter().enumerate() {
        let mut heights_row = heights_row.clone();
        heights_row.reverse();

        let visibility_row = build_max_vec(&heights_row, 0b0010);

        for (j, visibility) in visibility_row.iter().rev().enumerate() {
            visibility_matrix[i][j] = visibility_matrix[i][j] | visibility;
        }
    }

    let transposed_matrix = transpose(matrix);

    // top to bottom
    for (i, heights_row) in (&transposed_matrix).iter().enumerate() {
        let visibility_row = build_max_vec(&heights_row, 0b0010);

        for (j, visibility) in visibility_row.iter().enumerate() {
            visibility_matrix[j][i] = visibility_matrix[j][i] | visibility;
        }
    }

    // bottom to top
    for (i, heights_row) in (&transposed_matrix).iter().enumerate() {
        let mut heights_row = heights_row.clone();
        heights_row.reverse();

        let visibility_row = build_max_vec(&heights_row, 0b0010);

        for (j, visibility) in visibility_row.iter().rev().enumerate() {
            visibility_matrix[j][i] = visibility_matrix[j][i] | visibility;
        }
    }

    let mut cnt = 0;
    visibility_matrix.iter().for_each(|visibility_row| {
        visibility_row.iter().for_each(|visibility| {
            if visibility > &0 {
                cnt += 1;
            }
        });
    });

    println!("Count: {:?}", cnt);
}
