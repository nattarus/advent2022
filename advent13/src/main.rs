use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/data").unwrap();
    // let pairs = input.split("\n\n");
    // let mut orders = 0;
    //
    // for (idx, i) in pairs.enumerate() {
    //     let left = i.lines().next().unwrap();
    //     let right = i.lines().nth(1).unwrap();
    //
    //     println!("{:?}", left);
    //     println!("{:?}", right);
    //
    //     if compare(&left, &right) == 0 {
    //         orders += idx + 1;
    //     }
    //     // println!("{:#?}...", split_list(left));
    //     println!("{:?}...\n", orders)
    // }
    let sig1 = "[[2]]";
    let sig2 = "[[6]]";
    let signals: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();
    let mut sorted = Vec::new();
    sorted.push(sig1);
    sorted.push(sig2);
    for s in signals {
        for (idx, f) in sorted.iter().enumerate() {
            let com = compare(s, f);
            if com == 1 {
                sorted.insert(idx, s);
                break;
            }
        }
    }
    println!("{:#?}", sorted);
    let first_idx = sorted.iter().position(|x| *x == sig1).unwrap() + 1;
    let second_idx = sorted.iter().position(|x| *x == sig2).unwrap() + 1;
    println!("{:#?} {}", first_idx, second_idx);
    println!("{}", first_idx * second_idx);
}

fn compare(l: &str, r: &str) -> i32 {
    println!("{} : {}", l, r);
    //
    if l == "[]" && r != "[]" {
        return 1;
    } else if l != "[]" && r == "[]" {
        return -1;
    } else if l == "[]" && r == "[]" {
        return 0;
    }

    let ll = split_list(l);
    let rr = split_list(r);

    for (lll, rrr) in ll.iter().zip(rr.iter()) {
        if lll.starts_with('[') && rrr.starts_with('[') {
            let res = compare(lll, rrr);
            if res != 0 {
                return res;
            }
        }
        if lll.starts_with('[') && !rrr.starts_with('[') {
            let nested = format!("[{}]", rrr);
            let res = compare(lll, &nested);
            if res != 0 {
                return res;
            }
        }

        if !lll.starts_with('[') && rrr.starts_with('[') {
            let nested = format!("[{}]", lll);
            let res = compare(&nested, rrr);
            if res != 0 {
                return res;
            }
        }

        if !lll.starts_with('[') && !rrr.starts_with('[') {
            let l_val = lll.parse::<i32>().unwrap();
            let r_val = rrr.parse::<i32>().unwrap();
            if l_val < r_val {
                return 1;
            }
            if l_val > r_val {
                return -1;
            }
        }
    }
    if ll.len() > rr.len() {
        return -1;
    } else if ll.len() < rr.len() {
        return 1;
    }
    return 0;
}

fn split_list(s: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut in_array_depth = 0;
    let mut child = String::from("");
    for c in s.chars() {
        if c == '[' {
            if in_array_depth != 0 {
                child.push(c)
            }
            in_array_depth += 1;
        } else if c == ']' {
            in_array_depth -= 1;
            if in_array_depth != 0 {
                child.push(c)
            }
        } else if c == ',' && in_array_depth == 1 {
            res.push(child);
            child = String::from("");
        } else {
            child.push(c)
        }
    }
    res.push(child);

    return res;
}
