use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let file = fs::File::open("./src/data").expect("er");
    let reader = BufReader::new(file);
    let mut char_list = ".abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;
    let mut line_num = 0;
    let mut first = "".to_string();
    let mut second = "".to_string();
    let mut third;
    let mut char_match = false;
    for line in reader.lines() {
        if let Ok(data) = line {
            if line_num == 0 {
                first = data;
                line_num = line_num + 1
            } else if line_num == 1 {
                second = data;
                line_num = line_num + 1
            } else if line_num == 2 {
                third = data;
                char_match = false;

                line_num = 0;
                for ch1 in first.chars() {
                    for ch2 in second.chars() {
                        if ch1 == ch2 && !char_match {
                            println!("ma");
                            for ch3 in third.chars() {
                                if ch1 == ch3 && !char_match {
                                    char_match = true;
                                    let pos = char_list.chars().position(|c| c == ch1).unwrap();
                                    sum = sum + pos;
                                }
                            }
                        }
                    }
                }
            }
            // let first = &data[0..data.len() / 2];
            // let second = &data[data.len() / 2..];
            // for ch1 in first.chars() {
            //     for ch2 in second.chars() {
            //         if ch1 == ch2 && !char_match {
            //             char_match = true;
            //             let pos = char_list.chars().position(|c| c == ch1).unwrap();
            //             sum = sum + pos;
            //
            //             println!("{}:{}", ch1, pos)
            //             // println!("{}:{}:{}:{}", first, second, ch1, pos);
            //         }
            //     }
            // }
            // println!("{},{}", first.len(), second.len());
        }
    }
    // println!("{}", 'b' as u32 - 96)
    println!("{}", sum)
}
