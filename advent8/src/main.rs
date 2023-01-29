use std::{cmp, collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("src/data").unwrap();

    // let mut map = HashMap::new();
    let mut rows = Vec::new();
    let width = input.lines().next().unwrap().len();
    let mut cols = Vec::new();
    for _i in 0..width {
        cols.push(Vec::new());
    }

    for (row_idx, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (col_idx, char) in line.chars().enumerate() {
            let height: isize = char.to_digit(10).unwrap().try_into().unwrap();
            // println!("{}", height);
            // map.insert((row, col), height);
            row.push(height);
            cols[col_idx].push(height);
        }
        rows.push(row)
    }

    let mut count = 0;
    let mut max = 0;

    // println!("{:?}", cols[0][1..2]);

    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, char) in line.chars().enumerate() {
            let height: isize = char.to_digit(10).unwrap().try_into().unwrap();

            let left = rows[row_idx][0..col_idx]
                .iter()
                .rev()
                .position(|x| x >= &height)
                .map(|x| x + 1)
                .unwrap_or(col_idx);

            let right = rows[row_idx][col_idx + 1..width]
                .iter()
                .position(|x| x >= &height)
                .map(|x| x + 1)
                .unwrap_or(width - col_idx - 1);

            let top = cols[col_idx][0..row_idx]
                .iter()
                .rev()
                .position(|x| x >= &height)
                .map(|x| x + 1)
                .unwrap_or(row_idx);
            let bottom = cols[col_idx][row_idx + 1..rows.len()]
                .iter()
                .position(|x| x >= &height)
                .map(|x| x + 1)
                .unwrap_or(rows.len() - row_idx - 1);

            // if [left, right, top, bottom].iter().all(|x| x.is_some()) {
            let score = left * top * right * bottom;
            max = cmp::max(score, max);
            count += 1;
            // }

            // if 0 == *[left, top, bottom, right].iter().min().unwrap() {
            println!(
                "{},{}: {} : {} {} {} {}",
                row_idx, col_idx, max, right, left, bottom, top
            );
            // }
        }
    }
    println!("Hello, world!, {},{}", count, max);

    // println!("{:?}", rows);

    // for ((row, col), height) in map.iter() {
    //     let left = map
    //         .iter()
    //         .filter(|&((r, c), _h)| (r == row) && c < col)
    //         .map(|(&(_r, _c), &h)| h)
    //         .max()
    //         .unwrap_or(-1);
    //     let right = map
    //         .iter()
    //         .filter(|&((r, c), _h)| (r == row) && c > col)
    //         .map(|(&(_r, _c), &h)| h)
    //         .max()
    //         .unwrap_or(-1);
    //     let top = map
    //         .iter()
    //         .filter(|&((r, c), _h)| c == col && r > row)
    //         .map(|(&(_r, _c), &h)| h)
    //         .max()
    //         .unwrap_or(-1);
    //     let bottom = map
    //         .iter()
    //         .filter(|&((r, c), _h)| c == col && r < row)
    //         .map(|(&(_r, _c), &h)| h)
    //         .max()
    //         .unwrap_or(-1);
    //
    //     if height > [left, top, bottom, right].iter().min().unwrap() || *row == 0 || *col == 0 {
    //         // if *col == 0 {
    //         //     println!(
    //         //         "{},{}: {} : {} {} {} {}",
    //         //         row, col, height, right, left, bottom, top
    //         //     );
    //         // }
    //         count += 1;
    //     }
    //
    //     // println!("{},{} : {} {}", row, col, right, left)
    // }
}
