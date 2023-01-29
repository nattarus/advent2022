use std::{fs::read_to_string, num};

fn main() {
    let input = read_to_string("src/data").unwrap();
    let mut sensors_baecon = Vec::new();
    for l in input.lines() {
        let mut l_iter = l.split_whitespace();
        let s_x: i32 = l_iter
            .nth(2)
            .unwrap()
            .trim_start_matches("x=")
            .trim_end_matches(",")
            .parse()
            .unwrap();
        let s_y: i32 = l_iter
            .next()
            .unwrap()
            .trim_start_matches("y=")
            .trim_end_matches(":")
            .parse()
            .unwrap();
        let b_x: i32 = l_iter
            .nth(4)
            .unwrap()
            .trim_start_matches("x=")
            .trim_end_matches(",")
            .parse()
            .unwrap();
        let b_y: i32 = l_iter
            .next()
            .unwrap()
            .trim_start_matches("y=")
            .parse()
            .unwrap();
        sensors_baecon.push(((s_x, s_y), (b_x, b_y)));
    }
    for i in 0..4000000 {
        // println!("{}", i);
        let mut range_vec = Vec::new();
        let mut baecon_on_y = Vec::new();
        let y = i;

        for ((s_x, s_y), (b_x, b_y)) in sensors_baecon.iter() {
            let diff_x = i32::abs(s_x - b_x);
            let diff_y = i32::abs(s_y - b_y);
            let diff = diff_x + diff_y;
            let diff_line = i32::abs(s_y - y);
            if diff_line < diff {
                let range = diff - diff_line;
                let y_range = (s_x - range, s_x + range);
                range_vec.push(y_range);
                baecon_on_y.push((b_x, b_y));

                // println!("{:?}", l);
                // println!("{:?} {} {}", diff_x, diff_y, diff);
                // println!("{:?}", y_range);
            }
        }

        // let after_merge = Vec::new();
        let mut overlap = true;
        range_vec.sort_by(|a, b| a.0.cmp(&b.0));
        // let mut res = range_vec.clone();

        while overlap {
            overlap = false;
            for i in 0..range_vec.len() - 1 {
                if range_vec[i].1 >= range_vec[i + 1].0 {
                    range_vec[i].1 = range_vec[i].1.max(range_vec[i + 1].1);
                    range_vec.remove(i + 1);
                    overlap = true;
                    break;
                }
            }
            // println!("{:?}", range_vec);
        }
        if range_vec.len() > 1 {
            println!("{:?} {}", range_vec, y);
            println!(
                "{}",
                ((range_vec[0].1 + 1) as i64 * 4000000 as i64) + y as i64
            )
        }

        // let sum: i32 = range_vec.iter().map(|a| a.1 - a.0 + 1).sum();
        // baecon_on_y.sort_by(|a, b| a.0.cmp(&b.0));
        // println!(
        //     "{:?}",
        //     baecon_on_y
        //         .iter()
        //         .filter(|a| a.1 == y)
        //         .collect::<Vec<&(i32, i32)>>()
        // );
        // println!("{}", sum);
    }
}
