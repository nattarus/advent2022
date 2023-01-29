use std::{fs::read_to_string, str::Chars};

fn main() {
    let input = read_to_string("src/data").unwrap();
    let wind = input.chars().collect::<Vec<char>>();

    println!("{}", wind.len());
    let mut tiles = Tertis::new(wind);
    // let rmn = (tiles.wind.len()) * 5;

    let elephent: usize = 1000000000000;
    // let elephent: usize = 2022;
    // for i in 0..elephent {
    //     // if i % 100000 == 0 {
    //     //     println!("{}", i);
    //     // }
    //     tiles.turn();
    // }
    for i in 0..tiles.rmn {
        tiles.turn();
    }
    let first_height = tiles.height_store;
    let first_idx = tiles.rock_idx;

    for i in 0..tiles.rmn * 5 {
        tiles.turn();
    }
    let second_h = tiles.height;
    let second_idx = tiles.rock_idx;

    // for _i in 0..left {
    //     tiles.turn();
    // }

    let times = elephent / tiles.rmn;
    println!("{} / {} = {}", elephent, tiles.rmn, times);
    println!("{} * {} = {}", times, tiles.rmn, times * tiles.rmn);

    println!("{} * {} = {}", times, tiles.diff, times * tiles.diff);

    tiles.height_store = times * tiles.diff - tiles.diff + first_height;
    println!("{}", first_height);

    let left = elephent % tiles.rmn;

    for i in 0..left {
        tiles.turn();
    }
    // tiles.print();
    println!(
        "{} + {} = {}",
        tiles.height,
        tiles.height_store,
        tiles.height + tiles.height_store
    );
}

enum Tile {
    Rock,
    Emthy,
}

struct Tertis {
    map: Vec<Vec<char>>,
    height: usize,
    rock: Option<Rock>,
    wind: Vec<char>,
    diff: usize,
    wind_idx: usize,
    rock_idx: usize,
    height_store: usize,
    rmn: usize,
}

impl Tertis {
    fn new(wind: Vec<char>) -> Self {
        let rocks = vec![vec!['.'; 7]; 10];
        let rmn = (wind.len()) * 5 * 1;
        return Self {
            map: rocks,
            height: 0,
            rock: None,
            wind,
            wind_idx: 0,
            diff: 0,
            rock_idx: 1,
            height_store: 0,
            rmn,
        };
    }
    fn truncate(&mut self) {
        let mut area = Vec::new();
        flood(&self.map, (0, self.map.len() - 1), &mut area);
        // println!("{:?}", area);
        // println!("{}", area.iter().map(|a| a.1).min().unwrap());
        let height = area.iter().map(|a| a.1).min().unwrap();
        // println!("{}", height);
        // let mut prev = vec!['.'; 7].as_ref();
        // for (i, l) in self.map.iter().rev().enumerate() {
        // for c, p_c in l.iter().zip(prev.iter()) {
        //
        // }
        // if l.iter().all(|t| *t == 'x') {
        // let height = self.map.len() - i;
        if height != 0 {
            let height = height - 1;
            self.height_store += height;
            self.height -= height;
            self.diff = height;
            let rocks = vec![vec!['.'; 7]; height];
            self.map = self.map[height..self.map.len()].to_vec();
            self.map.extend(rocks);

            println!(
                "truncate!!! height:{} idx:{} store:{} ,map_len:{}",
                height,
                self.rock_idx,
                self.height_store,
                self.map.len()
            );
        }
        // break;
        // }
        // prev = l
        // if i > self.height {
        //     break;
        // }
        // }
    }
    fn turn(&mut self) {
        self.insert_rock();
        // self.print();
        while self.rock.is_some() {
            self.wind_action();
            self.move_rock_down();
        }
        if self.rock_idx % self.rmn == 0 {
            self.truncate();
            // println!("{} {}", self.height, self.rock_idx)
        }
        self.update_height();
        // self.print();
    }

    fn read_wind(&mut self) -> char {
        if self.wind_idx == self.wind.len() - 1 {
            self.wind_idx = 1;
            return self.wind[0];
        }
        let c = self.wind[self.wind_idx];
        self.wind_idx += 1;
        return c;
    }
    fn wind_action(&mut self) {
        let c = self.read_wind();
        if c == '<' {
            if self.can_move_left() {
                self.move_rock_left()
            }
        } else if c == '>' {
            if self.can_move_right() {
                self.move_rock_right()
            }
        } else {
            println!("{}", c);
            panic!();
        }
    }

