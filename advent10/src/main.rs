use std::fs;

fn main() {
    let input = fs::read_to_string("./src/data").unwrap();

    let mut reg_x: isize = 1;
    // let mut prev_reg_x: isize = 1;

    // let mut sum = 0;
    let mut cycle = 0;

    let mut rows = Vec::new();

    for l in input.lines() {
        let op = l.split_whitespace().next().unwrap();
        cycle += 1;

        let splite_pos = reg_x % 40;
        if splite_pos <= cycle % 40 && cycle % 40 <= splite_pos + 2 {
            rows.push("#");
        } else {
            rows.push(".");
        }

        if cycle % 40 == 0 {
            println!("{:?}", rows.join(""));
            rows = Vec::new();
        }

        if op == "addx" {
            let val: isize = l.split_whitespace().nth(1).unwrap().parse().unwrap();
            cycle += 1;

            let splite_pos = reg_x % 40;
            if splite_pos <= cycle % 40 && cycle % 40 <= splite_pos + 2 {
                rows.push("#");
            } else {
                rows.push(".");
            }

            if cycle % 40 == 0 {
                println!("{:?}", rows.join(""));
                rows = Vec::new();
            }

            reg_x += val;
        }

        // prev_reg_x = reg_x;
    }
}
