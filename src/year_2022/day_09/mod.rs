use std::collections::{HashMap, HashSet};

mod inputs;

pub fn run() {
    println!("--------------------------");
    println!("Advent of Code 2022 Day 09");
    println!("--------------------------\n");

    let result_test_first = process_input_first(inputs::TEST);
    let result_test_second = process_input_second(inputs::TEST);
    println!(
        "test:   first half: {:?}, second half: {:?}",
        result_test_first, result_test_second
    );

    let result_actual_first = process_input_first(inputs::ACTUAL);
    let result_actual_second = process_input_second(inputs::ACTUAL);
    println!(
        "actual: first half: {:?}, second half: {:?}\n",
        result_actual_first, result_actual_second
    );
}

fn process_input_first(raw: &str) -> usize {
    let grid: Vec<Vec<u32>> = raw.split("\n").map(|s| {
        s.chars().into_iter().map(|i| {
            i.to_string().parse::<u32>().unwrap()
        }).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    let mut visible_trees: HashSet<(usize,usize)> = HashSet::new();

    // view from left
    for y in 0..grid.len() {

        let mut tallest_tree = grid[y][0];

        for x in 0..grid[0].len() {
            if x == 0 
            || x == (grid[0].len() - 1)
            || y == 0
            || y == (grid.len() - 1) {
                visible_trees.insert((x,y));
                continue;
            }

            if grid[y][x] > tallest_tree {
                visible_trees.insert((x,y));
                tallest_tree = grid[y][x];
            }

        }

    }

    // view from right
    for y in (0..grid.len()).rev() {

        let mut tallest_tree = grid[y][grid.len() - 1];

        for x in (0..grid[0].len()).rev() {
            let tree = grid[y][x];

            if x == 0 
            || x == (grid[0].len() - 1)
            || y == 0
            || y == (grid.len() - 1) {
                visible_trees.insert((x,y));
                continue;
            }

            if tree > tallest_tree {
                visible_trees.insert((x,y));
                tallest_tree = tree;
            }

        }

    }

    // view from top
    for x in 0..(grid[0].len() - 1) {

        let mut tallest_tree = grid[0][x];

        for y in 0..grid.len() {
            if x == 0 
            || x == (grid[0].len() - 1)
            || y == 0
            || y == (grid.len() - 1) {
                visible_trees.insert((x,y));
                continue;
            }

            if grid[y][x] > tallest_tree {
                visible_trees.insert((x,y));
                tallest_tree = grid[y][x];
            }

        }

    }

    // view from bottom
    for x in (0..grid[0].len()).rev() {

        let mut tallest_tree = grid[grid.len()-1][x];

        for y in (0..grid.len()).rev() {
            if x == 0 
            || x == (grid[0].len() - 1)
            || y == 0
            || y == (grid.len() - 1) {
                visible_trees.insert((x,y));
                continue;
            }
            if grid[y][x] > tallest_tree {
                visible_trees.insert((x,y));
                tallest_tree = grid[y][x];
            }

        }

    }

    visible_trees.len()
}

fn process_input_second(raw: &str) -> usize {
    let grid: Vec<Vec<u32>> = raw.split("\n").map(|s| {
        s.chars().into_iter().map(|i| {
            i.to_string().parse::<u32>().unwrap()
        }).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    let mut scenic_score: Vec<Vec<usize>> = Vec::new();

    // view from left
    for y in 0..grid.len() {
        scenic_score.push(Vec::new());

        let mut trees: HashMap<usize, Vec<usize>> = HashMap::new();

        for x in 0..grid[0].len() {
            scenic_score[y].push(1);

            let taller_tree = trees.keys().filter(|&key| {
                key >= &grid[y][x].try_into().unwrap()
            }).map(|key| {
                trees[key].last().unwrap()
            }).max();

            let scenic_left = match taller_tree {
                Some(tree) => x - tree,
                None => x,
            };

            scenic_score[y][x] *= scenic_left;

            trees.entry(grid[y][x].try_into().unwrap()).and_modify(|positions| {
                positions.push(x)
            }).or_insert(vec![x]);

        }
    }

    // view from right
    for y in 0..grid.len() {

        let mut trees: HashMap<usize, Vec<usize>> = HashMap::new();

        for x in (0..grid[0].len()).rev() {

            let taller_tree = trees.keys().filter(|&key| {
                key >= &grid[y][x].try_into().unwrap()
            }).map(|key| {
                trees[key].last().unwrap()
            }).min();

            let scenic_right = match taller_tree {
                Some(tree) => tree - x,
                None => (grid[y].len() - 1) - x,
            };

            scenic_score[y][x] *= scenic_right;

            trees.entry(grid[y][x].try_into().unwrap()).and_modify(|positions| {
                positions.push(x)
            }).or_insert(vec![x]);

        }
    }


    // view from top
    for x in 0..(grid[0].len()) {

        let mut trees: HashMap<usize, Vec<usize>> = HashMap::new();

        for y in 0..grid.len() {

            let taller_tree = trees.keys().filter(|&key| {
                key >= &grid[y][x].try_into().unwrap()
            }).map(|key| {
                trees[key].last().unwrap()
            }).max();

            let scenic_up = match taller_tree {
                Some(tree) => y - tree,
                None => y,
            };

            scenic_score[y][x] *= scenic_up;

            trees.entry(grid[y][x].try_into().unwrap()).and_modify(|positions| {
                positions.push(y)
            }).or_insert(vec![y]);

        }
    }

    // view from bottom
    for x in 0..(grid[0].len()) {

        let mut trees: HashMap<usize, Vec<usize>> = HashMap::new();

        for y in (0..grid.len()).rev() {

            let taller_tree = trees.keys().filter(|&key| {
                key >= &grid[y][x].try_into().unwrap()
            }).map(|key| {
                trees[key].last().unwrap()
            }).min();

            let scenic_down = match taller_tree {
                Some(tree) => tree - y,
                None => (grid.len() - 1) - y,
            };

            scenic_score[y][x] *= scenic_down;

            trees.entry(grid[y][x].try_into().unwrap()).and_modify(|positions| {
                positions.push(y)
            }).or_insert(vec![y]);

        }
    }

    *scenic_score.iter().map(|row| {
        row.iter().max().unwrap()
    }).max().unwrap()
}
