use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("./src/data").unwrap();

    let mut chars = input.chars();
    let mut chars2 = input.chars();

    let mut count_map: HashMap<char, usize> = HashMap::new();

    let mut count = 14;
    // let mut latest_char = ';';

    for _i in 0..14 {
        let c = chars.next().unwrap();
        *count_map.entry(c).or_insert(0) += 1;
    }
    println!("{:?}", count_map);

    loop {
        // println!("{:?}", count_map);
        // println!("{:?},{}", count_map, count);
        if count_map.values().all(|&x| x < 2) {
            println!("{:?},{}", count_map, count);
        }

        *count_map.entry(chars2.next().unwrap()).or_insert(0) -= 1;

        let c = chars.next().unwrap();
        // println!("{:?}", c);
        *count_map.entry(c).or_insert(0) += 1;

        count = count + 1;
    }

    // loop {
    // let first = chars.next().unwrap();
    // let second = chars.next().unwrap();
    // let third = chars.next().unwrap();
    // let forth = chars.next().unwrap();
    // println!("{},{},{},{}", first, second, third, forth);
    // if first != second
    //     && first != third
    //     && first != forth
    //     && second != third
    //     && second != forth
    //     && third != forth
    // {
    //     println!("{}", count + 4);
    //     return;
    // }
    // chars = input.chars();
    // chars.nth(count);

    // for c in chars.collect::<Vec<char>>()[count..count + 4].iter() {}
    // count = count + 1;
    // }
    // for (idx, c) in input.enumerate() {
    //     input[0]
    // }
}
