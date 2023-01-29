use std::{collections::LinkedList, fs::read_to_string};

#[derive(Clone, Debug)]
struct Node {
    val: i64,
    idx: i64,
}

struct File {
    oir: Vec<i64>,
    data: Vec<i64>,
    current: usize,
}

fn main() {
    let key = 811589153;
    // let input = read_to_string("src/test").unwrap();
    let input = read_to_string("src/data").unwrap();
    let mut nums = Vec::new();
    // nums.reserve(6000);
    // let mut list = LinkedList::new();
    for (idx, l) in input.lines().enumerate() {
        let val: i64 = l.parse::<i64>().unwrap() * key;
        let i: i64 = idx.try_into().unwrap();

        let n = Node { idx: i, val };
        //
        // let b = Box::new(n);

        nums.push(n);
        // list.push_back(val);
    }
    // for n in nums.iter_mut() {
    //     n.idx += n.val;
    // }
    let len = nums.len() as i64;
    // let c = list.iter().find(|l| l ==);
    //
    // nums.sort();
    // nums.dedup();
    // println!("count {}", nums.len());
    let ori = nums.clone();

    for i in 0..10 {
        for node in ori.clone() {
            let idx: i64 = nums
                .iter()
                .position(|a| a.idx == node.idx)
                .unwrap()
                .try_into()
                .unwrap();
            let mut new_idx = idx + node.val;
            // println!("{}", new_idx);
            if new_idx > len - 1 {
                new_idx = new_idx % (nums.len() as i64 - 1);
            }
            // println!("{}", new_idx);
            if new_idx < 0 {
                new_idx = new_idx % (nums.len() as i64 - 1);
                new_idx += nums.len() as i64 - 1;
                // println!("--{}", new_idx);
            }
            if new_idx == len - 1 {
                new_idx = 0
            }
            // println!("{}", new_idx);
            nums.remove(idx as usize);
            nums.insert(new_idx as usize, node);

            // println!(
            //     "{:?},{}",
            //     nums.iter().map(|a| a.val).collect::<Vec<i64>>(),
            //     new_idx
            // );
        }
        // println!("{:?}", nums.iter().map(|a| a.val).collect::<Vec<i64>>());
    }

    let zero_idx: i64 = nums
        .iter()
        .position(|a| a.val == 0)
        .unwrap()
        .try_into()
        .unwrap();
    println!("zero {}", zero_idx);

    let mut sum = 0;

    for num in [1000, 2000, 3000] {
        let mut new_idx = num;
        if num > len {
            new_idx = new_idx % (len);
        }
        new_idx = zero_idx + new_idx;
        if new_idx >= len {
            new_idx = new_idx % (nums.len() as i64);
        }
        println!("new {}", new_idx);
        // if new_idx < 0 {
        //     new_idx += nums.len() as i64 - 1
        // }
        // if new_idx == 0 {
        //     new_idx = len - 1
        // }
        let v = &nums[new_idx as usize];
        println!("{}", v.val);
        sum += v.val;
    }
    println!("{}", sum);

    // nums.sort_by(|a, b| a.idx.cmp(&b.idx));

    // println!("{:#?}", nums);

    // for i in 0..nums.len() {
    //     nums[i].next = &Some(nums[i + 1]);
    //     // nums[i].prev = Some(Box::new(nums[i - 1]));
    // }
}
