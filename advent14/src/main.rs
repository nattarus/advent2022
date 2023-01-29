use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq)]
enum Tile {
    Emphy,
    Sand,
    Rock,
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = read_to_string("./src/data").unwrap();
    let mut map: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut dia = vec![vec!['.'; 200]; 170];
    for l in input.lines() {
        let mut dots = Vec::new();
        for dot in l.split(" -> ") {
            let x = dot.split(',').next().unwrap().parse::<i32>().unwrap();
            let y = dot.split(',').last().unwrap().parse::<i32>().unwrap();
            dots.push((x, y));
        }
        for (from, to) in dots.iter().zip(dots.iter().skip(1)) {
            if to.0 == from.0 {
                let start = to.1.min(from.1);
                let end = to.1.max(from.1);
                let diff = end - start + 1;
                for i in 0..diff {
                    map.insert((to.0 - 400, start + i), Tile::Rock);
                }
            }
            if to.1 == from.1 {
                let start = to.0.min(from.0);
                let end = to.0.max(from.0);
                let diff = end - start + 1;
                for i in 0..diff {
                    map.insert((start + i - 400, to.1), Tile::Rock);
                }
            }
        }
    }
    let mut sand_pos = (100, 0);
    let mut count = 1;
    let highest = map.keys().map(|p| p.1).max().unwrap();
    let mut full = false;
    println!("{}", highest);
    while sand_pos.1 < 1500 {
        // break;
        if full {
            break;
        }
        while sand_pos.1 < 1500 && !full {
            println!("{:?}", sand_pos);
            if sand_pos.1 != highest + 1 {
                let under = map.get(&(sand_pos.0, sand_pos.1 + 1));
                if let None = under {
                    sand_pos = (sand_pos.0, sand_pos.1 + 1);
                    // println!("down");
                    continue;
                }

                let left = map.get(&(sand_pos.0 - 1, sand_pos.1 + 1));
                if let None = left {
                    sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
                    println!("left");
                    continue;
                }
                let right = map.get(&(sand_pos.0 + 1, sand_pos.1 + 1));
                if let None = right {
                    println!("right");
                    sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
                    continue;
                }
                if sand_pos.1 == 0 {
                    println!("{}", count);
                    full = true;
                    break;
                }
            }
            map.insert(sand_pos, Tile::Sand);
            count += 1;
            sand_pos = (500, 0);
            break;
        }
    }
    // println!("{}", count);
    for t in map {
        let x = t.0 .0;
        let y = t.0 .1;
        let tile = t.1;
        if tile == Tile::Rock {
            dia[y as usize][x as usize] = 'x'
        }
        if tile == Tile::Sand {
            dia[y as usize][x as usize] = 's'
        }
    }
    for l in dia {
        let line: String = l.iter().collect();
        println!("{}", line)
    }
    // for

    // println!("Hello, world! {:#?}", map.keys());
}
