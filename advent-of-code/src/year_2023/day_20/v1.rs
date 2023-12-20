use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub fn part_1(input: &str) -> Option<usize> {
    let mut modules = BTreeMap::new();
    let mut transmitters: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut conjunctions = Vec::new();

    input.lines().for_each(|l| {
        let mut pieces = l.split_whitespace();
        let mut module = Module::Output;
        let m = pieces.next().unwrap();
        let mut name = m.to_string();
        _ = pieces.next();
        let mut receivers = pieces.map(|m| m.trim_matches(',')).collect_vec();
        if m.starts_with("%") {
            name = m[1..].to_string();
            let mut ff = FlipFlop::new(&name);
            for rx in receivers {
                ff.receiver(rx);
                transmitters
                    .entry(rx.to_string())
                    .and_modify(|v| {
                        v.push(name.to_owned());
                    })
                    .or_insert(vec![name.to_owned()]);
            }

            module = Module::FlipFlop(ff);
        } else if m.starts_with("&") {
            name = m[1..].to_string();
            conjunctions.push(name.to_owned());
            let mut c = Conjunction::new(&name);
            for rx in receivers {
                c.receiver(rx);
                transmitters
                    .entry(rx.to_string())
                    .and_modify(|v| {
                        v.push(name.to_owned());
                    })
                    .or_insert(vec![name.to_owned()]);
            }

            module = Module::Conjunction(c);
        } else if m == "broadcaster" {
            let mut b = Broadcast::new();
            for rx in receivers {
                b.receiver(rx);
                transmitters
                    .entry(rx.to_string())
                    .and_modify(|v| {
                        v.push(name.to_owned());
                    })
                    .or_insert(vec![name.to_owned()]);
            }
            module = Module::Broadcast(b);
        } else {
            name = "output".to_string();
            module = Module::Output;
        }

        modules.insert(name, module);
    });

    for c in conjunctions {
        if let Some(Module::Conjunction(cj)) = modules.get_mut(&c) {
            for t in transmitters.get(&c)? {
                cj.transmitter(t);
            }
        }
    }

    for m in transmitters.keys() {
        if modules.get(m).is_none() {
            modules.insert(m.to_string(), Module::Output);
        }
    }

    let mut tx = VecDeque::new();
    let (mut tl, mut th) = (0, 0);
    for l in 1..=1000 {
        tx.push_back(("button".to_string(), "broadcaster".to_string(), false));
        let (mut lows, mut highs) = (0, 0);
        while !tx.is_empty() {
            let (t, r, s) = tx.pop_front()?;
            if s {
                highs += 1;
            } else {
                lows += 1;
            }
            for new_tx in modules.get_mut(&r).unwrap().receive(&t, s) {
                tx.push_back(new_tx);
            }
        }

        tl += lows;
        th += highs;
    }

    Some(tl * th)
}

