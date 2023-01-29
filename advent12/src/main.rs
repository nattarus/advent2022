use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("./src/data").unwrap();
    let mut matrix = Vec::new();
    // let mut map = HashMap::new();
    let mut current_pos = (0, 0);
    let mut target_pos = (0, 0);
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    for (row, l) in input.lines().enumerate() {
        let mut rows = Vec::new();
        for (col, c) in l.chars().enumerate() {
            if c.is_uppercase() {
                if c == 'S' {
                    current_pos = (col, row);
                    // map.insert((col, row), 'a');
                    rows.push('a');
                }
                if c == 'E' {
                    target_pos = (col, row);
                    rows.push('z');
                }
            } else {
                // let val: u32 = (c as u32) - 95;
                // map.insert((col, row), c);
                rows.push(c)
            }
        }
        matrix.push(rows);
    }
    let mut matrix_val = vec![vec![usize::MAX; width]; height];

    matrix_val[target_pos.1][target_pos.0] = 0;

    // recur(
    //     &target_pos,
    //     &current_pos,
    //     &matrix,
    //     &mut matrix_val,
    //     width,
    //     height,
    // );
    // println!("{:?} {}", 'f' as usize, 'e' as usize);
    // println!("{:?}", matrix_val[30])
    // println!("{:?}", matrix_val[target_pos.1][target_pos.0]);

    // let mut mins = Vec::new();
    // for (l_c, l_v) in matrix.iter().zip(matrix_val.iter()) {
    //     for (c, v) in l_c.iter().zip(l_v.iter()) {
    //         if *c == 'a' {
    //             mins.push(v);
    //         }
    //     }
    // }
    // println!("{:?}", mins.iter().min());

    // for l in matrix {
    //     println!(
    // "{:?}",
    // l.iter()
    //     .map(|m| {
    //         if *m == usize::MAX {
    //             return String::from("0");
    //         }
    //         // return String::from("x");
    //         return m.to_string();
    //     })
    //     .collect::<Vec<String>>()
    //     .join(" ")
    //         l.iter()
    //             .map(|m| m.to_string())
    //             .collect::<Vec<String>>()
    //             .join("")
    //     );
    // }
}

fn walk(
    from: (usize, usize),
    to: (usize, usize),
    map: &Vec<Vec<char>>,
    val_map: &mut Vec<Vec<usize>>,
) -> bool {
    let from_char = map[from.1][from.0];
    let to_char = map[to.1][to.0];
    let diff = to_char as i32 - from_char as i32;
    let can_walk = diff > -2;

    let from_val = val_map[from.1][from.0];
    let to_val = val_map[to.1][to.0];
    let is_less_step = from_val + 1 < to_val;
    // if from_char == 'f' && to_char == 'g' {
    //     println!("walk {} {} {:?}", from_char, to_char, can_walk);
    //     println!("walk {} {} {:?}", from_val, to_val, is_less_step);
    // }
    if can_walk && is_less_step {
        val_map[to.1][to.0] = from_val + 1;
        return true;
    }
    return false;
}

fn recur(
    current_pos: &(usize, usize),
    target_pos: &(usize, usize),
    map: &Vec<Vec<char>>,
    val_map: &mut Vec<Vec<usize>>,
    width: usize,
    height: usize,
) {
    // for l in map {
    //     println!(
    //         "{:?}",
    //         l.iter()
    //             .map(|m| m.to_string())
    //             .collect::<Vec<String>>()
    //             .join("")
    //     );
    // }
    // println!("recure");
    if current_pos.0 != 0 {
        let to = (current_pos.0 - 1, current_pos.1);
        if walk(*current_pos, to, &map, val_map) {
            recur(&to, target_pos, map, val_map, width, height);
        }
    }
    if current_pos.0 < width - 1 {
        let to = (current_pos.0 + 1, current_pos.1);
        if walk(*current_pos, to, &map, val_map) {
            recur(&to, target_pos, map, val_map, width, height);
        }
    }
    if current_pos.1 != 0 {
        let to = (current_pos.0, current_pos.1 - 1);
        if walk(*current_pos, to, &map, val_map) {
            recur(&to, target_pos, map, val_map, width, height);
        }
    }
    if current_pos.1 < height - 1 {
        let to = (current_pos.0, current_pos.1 + 1);
        // println!("{:?} {}", to, height);
        if walk(*current_pos, to, &map, val_map) {
            recur(&to, target_pos, map, val_map, width, height);
        }
    }
}
