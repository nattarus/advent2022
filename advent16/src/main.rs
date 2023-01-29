use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    rate: i32,
    connect: Vec<String>,
    open: i32,
    cost: i32,
}

struct Valves {
    data: HashMap<String, Valve>,
    sum: i32,
}

impl Valves {
    fn new(data: HashMap<String, Valve>) {}
    fn open(&mut self, key: &str) {
        let v = self.data.get_mut(key).unwrap();
        v.open += 1;
    }
    fn close(&mut self, key: &str) {
        let v = self.data.get_mut(key).unwrap();
        v.open -= 1;
    }
    fn sum(&self) -> i32 {
        let sum = self
            .data
            .iter()
            .filter(|v| v.1.open > 0)
            .map(|v| v.1.rate)
            .sum::<i32>();
        // println!("{}", sum);
        return sum;
    }
    fn all_sum(&self) -> i32 {
        let sum = self.data.iter().map(|v| v.1.rate).sum::<i32>();
        // println!("{}", sum);
        return sum;
    }
    fn is_close(&self, key: &str) -> bool {
        let v = self.data.get(key).unwrap();
        return v.open == 0;
    }
    fn is_all_open(&self) -> bool {
        return self
            .data
            .iter()
            .filter(|v| v.1.rate > 0)
            .all(|a| a.1.open > 0);
    }
}

fn main() {
    let input = read_to_string("src/data").unwrap();
    let mut valve_map: HashMap<String, Valve> = HashMap::new();
    let mut valves = Vec::new();

    for l in input.lines() {
        let mut iter = l.split_whitespace();
        let name = iter.nth(1).unwrap().to_string();
        let rate: i32 = iter
            .nth(2)
            .unwrap()
            .trim_start_matches("rate=")
            .trim_end_matches(";")
            .parse()
            .unwrap();
        iter.nth(3);
        let lead_to: Vec<String> = iter.map(|a| a.trim_end_matches(",").to_string()).collect();

        let valve = Valve {
            name: name.clone(),
            rate,
            connect: lead_to.clone(),
            open: 0,
            cost: i32::MAX,
        };
        // println!("{:?}", valve.connect);
        valves.push(valve.clone());
        if valve.rate > 0 || valve.name == "AA" {
            valve_map.insert(name.clone(), valve);
        }
    }

    // let mut path = Vec::new();
    // let mut opened_v = Vec::new();
    // let mut opened_f = Vec::new();
    // let max = action(
    //     &mut path,
    //     &mut opened_v,
    //     &mut opened_f,
    //     "none",
    //     1,
    //     "AA",
    //     &mut valve_map,
    //     0,
    //     0,
    // );
    // println!("{}", max)
    // let valves = find_cost("AA", &mut valves);

    let mut time = 1;
    let mut valves_map: HashMap<String, Vec<Valve>> = HashMap::new();
    for v in valves.clone() {
        if v.rate > 0 || v.name == "AA" {
            let valves_cost = find_cost(&v.name, valves.clone())
                .iter()
                .filter(|v| v.rate > 0 || v.name == "AA")
                .map(|v| v.clone())
                .collect();
            valves_map.insert(v.name, valves_cost);
        }
    }

    let mut current_valve = "AA";
    let e = Person {
        to: "AA".to_string(),
        turn: 0,
    };
    let me = Person {
        to: "AA".to_string(),
        turn: 0,
    };
    let sum = valve_map.iter().map(|v| v.1.rate).sum::<i32>();
    let mut valves = Valves {
        data: valve_map,
        sum,
    };

    walking2(0, "AA", "AA", 0, 0, &mut valves, &valves_map, 26, 0, 0);
}

struct Person {
    to: String,
    turn: i32,
}
impl Person {
    fn tick(&self, turn: i32) -> Self {
        return Self {
            to: self.to.to_string(),
            turn: self.turn - turn,
        };
    }
}

