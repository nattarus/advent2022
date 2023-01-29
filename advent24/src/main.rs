use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, Debug)]
struct Storm {
    map: HashMap<(i32, i32), Vec<(i32, i32)>>,
    map_loop: Vec<HashMap<(i32, i32), Vec<(i32, i32)>>>,
    width: i32,
    height: i32,
    rmn: i32,
    start: (i32, i32),
    target: (i32, i32),
}

impl Storm {
    fn turn(&mut self) {
        let mut new_map: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        for (k, v) in self.map.iter() {
            for d in v {
                let mut n_x = k.0 + d.0;
                if n_x == self.width - 1 {
                    n_x = 1
                }
                if n_x == 0 {
                    n_x = self.width - 2
                }
                let mut n_y = k.1 + d.1;
                if n_y == self.height - 1 {
                    n_y = 1
                }
                if n_y == 0 {
                    n_y = self.height - 2
                }
                // new_map
                //     .get_mut(&(n_x, n_y))
                //     .get_or_insert(&mut Vec::new())
                //     .push(*d);
                new_map
                    .entry((n_x, n_y))
                    .and_modify(|v| v.push(*d))
                    .or_insert(vec![*d]);
            }
        }
        self.map = new_map;
    }
    fn gen_map_loop(&mut self) {
        let rmn = (self.width - 2) * (self.height - 2);
        let mut temp = self.clone();
        for i in 0..rmn {
            temp.turn();
            self.map_loop.push(temp.map.clone());
        }
    }
    fn can_walk(&self, turn: i32, point: &(i32, i32)) -> bool {
        if point == &self.target {
            return true;
        }
        if point == &self.start {
            return true;
        }
        if point.0 <= 0 || point.0 >= self.width - 1 || point.1 <= 0 || point.1 >= self.height - 1 {
            return false;
        }
        let idx = turn % self.rmn;
        if self.map_loop[idx as usize].get(&point).is_some() {
            return false;
        } else {
            return true;
        }
    }
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(d) = self.map.get(&(x, y)) {
                    for di in DIRECTION {
                        if di.1 == d[0] {
                            print!("{}", di.0)
                        }
                    }
                } else {
                    print!(".")
                }
            }
            print!("\n");
        }
    }
}

fn walk(
    current: (i32, i32),
    storm: &Storm,
    count: i32,
    mut min: i32,
    cache: &mut HashMap<((i32, i32), i32), i32>,
) -> i32 {
    // dbg!(current, count);

    let count = count + 1;
    // storm.turn();

    if let Some(_i) = cache.get(&(current, count)) {
        return min;
    } else {
        cache.insert((current, count), 1);
    }

    if storm.target == current {
        if count < min {
            min = count;

            // dbg!(min, current);
        }
        return min;
    }
    if count > 10000 || count >= min {
        return min;
    }
    let up = (current.0, current.1 - 1);
    let down = (current.0, current.1 + 1);
    let left = (current.0 - 1, current.1);
    let right = (current.0 + 1, current.1);

    if storm.can_walk(count, &down) {
        min = walk(down, storm, count, min, cache);
    }
    if storm.can_walk(count, &right) {
        min = walk(right, storm, count, min, cache);
    }

    if storm.can_walk(count, &up) {
        min = walk(up, storm, count, min, cache);
    }
    if storm.can_walk(count, &left) {
        min = walk(left, storm, count, min, cache);
    }
    if storm.can_walk(count, &current) {
        min = walk(current, storm, count, min, cache);
    }

    return min;
}

const DIRECTION: [(char, (i32, i32)); 4] =
    [('<', (-1, 0)), ('>', (1, 0)), ('^', (0, -1)), ('v', (0, 1))];

fn main() {
    let input = read_to_string("src/data").unwrap();
    let height = input.lines().count() as i32;
    let width = input.lines().map(|l| l.chars().count()).max().unwrap() as i32;
    let mut current = (1, 0);
    let target = (width - 2, height - 1);
    let mut map: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            for d in DIRECTION {
                if c == d.0 {
                    map.entry((x, y))
                        .and_modify(|v| v.push(d.1))
                        .or_insert(vec![d.1]);
                }
            }
        }
    }
    let rmn = (width - 2) * (height - 2);
    let mut storm = Storm {
        map,
        width,
        height,
        rmn,
        map_loop: Vec::new(),
        start: current,
        target,
    };
    dbg!(storm.rmn, storm.width, storm.height);
    storm.gen_map_loop();
    println!("gen loop");
    let mut cache = HashMap::new();
    // dbg!(storm.map);
    // storm.print();
    // storm.turn();
    // storm.print();
    // storm.turn();
    // storm.print();
    // storm.turn();
    // storm.print();
    let first = walk(current, &storm, 0, 9999, &mut cache);
    dbg!(first);
    storm.start = target;
    storm.target = (0, 1);
    let mut cache = HashMap::new();

    let second = walk(target, &storm, first, 29999, &mut cache);
    dbg!(second - first);

    storm.start = (0, 1);
    storm.target = target;
    let mut cache = HashMap::new();
    let third = walk((0, 1), &storm, second, 29999, &mut cache);
    dbg!(third - second, third);
}
