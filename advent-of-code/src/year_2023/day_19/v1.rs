use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::format;

pub fn part_1(input: &str) -> Option<usize> {
    let mut workflows: HashMap<String, Vec<(Option<&str>, Option<usize>, Option<usize>, &str)>> =
        HashMap::new();
    workflows.insert("A".to_string(), vec![(None, None, None, "A")]);
    workflows.insert("R".to_string(), vec![(None, None, None, "R")]);

    let (w, r) = input.split_once("\n\n")?;
    w.lines().for_each(|l| {
        let (wf, rules) = l.split_once('{').unwrap();
        let rules_raw = rules.trim_matches('}').split(',');

        let mut rules = Vec::new();
        for rule in rules_raw {
            if rule.contains(':') {
                let (lhs, to) = rule.split_once(':').unwrap();
                if lhs.contains('>') {
                    let (l, r) = lhs.split_once('>').unwrap();
                    rules.push((Some(l), Some(0), Some(r.parse::<usize>().unwrap()), to));
                    continue;
                }
                if lhs.contains('<') {
                    let (l, r) = lhs.split_once('<').unwrap();
                    rules.push((Some(l), Some(1), Some(r.parse::<usize>().unwrap()), to));
                    continue;
                }
            } else {
                rules.push((None, None, None, rule))
            }
        }

        workflows.insert(wf.to_string(), rules);
    });

    let mut sum = 0;
    r.lines().for_each(|l| {
        let rating =
            l.trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .fold((0, 0, 0, 0), |acc, p| {
                    let (c, v) = p.split_once('=').unwrap();
                    match c {
                        "x" => (acc.0 + v.parse::<usize>().unwrap(), acc.1, acc.2, acc.3),
                        "m" => (acc.0, acc.1 + v.parse::<usize>().unwrap(), acc.2, acc.3),
                        "a" => (acc.0, acc.1, acc.2 + v.parse::<usize>().unwrap(), acc.3),
                        "s" => (acc.0, acc.1, acc.2, acc.3 + v.parse::<usize>().unwrap()),
                        _ => unreachable!(),
                    }
                });

        let mut w = vec![workflows.get("in").unwrap()];

        while !w.is_empty() {
            let rules = w.remove(0);
            'rules: for rule in rules {
                // dbg!(rule);
                match rule {
                    (None, None, None, "A") => {
                        sum += rating.0 + rating.1 + rating.2 + rating.3;
                        break 'rules;
                    }
                    (None, None, None, "R") => {
                        break 'rules;
                    }
                    (None, None, None, rule) => {
                        w.push(workflows.get(*rule).unwrap());
                        break 'rules;
                    }
                    (Some("x"), Some(0), Some(v), rule) => {
                        if rating.0 > *v {
                            w.push(workflows.get(*rule).unwrap());
                            break 'rules;
                        }
                    }
                    (Some("m"), Some(0), Some(v), rule) => {
                        if rating.1 > *v {
                            w.push(workflows.get(*rule).unwrap());
                            break 'rules;
                        }
                    }
                    (Some("a"), Some(0), Some(v), rule) => {
                        if rating.2 > *v {
                            w.push(workflows.get(*rule).unwrap());
                            break 'rules;
                        }
                    }
                    (Some("s"), Some(0), Some(v), rule) => {
                        if rating.3 > *v {
                            w.push(workflows.get(*rule).unwrap());
                            break 'rules;
                        }
                    }
                    (Some("x"), Some(1), Some(v), rule) => {
                        if rating.0 < *v {
                            w.push(workflows.get(*rule).unwrap());
                            break 'rules;
                        }
                    }
                    (Some("m"), Some(1), Some(v), rule) => {
                        if rating.1 < *v {
                            w.push(workflows.get(*rule).unwrap());
                            break 'rules;
                        }
                    }
                    (Some("a"), Some(1), Some(v), rule) => {
                        if rating.2 < *v {
                            w.push(workflows.get(*rule).unwrap());
                            break 'rules;
                        }
                    }
                    (Some("s"), Some(1), Some(v), rule) => {
                        if rating.3 < *v {
                            w.push(workflows.get(*rule).unwrap());
                            break 'rules;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    });

    Some(sum)
}

pub fn part_2(input: &str) -> Option<usize> {
    let mut workflows: HashMap<String, Vec<(Option<&str>, Option<usize>, Option<usize>, &str)>> =
        HashMap::new();
    workflows.insert("A".to_string(), vec![(None, None, None, "A")]);
    workflows.insert("R".to_string(), vec![(None, None, None, "R")]);

    let (w, r) = input.split_once("\n\n")?;
    w.lines().for_each(|l| {
        let (wf, rules) = l.split_once('{').unwrap();
        let rules_raw = rules.trim_matches('}').split(',');

        let mut rules = Vec::new();
        for rule in rules_raw {
            if rule.contains(':') {
                let (lhs, to) = rule.split_once(':').unwrap();
                if lhs.contains('>') {
                    let (l, r) = lhs.split_once('>').unwrap();
                    rules.push((Some(l), Some(0), Some(r.parse::<usize>().unwrap()), to));
                    continue;
                }
                if lhs.contains('<') {
                    let (l, r) = lhs.split_once('<').unwrap();
                    rules.push((Some(l), Some(1), Some(r.parse::<usize>().unwrap()), to));
                    continue;
                }
            } else {
                rules.push((None, None, None, rule))
            }
        }

        workflows.insert(wf.to_string(), rules);
    });

    let sum = flow("".to_string(), "in", &workflows, Limits::new());

    Some(sum)
}

fn flow(
    path: String,
    position: &str,
    workflows: &HashMap<String, Vec<(Option<&str>, Option<usize>, Option<usize>, &str)>>,
    limits: Limits,
) -> usize {
    let rules = workflows.get(position).unwrap();
    let mut limits = limits;
    let mut sum = 0;
    for rule in rules {
        let mut loc_limits = limits;
        match rule {
            (None, None, None, "A") => sum += loc_limits.variations(),
            (Some(var), Some(cmp), Some(val), "A") => {
                let new_loc_limits = loc_limits.constraint(var, *cmp, *val);
                sum += loc_limits.variations();
                limits = new_loc_limits;
            }
            (None, None, None, "R") => {}
            (Some(var), Some(cmp), Some(val), "R") => {
                let new_loc_limits = loc_limits.constraint(var, *cmp, *val);
                limits = new_loc_limits;
            }
            (Some(var), Some(cmp), Some(val), to) => {
                let new_loc_limits = loc_limits.constraint(var, *cmp, *val);
                sum += flow(format!("{path}->{position}"), to, workflows, loc_limits);
                limits = new_loc_limits;
            }
            (None, None, None, to) => {
                sum += flow(format!("{path}->{position}"), to, workflows, loc_limits);
            }
            _ => todo!(),
        };
    }

    sum
}

#[derive(Debug, Copy, Clone)]
struct Limits {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl Limits {
    fn new() -> Self {
        Self {
            x: (0, 4000),
            m: (0, 4000),
            a: (0, 4000),
            s: (0, 4000),
        }
    }
    fn constraint(&mut self, var: &str, cmp: usize, val: usize) -> Self {
        let mut inverse = *self;
        match (var, cmp) {
            ("x", 0) => {
                // >
                self.x.0 = val;
                inverse.x.1 = val;
            }
            ("x", 1) => {
                // <
                self.x.1 = val - 1;
                inverse.x.0 = val - 1;
            }
            ("m", 0) => {
                // >
                self.m.0 = val;
                inverse.m.1 = val;
            }
            ("m", 1) => {
                // <
                self.m.1 = val - 1;
                inverse.m.0 = val - 1;
            }
            ("a", 0) => {
                // >
                self.a.0 = val;
                inverse.a.1 = val;
            }
            ("a", 1) => {
                // <
                self.a.1 = val - 1;
                inverse.a.0 = val - 1;
            }
            ("s", 0) => {
                // >
                self.s.0 = val;
                inverse.s.1 = val;
            }
            ("s", 1) => {
                // <
                self.s.1 = val - 1;
                inverse.s.0 = val - 1;
            }
            _ => unreachable!(),
        };

        inverse
    }

    fn variations(&self) -> usize {
        (self.x.1 - self.x.0)
            * (self.m.1 - self.m.0)
            * (self.a.1 - self.a.0)
            * (self.s.1 - self.s.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::year_2023::Data;

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_19_part_1")
            .expect("src/year_2023/day_19_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(368_964));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_19_part_2")
            .expect("src/year_2023/day_19_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(127_675_188_176_682));
    }
}
