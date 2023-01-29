use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let file = fs::File::open("./src/data").expect("er");
    let reader = BufReader::new(file);
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut max = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    let mut ele = 0;
    for line in reader.lines() {
        if let Ok(data) = line {
            if data == "" {
                if ele > max3 {
                    max2 = max3;
                    max3 = ele;
                } else if ele > max2 {
                    max = max2;
                    max2 = ele
                } else if ele > max {
                    max = ele
                }

                x.push(ele);
                ele = 0;
                y = Vec::new();
            } else {
                let cal = data.parse::<i32>().unwrap();
                ele = ele + cal;
                y.push(data.parse::<i32>().unwrap());
            }
        }
    }
    x.sort();
    x.reverse();
    let result: &i32 = &x[0..3].iter().sum();
    println!("{:?}", result);
    // println!("{:?}", max + max2 + max3);
}
