use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let file = fs::File::open("./src/data").expect("er");
    let reader = BufReader::new(file);
    let mut score = 0;
    for line in reader.lines() {
        if let Ok(data) = line {
            let opponent = data.chars().nth(0).unwrap();
            let you = data.chars().nth(2).unwrap();
            if opponent == 'A' {
                if you == 'X' {
                    score = score + 0;
                    score = score + 3;
                } else if you == 'Y' {
                    score = score + 3;
                    score = score + 1;
                } else if you == 'Z' {
                    score = score + 6;
                    score = score + 2;
                }
            } else if opponent == 'B' {
                if you == 'X' {
                    score = score + 0;
                    score = score + 1;
                } else if you == 'Y' {
                    score = score + 3;
                    score = score + 2;
                } else if you == 'Z' {
                    score = score + 6;
                    score = score + 3;
                }
            } else if opponent == 'C' {
                if you == 'X' {
                    score = score + 0;
                    score = score + 2;
                } else if you == 'Y' {
                    score = score + 3;
                    score = score + 3;
                } else if you == 'Z' {
                    score = score + 6;
                    score = score + 1;
                }
            }
        }
    }
    println!("{}", score)
}
