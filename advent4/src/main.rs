use std::fs;

fn main() {
    let input = fs::read_to_string("./src/data").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let mut elves = line.split(",");
        let mut first = elves.next().unwrap().split('-');
        let mut second = elves.next().unwrap().split('-');
        let first_start = first.next().unwrap().parse::<u32>().unwrap();
        let first_end = first.next().unwrap().parse::<u32>().unwrap();
        let second_start = second.next().unwrap().parse::<u32>().unwrap();
        let second_end = second.next().unwrap().parse::<u32>().unwrap();

        if first_start < second_start && first_end < second_start {
            println!("{}", line);
            sum = sum + 1
        } else if second_start < first_start && second_end < first_start {
            println!("{}", line);
            sum = sum + 1
        }
    }
    println!("{}", input.lines().count() - sum);
}
