use std::{collections::HashMap, fmt, fs, io::Error, thread, time};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("./src/data")?;
    let (dia, instuction) = parse_block(&input);
    let mut stack = Stack::new(dia);
    for inc in instuction.lines() {
        println!("{}", inc);
        if !inc.is_empty() {
            let (amount, from, to) = parse_instruction(inc);
            stack.move_inc2(amount, from, to);
            println!("{}", stack);
            let ten_millis = time::Duration::from_millis(100);

            thread::sleep(ten_millis);
            print!("{}[2J", 27 as char);
        }
    }

    let mut res = Vec::new();
    for col in stack.data {
        res.push(col.last().unwrap().clone());
    }
    println!("{}", res.iter().clone().collect::<String>());
    // println!( "{}",res.iter().collect::<&str>().join(''));

    return Ok(());
}

struct Stack {
    data: Vec<Vec<char>>,
}

impl Stack {
    fn new(dia: &str) -> Self {
        let mut stack = Vec::new();
        let mut data = dia.lines().rev();
        let keys_line = data.next().unwrap();
        let keys = keys_line.split_whitespace();

        let mut locs = Vec::new();
        for col in keys {
            let loc = keys_line.find(col).unwrap();
            let idx = col.parse::<i32>().unwrap();
            locs.push((idx, loc));
            stack.push(Vec::new());
        }

        for line in data {
            for (idx, loc) in locs.iter() {
                let val = line.chars().nth(*loc);
                if let Some(block) = val {
                    if !block.is_whitespace() {
                        stack[*idx as usize - 1].push(block);
                    }
                }
            }
        }
        return Stack { data: stack };
    }

    fn move_inc(&mut self, amount: usize, from: usize, to: usize) {
        for _ in 0..amount {
            // let mut from_vec = self.data[from];
            let block = self.data[from].pop().unwrap();
            self.data[to].push(block);
            println!("{}{}", from, to);
        }
    }
    fn move_inc2(&mut self, amount: usize, from: usize, to: usize) {
        let mut blocks = Vec::new();
        for _ in 0..amount {
            // let mut from_vec = self.data[from];
            let block = self.data[from].pop().unwrap();
            blocks.push(block);
        }
        blocks.reverse();
        self.data[to].append(&mut blocks);
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.data {
            write!(f, "{:?}\n", line)?;
        }
        write!(f, "----")
    }
}

fn parse_block(input: &str) -> (&str, &str) {
    let mut data = input.split("\n\n");
    let dia = data.next().unwrap();
    let instuction = data.next().unwrap();
    return (dia, instuction);
}

fn parse_instruction(inc: &str) -> (usize, usize, usize) {
    let mut data = inc.split_whitespace();
    let amount = data.nth(1).unwrap().parse::<usize>().unwrap();
    let to = data.nth(1).unwrap().parse::<usize>().unwrap() - 1;
    let from = data.nth(1).unwrap().parse::<usize>().unwrap() - 1;
    return (amount, to, from);
}
