use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone)]
struct M<'a> {
    name: &'a str,
    val: Option<i64>,
    first: Option<&'a str>,
    op: Option<&'a str>,
    last: Option<&'a str>,
}

fn main() {
    let input = read_to_string("src/data").unwrap();
    let mut data = Vec::new();
    let mut map = HashMap::new();
    // let mut humn: M;
    for l in input.lines() {
        let mut iter = l.split_whitespace();
        let name = iter.next().unwrap().trim_end_matches(":");
        let first = iter.next().unwrap();
        if let Ok(val) = first.parse::<i64>() {
            // let first_val: Option<&i64> = None;
            // data.push(M {
            //     name: name.to_string(),
            //     val: Some(val),
            //     first: None,
            //     op: None,
            //     last: None,
            // });

            map.insert(name, val);
        } else {
            let op = iter.next().unwrap();
            let last = iter.next().unwrap();
            data.push(M {
                name,
                val: None,
                first: Some(first),
                op: Some(op),
                last: Some(last),
            })
        }
        // if name == "humn" {
        //     let op = iter.next().unwrap();
        //     let last = iter.next().unwrap();
        //     data.push(M {
        //         name,
        //         val: None,
        //         first: Some(first),
        //         op: Some(op.to_string()),
        //         last: Some(last),
        //     })
        // }
    }
    map.remove("humn");

    let mut left_data = data.clone();
    let mut working = true;
    while working {
        let mut dat = Vec::new();
        working = false;
        for d in left_data {
            if d.first.unwrap() == "humn" || d.last.unwrap() == "humn" {
                dat.push(d);
                continue;
            }
            let first_val = map.get(&d.first.unwrap());
            let second_val = map.get(&d.last.unwrap());
            if first_val.is_some() && second_val.is_some() {
                let first_val = first_val.unwrap();
                let second_val = second_val.unwrap();
                let op = d.op.unwrap();
                let mut res = 0;
                if op == "*" {
                    res = first_val * second_val;
                }
                if op == "-" {
                    res = first_val - second_val;
                }
                if op == "+" {
                    res = first_val + second_val;
                }
                if op == "/" {
                    res = first_val / second_val;
                }
                map.insert(d.name, res);
                working = true;
            } else {
                dat.push(d);
            }
        }
        left_data = dat;
    }

    // println!("{}", map.get("root").unwrap());
    // println!("{:?}", left_data);
    while map.get("humn").is_none() {
        // println!("{} {}", map.values().len(), input.lines().count());
        // println!("{:?}", map);
        // println!("{:?}", left_data);
        for d in left_data.iter() {
            if d.name == "root" {
                let first_val = map.get(&d.first.unwrap());
                if let Some(val) = first_val {
                    // println!("{}", val);
                    map.insert(d.last.unwrap(), *val);
                }
                let second_val = map.get(&d.last.unwrap());
                if let Some(val) = second_val {
                    // println!("{}", val);
                    map.insert(d.first.unwrap(), *val);
                }
                continue;
            }

            let result = map.get(d.name);

            let op = d.op.unwrap();
            if let Some(res) = result {
                let first_val = map.get(&d.first.unwrap());
                if let Some(val) = first_val {
                    let mut other = 0;
                    if op == "*" {
                        // res = val * other;
                        other = res / val;
                    }
                    if op == "-" {
                        // res = val - other;
                        other = val - res;
                    }
                    if op == "+" {
                        // res = val + other;
                        other = res - val;
                    }
                    if op == "/" {
                        // res = val / other;
                        other = val / res;
                    }
                    map.insert(d.last.unwrap(), other);
                }
            }

            let result = map.get(d.name);

            if let Some(res) = result {
                let last_val = map.get(&d.last.unwrap());
                if let Some(val) = last_val {
                    let mut other = 0;
                    if op == "*" {
                        // res = other * val;
                        other = res / val;
                    }
                    if op == "-" {
                        // res = other - val;
                        other = val + res;
                    }
                    if op == "+" {
                        // res = other + val;
                        other = res - val;
                    }
                    if op == "/" {
                        // res = other / val;
                        // println!("{},{}", d.name, val);
                        other = val * res;
                    }
                    map.insert(d.first.unwrap(), other);
                }
            }
        }
    }
    println!("{}", map.get("humn").unwrap())

    // println!("{:#?}", left_data);
    // println!("{}", map.get("root").unwrap());

    // for i in 0..10000000 {
    //     let mut map = HashMap::new();
    //     // println!("{}", i);
    //     map.insert("humn", i);
    //     while map.get("humn").is_none() || map.values().len() != input.lines().count() {
    //         for l in input.lines() {
    //             let mut iter = l.split_whitespace();
    //             let name = iter.next().unwrap().trim_end_matches(":");
    //
    //             if name == "humn" {
    //                 continue;
    //             }
    //
    //             let first = iter.next().unwrap();
    //             let mut first_val: Option<&i64> = None;
    //             if let Ok(val) = first.parse::<i64>() {
    //                 map.insert(name, val);
    //             } else {
    //                 first_val = map.get(first);
    //                 let op = iter.next().unwrap();
    //                 let second_val = map.get(iter.next().unwrap());
    //                 if first_val.is_some() && second_val.is_some() {
    //                     let mut res = 0;
    //                     let first_val = first_val.unwrap();
    //                     let second_val = second_val.unwrap();
    //
    //                     if name == "root" {
    //                         if first_val == second_val {
    //                             println!("get {}", map.get("humn").unwrap());
    //                             return;
    //                         }
    //                         map.insert(name, 1);
    //                         break;
    //                     }
    //
    //                     if op == "*" {
    //                         res = first_val * second_val;
    //                     }
    //                     if op == "-" {
    //                         res = first_val - second_val;
    //                     }
    //                     if op == "+" {
    //                         res = first_val + second_val;
    //                     }
    //                     if op == "/" {
    //                         res = first_val / second_val;
    //                     }
    //                     map.insert(name, res);
    //                 }
    //             }
    //         }
    //     }
    //     // println!("{}", map.get("root").unwrap());
    // }
}
