use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    next_items: Vec<i64>,
    op: String,
    test: i64,
    if_true: i64,
    if_false: i64,
    count: i64,
}

fn main() {
    let input = read_to_string("src/data").unwrap();
    let monkeys = input.split("\n\n");
    let mut all = Vec::new();
    for m in monkeys {
        let mut monkey = Monkey {
            items: Vec::new(),
            next_items: Vec::new(),
            op: String::from(""),
            test: 0,
            if_true: 0,
            if_false: 0,
            count: 0,
        };

        for (idx, l) in m.lines().enumerate() {
            // if l.starts_with("Monkey ") {
            //     let idx: i64 = l.matches(char::is_numeric).next().unwrap().parse().unwrap();
            // monkey.items.push(idx)
            // }
            //

            if idx == 1 {
                let start = l
                    .trim_start_matches("  Starting items: ")
                    .split(", ")
                    .map(|i| i.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                monkey.items = start;
            }
            if idx == 2 {
                let op = l.trim_start_matches("  Operation: new = ");
                let mut op_vec = op.split_whitespace();
                let first = op_vec.next().unwrap();
                let operation = op_vec.next().unwrap();
                let last = op_vec.next().unwrap();
                monkey.op = op.to_string();
                // println!("{:?}", op);
            }
            if idx == 3 {
                let test = l
                    .trim_start_matches("  Test: divisible by ")
                    .parse()
                    .unwrap();
                monkey.test = test;
            }
            if idx == 4 {
                let if_true = l
                    .trim_start_matches("    If true: throw to monkey ")
                    .parse()
                    .unwrap();
                monkey.if_true = if_true;
            }
            if idx == 5 {
                let if_false = l
                    .trim_start_matches("    If false: throw to monkey ")
                    .parse()
                    .unwrap();
                monkey.if_false = if_false;
            }
        }

        all.push(monkey);
    }
    let mut monkeys = all.clone();

    let krn: i64 = monkeys.iter().map(|m| m.test).product();
    println!("{:?}", krn);

    for _i in 0..10000 {
        for (idx, monkey) in all.iter_mut().enumerate() {
            for item in monkeys[idx].items.clone() {
                let op = monkey.op.to_string();
                let mut op_vec = op.split_whitespace();
                let first = op_vec.next().unwrap().parse::<i64>().unwrap_or(item);
                let operation = op_vec.next().unwrap();
                let last = op_vec.next().unwrap().parse::<i64>().unwrap_or(item);

                let mut new_val = 0;
                if operation == "+" {
                    new_val = first + last;
                }
                if operation == "*" {
                    new_val = first * last;
                }

                new_val = new_val % krn;

                if new_val % monkey.test == 0 {
                    monkeys[monkey.if_true as usize].items.push(new_val);
                } else {
                    monkeys[monkey.if_false as usize].items.push(new_val);
                }
                monkeys[idx].items.remove(0);
                monkeys[idx].count += 1;
                // monkey.count += 1;
            }
        }

        // for m in monkeys.iter_mut() {
        //     m.items = m.next_items.clone();
        //     m.next_items = Vec::new();
        // println!("{:?}", m.items);
        // }

        // all = monkeys.clone();
        let mut counts = monkeys.iter().map(|m| m.count).collect::<Vec<i64>>();
        // counts.sort();
        // counts.reverse();
        println!("{:?}", counts);
    }

    // println!("{:#?}", monkeys);
    let mut counts = monkeys.iter().map(|m| m.count).collect::<Vec<i64>>();
    counts.sort();
    counts.reverse();
    // println!("{:#?}", counts);
    println!("{}, {}", counts[0], counts[1]);
    println!("{:#?}", counts[0] * counts[1]);
}
