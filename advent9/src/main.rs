use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("src/data").unwrap();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    let mut ropes = [
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    let mut tail_path = HashMap::new();

    // for l in input.lines().skip(1989) {
    for l in input.lines() {
        let direction = l.split_whitespace().next().unwrap();
        let amount: isize = l.split_whitespace().nth(1).unwrap().parse().unwrap();

        for _i in 0..amount {
            if direction == "U" {
                ropes[0].1 += 1;
            }
            if direction == "D" {
                ropes[0].1 -= 1;
            }
            if direction == "R" {
                ropes[0].0 += 1;
            }
            if direction == "L" {
                ropes[0].0 -= 1;
            }

            let mut prev_rope = ropes[0];
            for (x, y) in ropes.iter_mut().skip(1) {
                let diff_x: i32 = prev_rope.0 - *x;
                let diff_y: i32 = prev_rope.1 - *y;

                if diff_y.abs() > 1 {
                    *y += diff_y.signum();
                    if diff_x != 0 {
                        *x += diff_x.signum();
                    }
                } else {
                    if diff_x.abs() > 1 {
                        *x += diff_x.signum();
                        if diff_y != 0 {
                            *y += diff_y.signum();
                        }
                    }
                }

                prev_rope = (*x, *y);
            }
            println!("{:?} ", ropes);

            // if direction == "U" {
            //     head_pos.1 += 1;
            // }
            // if direction == "D" {
            //     head_pos.1 -= 1;
            // }
            // if direction == "R" {
            //     head_pos.0 += 1;
            // }
            // if direction == "L" {
            //     head_pos.0 -= 1;
            // }

            // let diff_x: i32 = head_pos.0 - tail_pos.0;
            // let diff_y: i32 = head_pos.1 - tail_pos.1;

            println!("c :{:?}", direction);
            // println!("{:?} ,  {:?}", diff_x, diff_y);

            // if diff_y.abs() > 1 {
            //     tail_pos.1 += diff_y.signum();
            //     if diff_x != 0 {
            //         tail_pos.0 += diff_x.signum();
            //     }
            // }
            //
            // if diff_x.abs() > 1 {
            //     tail_pos.0 += diff_x.signum();
            //     if diff_y != 0 {
            //         tail_pos.1 += diff_y.signum();
            //     }
            // }

            // println!("{:?} : {:?}", head_pos, tail_pos)
            *tail_path.entry(ropes[9]).or_insert(1) += 1;
            println!("{}", tail_path.keys().len());
        }
    }

    println!("{}", tail_path.keys().len());

    // println!("Hello, world! {:#?}", test[0]);
}
