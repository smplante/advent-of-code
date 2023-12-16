use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

pub fn part_1(input: &str) -> Option<usize> {
    let contraption = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();

    traverse(
        &contraption,
        Beam {
            d: Dir::E,
            p: (0, 0),
        },
    )
}

pub fn part_2(input: &str) -> Option<usize> {
    let contraption = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();

    let mut energized_cells = 0;

    for y in 0..contraption.len() {
        energized_cells = energized_cells.max(traverse(
            &contraption,
            Beam {
                d: Dir::E,
                p: (0, y),
            },
        )?);
        energized_cells = energized_cells.max(traverse(
            &contraption,
            Beam {
                d: Dir::W,
                p: (contraption[0].len() - 1, y),
            },
        )?);
    }

    for x in 0..contraption[0].len() {
        energized_cells = energized_cells.max(traverse(
            &contraption,
            Beam {
                d: Dir::S,
                p: (x, 0),
            },
        )?);
        energized_cells = energized_cells.max(traverse(
            &contraption,
            Beam {
                d: Dir::N,
                p: (x, contraption.len() - 1),
            },
        )?);
    }

    Some(energized_cells)
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Beam {
    d: Dir,
    p: (usize, usize),
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}

fn traverse(contraption: &Vec<Vec<u8>>, start: Beam) -> Option<usize> {
    let mut energized_cells: Vec<Vec<u8>> = contraption.clone();
    let mut traversed = HashSet::new();

    let mut beams = VecDeque::new();
    match (start.d, contraption[start.p.1][start.p.0]) {
        (Dir::E, b'\\') | (Dir::W, b'/') | (Dir::S, b'|') => {
            beams.push_back(Beam {
                d: Dir::S,
                p: start.p,
            });
        }
        (Dir::E, b'/') | (Dir::W, b'\\') | (Dir::N, b'|') => {
            beams.push_back(Beam {
                d: Dir::N,
                p: start.p,
            });
        }
        (Dir::E | Dir::W, b'|') => {
            beams.push_back(Beam {
                d: Dir::N,
                p: start.p,
            });
            beams.push_back(Beam {
                d: Dir::S,
                p: start.p,
            });
        }
        (Dir::N, b'\\') | (Dir::S, b'/') | (Dir::W, b'-') => {
            beams.push_back(Beam {
                d: Dir::W,
                p: start.p,
            });
        }
        (Dir::N, b'/') | (Dir::S, b'\\') | (Dir::E, b'-') => {
            beams.push_back(Beam {
                d: Dir::E,
                p: start.p,
            });
        }
        (Dir::N | Dir::S, b'-') => {
            beams.push_back(Beam {
                d: Dir::E,
                p: start.p,
            });
            beams.push_back(Beam {
                d: Dir::W,
                p: start.p,
            });
        }
        _ => {
            beams.push_back(start);
        }
    }

    while !beams.is_empty() {
        let beam = beams.pop_front()?;
        if traversed.contains(&beam) {
            continue;
        }
        traversed.insert(beam);
        energized_cells[beam.p.1][beam.p.0] = b'#';
        let (next_beams, energized) = travel(contraption, beam);
        for b in next_beams {
            beams.push_back(b);
        }
        for e in energized {
            energized_cells[e.1][e.0] = b'#';
        }
    }

    energized_cells
        .iter()
        .map(|row| row.iter().filter(|&c| c == &b'#').count())
        .sum1()
}

fn travel(contraption: &[Vec<u8>], beam: Beam) -> (Vec<Beam>, Vec<(usize, usize)>) {
    let mut energized = Vec::new();
    let mut beams = Vec::new();

    match beam.d {
        Dir::N => {
            for y in (0..beam.p.1).rev() {
                energized.push((beam.p.0, y));
                match contraption[y][beam.p.0] {
                    b'/' => {
                        beams.push(Beam {
                            d: Dir::E,
                            p: (beam.p.0, y),
                        });
                        break;
                    }
                    b'\\' => {
                        beams.push(Beam {
                            d: Dir::W,
                            p: (beam.p.0, y),
                        });
                        break;
                    }
                    b'-' => {
                        beams.push(Beam {
                            d: Dir::E,
                            p: (beam.p.0, y),
                        });
                        beams.push(Beam {
                            d: Dir::W,
                            p: (beam.p.0, y),
                        });
                        break;
                    }
                    _ => {}
                }
            }
        }
        Dir::S => {
            for y in beam.p.1 + 1..contraption.len() {
                energized.push((beam.p.0, y));
                match contraption[y][beam.p.0] {
                    b'/' => {
                        beams.push(Beam {
                            d: Dir::W,
                            p: (beam.p.0, y),
                        });
                        break;
                    }
                    b'\\' => {
                        beams.push(Beam {
                            d: Dir::E,
                            p: (beam.p.0, y),
                        });
                        break;
                    }
                    b'-' => {
                        beams.push(Beam {
                            d: Dir::E,
                            p: (beam.p.0, y),
                        });
                        beams.push(Beam {
                            d: Dir::W,
                            p: (beam.p.0, y),
                        });
                        break;
                    }
                    _ => {}
                }
            }
        }
        Dir::E => {
            for x in beam.p.0 + 1..contraption[0].len() {
                energized.push((x, beam.p.1));
                match contraption[beam.p.1][x] {
                    b'/' => {
                        beams.push(Beam {
                            d: Dir::N,
                            p: (x, beam.p.1),
                        });
                        break;
                    }
                    b'\\' => {
                        beams.push(Beam {
                            d: Dir::S,
                            p: (x, beam.p.1),
                        });
                        break;
                    }
                    b'|' => {
                        beams.push(Beam {
                            d: Dir::N,
                            p: (x, beam.p.1),
                        });
                        beams.push(Beam {
                            d: Dir::S,
                            p: (x, beam.p.1),
                        });
                        break;
                    }
                    _ => {}
                }
            }
        }
        Dir::W => {
            for x in (0..beam.p.0).rev() {
                energized.push((x, beam.p.1));
                match contraption[beam.p.1][x] {
                    b'/' => {
                        beams.push(Beam {
                            d: Dir::S,
                            p: (x, beam.p.1),
                        });
                        break;
                    }
                    b'\\' => {
                        beams.push(Beam {
                            d: Dir::N,
                            p: (x, beam.p.1),
                        });
                        break;
                    }
                    b'|' => {
                        beams.push(Beam {
                            d: Dir::N,
                            p: (x, beam.p.1),
                        });
                        beams.push(Beam {
                            d: Dir::S,
                            p: (x, beam.p.1),
                        });
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    (beams, energized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::year_2023::Data;

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_16_part_1")
            .expect("src/year_2023/day_16_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(7034));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_16_part_2")
            .expect("src/year_2023/day_16_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(7759));
    }
}