fn walking2(
    turn: i32,
    mut me_to: &str,
    mut e_to: &str,
    me_turn: i32,
    e_turn: i32,
    valve_vec: &mut Valves,
    valves_map: &HashMap<String, Vec<Valve>>,
    time_left: i32,
    mut max: i32,
    flow: i32,
) -> i32 {
    // if me.turn == 0 {
    //     if me.to != "AA" {
    //         valve_vec.open(&me.to);
    //     }
    // } else if e.turn == 0 {
    //     if e.to != "AA" {
    //         valve_vec.open(&e.to);
    //     }
    // }

    if time_left <= 0 || valve_vec.is_all_open() || ((valve_vec.sum * time_left) + flow < max) {
        let flow = flow + (valve_vec.sum() * time_left);

        if flow > max {
            println!("{}", flow);
            max = flow
        }
        return max;
    }

    if me_turn == 0 {
        let valves = valves_map.get(me_to).unwrap();
        for t in valves.iter() {
            if t.cost > 0 && valve_vec.is_close(&t.name)
            // && t.name != e_to
            // && t.name != me_to
            {
                let next_turn = e_turn.min(t.cost + 1);
                let next_valve = if e_turn > t.cost + 1 { &t.name } else { e_to };
                let flow = flow + (valve_vec.sum() * next_turn);
                valve_vec.open(next_valve);

                max = walking2(
                    next_turn,
                    &t.name,
                    e_to,
                    t.cost + 1 - next_turn,
                    e_turn - next_turn,
                    valve_vec,
                    valves_map,
                    time_left - next_turn,
                    max,
                    flow,
                );
                valve_vec.close(next_valve);
            }
        }
    }
    if e_turn == 0 {
        let valves = valves_map.get(e_to).unwrap();
        for t in valves.iter() {
            if t.cost > 0 && valve_vec.is_close(&t.name)
            // && t.name != e_to
            // && t.name != me_to
            {
                // println!("{}", t.cost);
                let next_turn = me_turn.min(t.cost + 1);
                let next_valve = if me_turn <= t.cost + 1 {
                    me_to
                } else {
                    &t.name
                };
                let flow = flow + (valve_vec.sum() * next_turn);
                valve_vec.open(&next_valve);

                max = walking2(
                    next_turn,
                    me_to,
                    &t.name,
                    me_turn - next_turn,
                    t.cost + 1 - next_turn,
                    valve_vec,
                    valves_map,
                    time_left - next_turn,
                    max,
                    flow,
                );
                valve_vec.close(&next_valve);
            }
        }
    }
    // println!("e");

    return max;
}

