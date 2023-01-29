use std::{collections::HashMap, fs::read_to_string};

const DIRECTION: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

struct Puzzle {
    field: HashMap<(i32, i32), char>,
    width: i32,
    height: i32,
    size: i32,
}
impl Puzzle {
    fn peak2(&self, current: &(i32, i32), u_d: usize) -> ((i32, i32), usize) {
        let (mut x, mut y) = current.clone();
        let mut d = u_d;
        x += DIRECTION[d].0;
        y += DIRECTION[d].1;
        // dbg!(x, y, self.in_block(&(x, y)));
        let b_x = self.in_block(&x);
        let b_y = self.in_block(&y);
        // dbg!(block);
        if b_x == 0 && b_y == 0 && d == 2 {
            // println!("dang");
            d = 0;
            // println!("dang");
            (x, y) = (0, (self.size * 3 - 1) - (y % self.size));
        }
        if b_x == -1 && b_y == 2 && d == 2 {
            d = 0;
            (x, y) = (self.size, (self.size - 1) - (y % self.size));
        }

        if b_x == 1 && b_y == -1 && d == 3 {
            d = 0;
            (x, y) = (0, self.size * 3 + (x % self.size));
        }
        if b_x == -1 && b_y == 3 && d == 2 {
            d = 1;
            (x, y) = (self.size + (y % self.size), 0);
        }

        if b_x == 2 && b_y == -1 && d == 3 {
            d = 3;
            (x, y) = (x % self.size, (4 * self.size) - 1);
        }
        if b_x == 0 && b_y == 4 && d == 1 {
            d = 1;
            (x, y) = ((2 * self.size) + x, 0);
        }

        if b_x == 3 && b_y == 0 && d == 0 {
            d = 2;
            (x, y) = ((2 * self.size) - 1, ((3 * self.size) - 1) - y);
        }
        if b_x == 2 && b_y == 2 && d == 0 {
            d = 2;
            (x, y) = ((3 * self.size) - 1, (self.size - 1) - (y % self.size));
        }

        if b_x == 2 && b_y == 1 && d == 1 {
            d = 2;
            (x, y) = ((2 * self.size) - 1, self.size + (x % self.size));
        }
        if b_x == 2 && b_y == 1 && d == 0 {
            d = 3;
            (x, y) = (2 * self.size + (y % self.size), self.size - 1);
        }

        if b_x == 0 && b_y == 1 && d == 2 {
            d = 1;
            (x, y) = (y % self.size, 2 * self.size);
        }
        if b_x == 0 && b_y == 1 && d == 3 {
            d = 0;
            (x, y) = (self.size, self.size + (x % self.size));
        }

        if b_x == 1 && b_y == 3 && d == 1 {
            d = 2;
            (x, y) = (self.size - 1, 3 * self.size + (x % self.size));
        }
        if b_x == 1 && b_y == 3 && d == 0 {
            d = 3;
            (x, y) = (self.size + (y % self.size), ((3 * self.size) - 1));
        }
        // if b_x == 3 && b_y == 0 && d == 0 {
        //
        // }
        let c = self.field.get(&(x, y)).unwrap();
        if *c == '#' {
            return (*current, u_d);
        } else {
            return ((x, y), d);
        }
    }
    fn in_block(&self, x: &i32) -> i32 {
        if x < &0 {
            return -1;
        }
        return x / self.size;
    }
}

fn main() {
    let input = read_to_string("src/data").unwrap();
    let mut iter = input.split("\n\n");
    let map = iter.next().unwrap();
    let inc = iter.next().unwrap();
    let height = map.lines().count() as i32;
    let width = map.lines().map(|l| l.chars().count()).max().unwrap() as i32;
    let size = width / 3;
    println!("{} x {}", width, height);
    dbg!(width, height);

    let mut dict = HashMap::new();
    for (y, l) in map.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            dict.insert((x as i32, y as i32), c);
        }
    }

    // for y in 0..height {
    //     for x in 0..width {
    //         let c = dict.get(&(x, y));
    //         if c.is_none() {
    //             dict.insert((x, y), ' ');
    //         }
    //     }
    // }
    let puzzle = Puzzle {
        width,
        height,
        field: dict,
        size,
    };

    let mut current = (size, 0);
    let mut direction = 0;

    // current = peak(&current, &direction, &puzzle);
    // dbg!(current);
    // println!("{:?}", current);

    let inc = &inc.replace("R", " R ");
    let inc = &inc.replace("L", " L ");
    for i in inc.split_whitespace() {
        if let Ok(val) = i.parse::<i32>() {
            // dbg!(val);
            for _i in 0..val {
                (current, direction) = puzzle.peak2(&current, direction);
            }
        } else {
            // dbg!(i);
            if i == "R" {
                direction = (direction + 5) % 4
            }
            if i == "L" {
                direction = (direction + 7) % 4
                // direction = turn_l(&direction)
            }
        }
        // dbg!(i, current, direction);
        // println!("{}, {:?}, {:?}", i, current, direction);
    }
    dbg!(current, direction);
    println!(
        "{}",
        ((current.1 + 1) * 1000) + ((current.0 + 1) * 4) + direction as i32
    )
}
//
// fn peak(current: &(i32, i32), direction: &(i32, i32), puzzle: &Puzzle) -> (i32, i32) {
//     let mut next = current.clone();
//     next.0 += direction.0;
//     next.1 += direction.1;
//     if next.0 == puzzle.width {
//         next.0 = 0;
//     }
//     if next.0 == -1 {
//         next.0 = puzzle.width - 1;
//     }
//     if next.1 == puzzle.height {
//         next.1 = 0;
//     }
//     if next.1 == -1 {
//         next.1 = puzzle.height - 1;
//     }
//     // dbg!(next);
//     while puzzle.field.get(&next).unwrap() == &' ' {
//         next.0 += direction.0;
//         next.1 += direction.1;
//         if next.0 == puzzle.width {
//             next.0 = 0;
//         }
//         if next.0 == -1 {
//             next.0 = puzzle.width - 1;
//         }
//         if next.1 == -1 {
//             next.1 = puzzle.height - 1;
//         }
//         if next.1 == puzzle.height {
//             next.1 = 0;
//         }
//     }
//     let c = puzzle.field.get(&next).unwrap();
//     if *c == '#' {
//         return *current;
//     }
//     return next;
// }
// fn turn_l(direction: &(i32, i32)) -> (i32, i32) {
//     let x = direction.0;
//     let y = direction.1;
//     if x == 1 {
//         return (0, -1);
//     }
//     if y == 1 {
//         return (1, 0);
//     }
//     if y == -1 {
//         return (-1, 0);
//     }
//     if x == -1 {
//         return (0, 1);
//     }
//     unreachable!();
// }
// fn turn_r(direction: &(i32, i32)) -> (i32, i32) {
//     let x = direction.0;
//     let y = direction.1;
//     if x == 1 {
//         return (0, 1);
//     }
//     if y == 1 {
//         return (-1, 0);
//     }
//     if y == -1 {
//         return (1, 0);
//     }
//     if x == -1 {
//         return (0, -1);
//     }
//     unreachable!();
// }
//
// fn turn_v(direction: &(i32, i32)) -> i32 {
//     let x = direction.0;
//     let y = direction.1;
//     if x == 1 {
//         return 0;
//     }
//     if y == 1 {
//         return 1;
//     }
//     if y == -1 {
//         return 3;
//     }
//     if x == -1 {
//         return 2;
//     }
//     unreachable!();
// }