    fn can_move_left(&self) -> bool {
        let rock = self.rock.as_ref().unwrap();
        for p in rock.pos.iter() {
            if p.0 == 0 || self.map[p.1][p.0 - 1] != '.' {
                return false;
            }
        }
        return true;
    }
    fn move_rock_left(&mut self) {
        let rock = self.rock.as_mut().unwrap();
        for p in rock.pos.iter_mut() {
            p.0 -= 1;
        }
    }
    fn can_move_right(&self) -> bool {
        let rock = self.rock.as_ref().unwrap();
        for p in rock.pos.iter() {
            if p.0 == 6 || self.map[p.1][p.0 + 1] != '.' {
                return false;
            }
        }
        return true;
    }
    fn move_rock_right(&mut self) {
        let rock = self.rock.as_mut().unwrap();
        for p in rock.pos.iter_mut() {
            p.0 += 1;
        }
    }

    fn insert_rock(&mut self) {
        if self.rock_idx % 5 == 1 {
            let rock = Rock::new_1(self.height);
            self.rock = rock.into();
        }
        if self.rock_idx % 5 == 2 {
            let rock = Rock::new_2(self.height);
            self.rock = rock.into();
        }
        if self.rock_idx % 5 == 3 {
            let rock = Rock::new_3(self.height);
            self.rock = rock.into();
        }
        if self.rock_idx % 5 == 4 {
            let rock = Rock::new_4(self.height);
            self.rock = rock.into();
        }
        if self.rock_idx % 5 == 0 {
            let rock = Rock::new_5(self.height);
            self.rock = rock.into();
        }
        self.rock_idx += 1;
    }
    fn update_height(&mut self) -> usize {
        for (i, l) in self.map.iter().skip(self.height as usize).enumerate() {
            if l.iter().all(|t| *t == '.') {
                self.height = self.height + i;
                let diff = self.map.len() - self.height;
                if diff < 10 {
                    let rocks = vec![vec!['.'; 7]; 20];
                    self.map.extend(rocks);
                }
                break;
            }
        }
        return self.height;
    }

    fn can_move_rock_down(&mut self) -> bool {
        let rock = self.rock.as_mut().unwrap();
        for p in rock.pos.iter_mut() {
            if p.1 == 0 || self.map[p.1 - 1][p.0] != '.' {
                return false;
            }
        }
        return true;
    }
    fn move_rock_down(&mut self) {
        if self.can_move_rock_down() {
            let rock = self.rock.as_mut().unwrap();
            for p in rock.pos.iter_mut() {
                p.1 -= 1;
            }
        } else {
            self.merge_rock();
        }
    }
    fn merge_rock(&mut self) {
        let rock = self.rock.as_mut().unwrap();
        for p in rock.pos.iter_mut() {
            self.map[p.1][p.0] = 'x'
        }
        self.rock = None;
    }
    fn print(&self) {
        let mut map_with_rock = self.map.clone();
        // let rock  =self.rock.as_ref().unwrap()
        if let Some(rock) = self.rock.as_ref() {
            for pos in rock.pos.iter() {
                map_with_rock[pos.1][pos.0] = 'x';
            }
        }
        for l in map_with_rock.iter().rev() {
            println!(
                "|{}|",
                l.iter().map(|a| a.to_string()).collect::<Vec<_>>().join("")
            );
        }
        println!("------------------------");
    }
}

struct Rock {
    pos: Vec<(usize, usize)>,
}
impl Rock {
    fn new_1(height: usize) -> Self {
        let y = height + 3;
        let pos = vec![(2, y), (3, y), (4, y), (5, y)];
        return Self { pos };
    }
    fn new_2(height: usize) -> Self {
        let y = height + 3;
        let pos = vec![(3, y), (2, y + 1), (3, y + 1), (4, y + 1), (3, y + 2)];
        return Self { pos };
    }
    fn new_3(height: usize) -> Self {
        let y = height + 3;
        let pos = vec![(2, y), (3, y), (4, y), (4, y + 1), (4, y + 2)];
        return Self { pos };
    }
    fn new_4(height: usize) -> Self {
        let y = height + 3;
        let pos = vec![(2, y), (2, y + 1), (2, y + 2), (2, y + 3)];
        return Self { pos };
    }
    fn new_5(height: usize) -> Self {
        let y = height + 3;
        let pos = vec![(2, y), (3, y), (2, y + 1), (3, y + 1)];
        return Self { pos };
    }
}

fn flood(map: &Vec<Vec<char>>, current: (usize, usize), area: &mut Vec<(usize, usize)>) {
    if area.iter().position(|a| *a == current).is_some() {
        return ();
    }
    area.push(current);
    if current.1 != 0 && map[current.1 - 1][current.0] == '.' {
        flood(map, (current.0, current.1 - 1), area)
    }
    if current.0 != 0 && map[current.1][current.0 - 1] == '.' {
        flood(map, (current.0 - 1, current.1), area)
    }
    if current.0 != 6 && map[current.1][current.0 + 1] == '.' {
        flood(map, (current.0 + 1, current.1), area)
    }
}