// fn walking(
//     me_to: &str,
//     e_to: &str,
//     me_turn: i32,
//     e_turn: i32,
//     me_rate: i32,
//     e_rate: i32,
//     valves_map: &mut HashMap<String, Vec<Valve>>,
//     time: i32,
//     mut max: i32,
//     flow: i32,
//     valve_f: &mut Vec<i32>,
//     valve_v: &mut Vec<String>,
// ) -> i32 {
//     // if me_turn == 0 && me_to != "AA" {
//     //     valve_f.push(me_rate);
//     // }
//     // if e_turn == 0  && e_to != "AA"{
//     //     valve_f.push(e_rate);
//     // }
//
//     let valves = valves_map.get(me_to).unwrap();
//     if valves
//         .iter()
//         .filter(|v| v.rate > 0)
//         .collect::<Vec<&Valve>>()
//         .len()
//         == valve_v.len()
//     {
//         let turn_left = 26 - time;
//         let mut flow: i32 = flow
//             + (valve_f
//                 .iter()
//                 .filter(|a| **a != e_rate && **a != me_rate)
//                 .sum::<i32>()
//                 * (turn_left));
//         // println!(
//         //     "{:?}",
//         //     valve_f
//         //         .iter()
//         //         .filter(|a| **a != e_rate && **a != me_rate)
//         //         .collect::<Vec<&i32>>()
//         // );
//         if me_turn == 0 {
//             flow = flow + (me_rate * turn_left);
//             if 0 < turn_left - e_turn {
//                 flow = flow + (turn_left - e_turn) * e_rate;
//             }
//         }
//         if e_turn == 0 {
//             flow = flow + (e_rate * turn_left);
//             if 0 < turn_left - me_turn {
//                 flow = flow + (turn_left - me_turn) * me_rate;
//             }
//         }
//
//         if flow > max {
//             // println!("{}", valve_v.len());
//             // println!("{:?}", valve_v);
//             // println!("{}", time);
//             println!("deplete {:?} {} {}", valve_v, flow, time);
//             println!("{:?} ", valve_f);
//         }
//         max = flow.max(max);
//         // println!("{}", max);
//         // println!("{}", time);
//         return max;
//     }
//
//     let mut valves = valves_map.get(e_to).unwrap();
//     if me_turn == 0 {
//         valves = valves_map.get(me_to).unwrap();
//     }
//     // println!("me turn");
//     for t in valves.clone().iter() {
//         if t.rate > 0 {
//             let cost = t.cost + 1;
//
//             if time + cost > 26 {
//                 let flow: i32 = flow + (valve_f.iter().sum::<i32>() * (26 - time));
//                 if flow > max {
//                     println!("{:?} {}", valve_v, flow);
//                     println!("{:?} ", valve_f);
//                 }
//                 max = flow.max(max);
//                 // println!("{}", max);
//                 return max;
//             }
//
//             valve_v.push(t.name.to_string());
//             valve_f.push(t.rate);
//
//             if me_turn == 0 {
//                 let next_turn = cost.min(e_turn);
//                 let flow: i32 = flow
//                     + (valve_f
//                         .iter()
//                         .filter(|a| **a != t.rate && **a != e_rate)
//                         .sum::<i32>()
//                         * next_turn);
//                 max = walking(
//                     &t.name,
//                     e_to,
//                     cost - next_turn,
//                     e_turn - next_turn,
//                     t.rate,
//                     e_rate,
//                     valves_map,
//                     time + next_turn,
//                     max,
//                     flow,
//                     valve_f,
//                     valve_v,
//                 );
//                 valve_f.retain(|f| *f != t.rate);
//                 valve_v.retain(|f| *f != t.name);
//             } else if e_turn == 0 {
//                 let next_turn = cost.min(me_turn);
//                 let flow: i32 = flow
//                     + (valve_f
//                         .iter()
//                         .filter(|a| **a != t.rate && **a != me_rate)
//                         .sum::<i32>()
//                         * next_turn);
//                 max = walking(
//                     me_to,
//                     &t.name,
//                     me_turn - next_turn,
//                     cost - next_turn,
//                     me_rate,
//                     t.rate,
//                     valves_map,
//                     time + next_turn,
//                     max,
//                     flow,
//                     valve_f,
//                     valve_v,
//                 );
//                 valve_f.retain(|f| *f != t.rate);
//                 valve_v.retain(|f| *f != t.name);
//             } else {
//                 assert!(false);
//             }
//
//             // valve_f.pop();
//         }
//     }
//
//     return max;
// }

fn find_cost(start: &str, mut valves: Vec<Valve>) -> Vec<Valve> {
    let mut visited = Vec::new();
    for v in valves.iter_mut() {
        v.cost = i32::MAX;
    }
    let mut valve = valves
        .iter()
        .find(|v| v.name == start)
        .expect("T T")
        .clone();
    valve.cost = 0;
    let len = valves.len();
    while len != visited.len() {
        for c in valve.connect.clone() {
            let leaf = valves.iter_mut().find(|v| v.name == c);
            if let Some(c) = leaf {
                c.cost = c.cost.min(valve.cost + 1);
            }
        }
        valves.sort_by(|a, b| b.cost.cmp(&a.cost));
        valve = valves.pop().unwrap();
        visited.push(valve.clone());
    }
    visited.sort_by(|a, b| (b.rate).cmp(&(a.rate)));
    // visited.sort_by(|a, b| (b.rate / b.cost).cmp(&(a.rate / a.cost)));
    // visited.sort_by(|a, b| (a.cost).cmp(&(b.cost)));
    return visited;
}
