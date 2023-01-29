use std::{collections::HashMap, fs::read_to_string};
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Bots {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Bots {
    fn new() -> Self {
        return Bots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Resource {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}
impl Resource {
    fn new() -> Self {
        return Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
    }
    fn add(&mut self, bot: &Bots) {
        self.ore += bot.ore;
        self.clay += bot.clay;
        self.obsidian += bot.obsidian;
        self.geode += bot.geode;
    }
    fn compare(&self, other: Self) -> bool {
        if self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian {
            return true;
        }
        return false;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct BluePrint {
    idx: usize,
    ore_bot: Resource,
    clay_bot: Resource,
    obsidian_bot: Resource,
    geode_bot: Resource,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct State {
    bots: Bots,
    resource: Resource,
    turn: usize,
}
impl State {
    fn turn(&mut self) {
        self.resource.add(&self.bots);
        self.turn += 1;
    }
    fn cache(
        &self,
    ) -> (
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    ) {
        return (
            self.turn,
            self.bots.ore,
            self.bots.clay,
            self.bots.geode,
            self.bots.obsidian,
            self.resource.ore,
            self.resource.clay,
            self.resource.geode,
            self.resource.obsidian,
        );
    }
    fn build_ore(&mut self, blue: &BluePrint) {
        self.resource.ore -= blue.ore_bot.ore;
        self.bots.ore += 1;
    }
    fn build_clay(&mut self, blue: &BluePrint) {
        self.resource.ore -= blue.clay_bot.ore;
        self.bots.clay += 1;
    }
    fn build_ob(&mut self, blue: &BluePrint) {
        self.resource.ore -= blue.obsidian_bot.ore;
        self.resource.clay -= blue.obsidian_bot.clay;
        self.bots.obsidian += 1;
        // println!("{:?}", s);
    }
    fn build_geode(&mut self, blue: &BluePrint) {
        self.resource.ore -= blue.geode_bot.ore;
        self.resource.obsidian -= blue.geode_bot.obsidian;
        self.bots.geode += 1;
    }
}

fn time_pass(
    s: &mut State,
    blue: &BluePrint,
    mut max: usize,
    cache: &mut HashMap<
        (
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
        ),
        bool,
    >,
) -> usize {
    // if s.turn == 22 {
    //     println!("+{:?}", s);
    // }
    let resource = s.resource;
    s.turn();
    // println!("-{:?}", s);
    // println!("");
    // if s.resource.ore > 10 {
    //     return ();
    // }
    if s.turn == 32 || resource.ore > 10 || cache.get(&s.cache()).is_some()
    // || s.resource.ore > 40 || s.resource.clay > 30 || s.resource.obsidian > 30
    // || s.resource.ore > 10 || s.resource.clay > 10
    {
        // println!("{:?}", s);
        if s.resource.geode > max {
            max = s.resource.geode;
            // println!("{:?}", s);
        }
        return max;
    }
    cache.insert(s.cache(), true);

    if resource.compare(blue.geode_bot) {
        let mut s_copy = s.clone();
        // println!("+{:?}", s_copy);
        s_copy.build_geode(blue);
        // println!("-{:?}", s_copy);
        // println!("");
        max = time_pass(&mut s_copy, blue, max, cache);
        return max;
    }

    if resource.compare(blue.obsidian_bot) {
        let mut s_copy = s.clone();
        // println!("+{:?}", s_copy);
        s_copy.build_ob(blue);
        // println!("-{:?}", s_copy);
        // println!("");
        max = time_pass(&mut s_copy, blue, max, cache);
    }

    if resource.compare(blue.clay_bot) {
        // println!("{:?}", blue);
        let mut s_copy = s.clone();
        s_copy.build_clay(blue);
        max = time_pass(&mut s_copy, blue, max, cache);
    }

    // if s.turn == 4 {
    //     println!("{:?}", s);
    // }
    if resource.compare(blue.ore_bot) {
        let mut s_copy = s.clone();
        s_copy.build_ore(blue);
        max = time_pass(&mut s_copy, blue, max, cache);
    }

    max = time_pass(s, blue, max, cache);
    return max;
}

fn main() {
    let input = read_to_string("src/data").unwrap();
    let mut blues = Vec::new();
    for l in input.lines() {
        let mut iter = l.split_whitespace();
        let idx: usize = iter.nth(1).unwrap().trim_end_matches(":").parse().unwrap();
        let ore_ore: usize = iter.nth(4).unwrap().parse().unwrap();
        let clay_ore: usize = iter.nth(5).unwrap().parse().unwrap();
        let ob_ore: usize = iter.nth(5).unwrap().parse().unwrap();
        let ob_clay: usize = iter.nth(2).unwrap().parse().unwrap();
        let geode_ore: usize = iter.nth(5).unwrap().parse().unwrap();
        let geode_ob: usize = iter.nth(2).unwrap().parse().unwrap();
        let blue_print = BluePrint {
            idx,
            ore_bot: Resource {
                ore: ore_ore,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay_bot: Resource {
                ore: clay_ore,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian_bot: Resource {
                ore: ob_ore,
                clay: ob_clay,
                obsidian: 0,
                geode: 0,
            },
            geode_bot: Resource {
                ore: geode_ore,
                clay: 0,
                obsidian: geode_ob,
                geode: 0,
            },
        };
        blues.push(blue_print);
    }
    let mut sum = 0;
    let mut prod = 1;
    for b in blues.iter() {
        // if b.idx > 3 {
        //     break;
        // }
        let mut state = State {
            bots: Bots::new(),
            resource: Resource::new(),
            turn: 0,
        };
        let mut cache = HashMap::new();
        let max = time_pass(&mut state, &b, 0, &mut cache);
        sum += max * b.idx;
        prod *= max;
        println!("idx:{} max:{}", b.idx, max);
        // break;
    }
    println!("{}", sum);
    println!("{}", prod);

    // println!("{:?}:", blues[0].obsidian_bot > blues[1].obsidian_bot);
}
