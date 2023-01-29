use std::{collections::HashMap, fs::read_to_string};

struct Game {
    map: HashMap<(i32, i32), (i32, i32)>,
    direction: i32,
    is_move: bool,
}

static DIRECTION: [[(i32, i32); 3]; 4] = [
    [(1, -1), (0, -1), (-1, -1)],
    [(1, 1), (0, 1), (-1, 1)],
    [(-1, -1), (-1, 0), (-1, 1)],
    [(1, -1), (1, 0), (1, 1)],
];
static SURROUND: [(i32, i32); 8] = [
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Game {
    fn turn(&mut self) {
        self.is_move = false;
        for e in self.map.clone().keys() {
            if self.is_sur(e) {
                self.go_move(e);
            }
        }
        let dup = self.check_dup();
        for e in self.map.iter_mut() {
            let count = dup.get(e.1).unwrap();
            if count > &1 {
                *e.1 = e.0.clone();
            }
        }
        let mut new_map = HashMap::new();
        for e in self.map.iter_mut() {
            if e.0 .0 != e.1 .0 || e.0 .1 != e.1 .1 {
                self.is_move = true;
            }
            new_map.insert(*e.1, *e.1);
            // let count = dup.get(e.1).unwrap();
            // if count > &1 {
            //     *e.1 = e.0.clone();
            // }
        }
        self.map = new_map;
        self.direction += 1;
    }
    fn is_sur(&self, (x, y): &(i32, i32)) -> bool {
        for (x_d, y_d) in SURROUND {
            let peak = (x + x_d, y + y_d);
            if self.map.get(&peak).is_some() {
                return true;
            }
        }
        return false;
    }
    fn go_move(&mut self, (x, y): &(i32, i32)) {
        for i in 0..4 {
            let idx = (i + self.direction) % 4;
            let mut can_move = true;
            let direction = DIRECTION[idx as usize];
            for (x_d, y_d) in direction.iter() {
                let peak = (x + x_d, y + y_d);
                if self.map.get(&peak).is_some() {
                    can_move = false;
                    break;
                }
            }
            if can_move {
                self.map
                    .insert((*x, *y), (x + direction[1].0, y + direction[1].1));
                break;
            }
        }
    }
    fn check_dup(&self) -> HashMap<(i32, i32), i32> {
        let mut count = HashMap::new();
        for m in self.map.iter() {
            *count.entry(*m.1).or_insert(0) += 1;
        }
        // println!("{:?}", count);
        return count;
    }
    fn print(&self) {
        for i in 0..20 {
            for j in 0..20 {
                if self.map.get(&(j, i)).is_some() {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            print!("\n");
        }
    }
}
fn main() {
    let input = read_to_string("src/data").unwrap();
    let mut map = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            if c == '#' {
                map.insert((x, y), (x, y));
            }
        }
    }
    let mut game = Game {
        map,
        direction: 0,
        is_move: true,
    };
    // println!("{:?}", game.map.keys());
    game.print();
    // for i in 0..10 {
    //     game.turn();
    //     game.print();
    //     // println!("{:?}", game.map.keys());
    // }
    while game.is_move {
        game.turn();
        game.print();
    }
    let width = game.map.keys().map(|p| p.1).max().unwrap()
        - game.map.keys().map(|p| p.1).min().unwrap()
        + 1;
    let height = game.map.keys().map(|p| p.0).max().unwrap()
        - game.map.keys().map(|p| p.0).min().unwrap()
        + 1;
    let count = game.map.keys().len() as i32;
    dbg!(width, height, width * height - count);
    dbg!(game.direction);
}
