use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("./src/data").unwrap();
    let mut data = Vec::new();
    for l in input.lines() {
        let mut iter = l.split(",");
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        let z: i32 = iter.next().unwrap().parse().unwrap();
        data.push((x, y, z));
    }
    // println!("{}", count_surface(&data));
    // let all_s = data.len() * 6;
    // println!("{},{}", data.len(), all_s);
    // let mut count = 0;
    // while data.len() != 0 {
    //     let c = data.pop().unwrap();
    //
    //     for (x, y, z) in data.iter() {
    //         if x == &c.0 && y == &c.1 && (&c.2 - z).abs() == 1 {
    //             count += 1;
    //             continue;
    //         }
    //         if y == &c.1 && z == &c.2 && (&c.0 - x).abs() == 1 {
    //             count += 1;
    //             continue;
    //         }
    //         if z == &c.2 && x == &c.0 && (&c.1 - y).abs() == 1 {
    //             count += 1;
    //             continue;
    //         }
    //     }
    // }
    // print!("{} - {} = {}", all_s, count * 2, all_s - count * 2);

    let max_x = data.iter().map(|x| x.0).max().unwrap();
    let max_y = data.iter().map(|x| x.1).max().unwrap();
    let max_z = data.iter().map(|x| x.2).max().unwrap();
    println!("{}, {}, {}", max_x, max_y, max_z);

    // let map = vec![vec!(vec!('x'; max_z); max_y); max_x];
    let mut hash_map = HashMap::new();
    for p in data.iter() {
        hash_map.insert(p.clone(), 'x');
    }
    let mut spaces = Vec::new();
    flood((0, 0, 0), &hash_map, &mut spaces);

    println!("{}", spaces.len());

    let mut inside = Vec::new();
    for x in 0..24 {
        for y in 0..24 {
            for z in 0..24 {
                let pos = (x, y, z);
                if hash_map.get(&&pos).is_none() && spaces.iter().find(|p| p == &&pos).is_none() {
                    inside.push(pos);
                }
            }
        }
    }
    let sur = count_surface(&data);
    // println!("{:?}", inside);
    let i_sur = count_surface(&inside);

    println!("{} - {} = {}", sur, i_sur, sur - i_sur);
}

fn count_surface(data: &Vec<(i32, i32, i32)>) -> usize {
    let mut temp = data.clone();
    let all_s = data.len() * 6;
    println!("{},{}", data.len(), all_s);
    let mut count = 0;
    while temp.len() != 0 {
        let c = temp.pop().unwrap();

        for (x, y, z) in data.iter() {
            if x == &c.0 && y == &c.1 && (&c.2 - z).abs() == 1 {
                count += 1;
                continue;
            }
            if y == &c.1 && z == &c.2 && (&c.0 - x).abs() == 1 {
                count += 1;
                continue;
            }
            if z == &c.2 && x == &c.0 && (&c.1 - y).abs() == 1 {
                count += 1;
                continue;
            }
        }
    }
    println!("{} - {} = {}", all_s, count, all_s - count);
    return all_s - count;
}

fn flood(
    pos: (i32, i32, i32),
    hash_map: &HashMap<(i32, i32, i32), char>,
    spaces: &mut Vec<(i32, i32, i32)>,
) {
    let current = hash_map.get(&pos);
    if current.is_some()
        || spaces.iter().position(|a| *a == pos).is_some()
        || pos.0 > 24
        || pos.0 < 0
        || pos.1 > 24
        || pos.1 < 0
        || pos.2 > 24
        || pos.2 < 0
    {
        return;
    } else {
        spaces.push(pos);
    }

    flood((pos.0 - 1, pos.1, pos.2), hash_map, spaces);
    flood((pos.0 + 1, pos.1, pos.2), hash_map, spaces);
    flood((pos.0, pos.1 - 1, pos.2), hash_map, spaces);
    flood((pos.0, pos.1 + 1, pos.2), hash_map, spaces);
    flood((pos.0, pos.1, pos.2 - 1), hash_map, spaces);
    flood((pos.0, pos.1, pos.2 + 1), hash_map, spaces);
}
