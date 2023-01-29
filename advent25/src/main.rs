use std::{collections::HashMap, fs::read_to_string};

const SNAFU: [(char, i64); 5] = [('2', 2), ('1', 1), ('0', 0), ('-', -1), ('=', -2)];
fn main() {
    let input = read_to_string("src/data").unwrap();
    let mut map = HashMap::new();
    let mut all_sum = 0;
    let mut all = vec![0; 30];
    for l in input.lines() {
        let mut val = 0;
        for (i, c) in l.chars().rev().enumerate() {
            let v = SNAFU.iter().find(|s| c == s.0).unwrap();
            *map.entry(i).or_insert(0) += v.1;

            val += (5_i64.pow(i as u32)) as i64 * v.1;
            all[i] += v.1;
        }
        dbg!(l, val);
        all_sum += val
    }

    for i in 0..all.len() - 1 {
        let v = all[i] % 5;

        println!("{:?} ", all);
        if v > 2 && v < 5 {
            all[i + 1] += all[i] / 5 + 1;
            all[i] = v - 5
        } else if v < -2 && v > -5 {
            all[i + 1] += all[i] / 5 - 1;
            all[i] = 5 + v
        } else {
            all[i + 1] += all[i] / 5;
            all[i] = v
        }
        // println!("{:?} ", all.len());
    }

    let mut res = String::new();

    for i in all {
        let val = SNAFU.iter().find(|s| s.1 == i).unwrap().0;
        res.push(val);
    }
    println!("{:?}", res.chars().rev().collect::<String>());
    // dbg!(all);
    // let keys = map.keys().max().unwrap();
    // for i in 0..*keys {
    //     let count = map.get_mut(&i).unwrap();
    //     let next = map.get_mut(&(i + 1)).unwrap();
    //     if *count > 2 || *count < -2 {
    //         *map.entry(i + 1).or_insert(0) += *count / 5;
    //         *count = *count % 5;
    //         println!("{:?} ", map);
    //     }
    // }
    // dbg!(all_sum);
    // for i in 0..10 {
    //     let j = 10 - i;
    //     let t = 5_i64.pow(j);
    //     if all_sum / t == 2 {
    //         all_sum = all_sum % t
    //     }
    //     if all_sum / t == 1 {
    //         all_sum = all_sum % t
    //     }
    //     println!("{}", i)
    // }
}