pub fn part_2(input: &str) -> Option<usize> {
    let mut modules = BTreeMap::new();
    let mut transmitters: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut conjunctions = Vec::new();

    input.lines().for_each(|l| {
        let mut pieces = l.split_whitespace();
        let mut module = Module::Output;
        let m = pieces.next().unwrap();
        let mut name = m.to_string();
        _ = pieces.next();
        let mut receivers = pieces.map(|m| m.trim_matches(',')).collect_vec();
        if m.starts_with("%") {
            name = m[1..].to_string();
            let mut ff = FlipFlop::new(&name);
            for rx in receivers {
                ff.receiver(rx);
                transmitters
                    .entry(rx.to_string())
                    .and_modify(|v| {
                        v.push(name.to_owned());
                    })
                    .or_insert(vec![name.to_owned()]);
            }
            module = Module::FlipFlop(ff);
        } else if m.starts_with("&") {
            name = m[1..].to_string();
            conjunctions.push(name.to_owned());
            let mut c = Conjunction::new(&name);
            for rx in receivers {
                c.receiver(rx);
                transmitters
                    .entry(rx.to_string())
                    .and_modify(|v| {
                        v.push(name.to_owned());
                    })
                    .or_insert(vec![name.to_owned()]);
            }
            module = Module::Conjunction(c);
        } else if m == "broadcaster" {
            let mut b = Broadcast::new();
            for rx in receivers {
                b.receiver(rx);
                transmitters
                    .entry(rx.to_string())
                    .and_modify(|v| {
                        v.push(name.to_owned());
                    })
                    .or_insert(vec![name.to_owned()]);
            }
            module = Module::Broadcast(b);
        } else {
            name = "output".to_string();
            module = Module::Output;
        }

        modules.insert(name, module);
    });

    for c in &conjunctions {
        if let Some(Module::Conjunction(cj)) = modules.get_mut(c) {
            for t in transmitters.get(c)? {
                cj.transmitter(t);
            }
        }
    }

    for m in transmitters.keys() {
        if modules.get(m).is_none() {
            modules.insert(m.to_string(), Module::Output);
        }
    }

    let mut cj_true = BTreeMap::new();

    let mut tx = VecDeque::new();
    for bp in 1..=1000000 {
        tx.push_back(("button".to_string(), "broadcaster".to_string(), false));
        while !tx.is_empty() {
            let (t, r, s) = tx.pop_front()?;
            if !s && conjunctions.contains(&t) && !cj_true.contains_key(&t) {
                cj_true.insert(t.to_owned(), bp);
                if cj_true.len() == conjunctions.len() - 1 {
                    return Some(cj_true.values().product());
                }
            }
            for new_tx in modules.get_mut(&r).unwrap().receive(&t, s) {
                tx.push_back(new_tx);
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
struct FlipFlop {
    rx: BTreeSet<String>,
    state: bool,
    name: String,
}

impl FlipFlop {
    fn new(name: &str) -> Self {
        Self {
            rx: BTreeSet::new(),
            state: false,
            name: name.to_owned(),
        }
    }

    fn is_default(&self) -> bool {
        !self.state
    }

    fn receiver(&mut self, m: &str) {
        self.rx.insert(m.to_string());
    }

    fn receive(&mut self, signal: bool) -> Vec<(String, String, bool)> {
        let mut r = Vec::new();
        if !signal {
            self.state = !self.state;
            for rx in &self.rx {
                r.push((self.name.to_owned(), rx.to_owned(), self.state));
            }
        }
        r
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    rx: BTreeSet<String>,
    tx: BTreeMap<String, bool>,
    state: bool,
    name: String,
}

impl Conjunction {
    fn new(name: &str) -> Self {
        Self {
            rx: BTreeSet::new(),
            tx: BTreeMap::new(),
            state: false,
            name: name.to_owned(),
        }
    }

    fn is_default(&self) -> bool {
        !self.state && self.tx.values().filter(|v| !**v).count() == self.tx.len()
    }

    fn receiver(&mut self, m: &str) {
        self.rx.insert(m.to_string());
    }

    fn transmitter(&mut self, m: &str) {
        self.tx.insert(m.to_string(), false);
    }

    fn receive(&mut self, transmitter: &str, signal: bool) -> Vec<(String, String, bool)> {
        let mut r = Vec::new();
        self.tx.insert(transmitter.to_string(), signal);
        let s = self.tx.values().filter(|v| **v).count() == self.tx.len();
        self.state = s;

        for rx in &self.rx {
            r.push((self.name.to_owned(), rx.to_owned(), !self.state));
        }

        r
    }
}

#[derive(Debug, Clone)]
struct Broadcast {
    rx: BTreeSet<String>,
}

impl Broadcast {
    fn new() -> Self {
        Self {
            rx: BTreeSet::new(),
        }
    }

    fn receiver(&mut self, m: &str) {
        self.rx.insert(m.to_string());
    }

    fn broadcast(&self) -> Vec<(String, String, bool)> {
        let mut r = Vec::new();
        for rx in &self.rx {
            r.push(("broadcaster".to_string(), rx.to_owned(), false));
        }
        r
    }
}

#[derive(Debug, Clone)]
enum Module {
    Broadcast(Broadcast),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Output,
}

impl Module {
    fn receive(&mut self, transmitter: &str, signal: bool) -> Vec<(String, String, bool)> {
        match self {
            Module::Output => Vec::new(),
            Module::Broadcast(m) => m.broadcast(),
            Module::FlipFlop(m) => m.receive(signal),
            Module::Conjunction(m) => m.receive(transmitter, signal),
        }
    }
    fn is_default(&self) -> bool {
        match self {
            Module::Output => true,
            Module::Broadcast(m) => true,
            Module::FlipFlop(m) => m.is_default(),
            Module::Conjunction(m) => m.is_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::year_2023::Data;

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_20_part_1")
            .expect("src/year_2023/day_20_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(925_955_316));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_20_part_2")
            .expect("src/year_2023/day_20_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(241_528_477_694_627));
    }
}
