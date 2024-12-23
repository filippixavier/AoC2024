use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day23.input");

fn get_input() -> Vec<(String, String)> {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            let split = line.split('-').collect::<Vec<_>>();
            (split[0].to_string(), split[1].to_string())
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let connections = get_input();
    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();

    for link in connections {
        let left = nodes.entry(link.0.clone()).or_default();
        left.push(link.1.clone());
        let right = nodes.entry(link.1).or_default();
        right.push(link.0);
    }

    let mut count = 0;

    let mut visited: HashSet<String> = HashSet::new();

    for (origin, network) in nodes.iter().filter(|(name, _)| name.starts_with('t')) {
        for combination in network.iter().combinations(2) {
            if nodes.get(combination[0]).unwrap().contains(combination[1]) {
                let mut signature = vec![origin, combination[0], combination[1]];
                signature.sort();
                if visited.insert(signature.into_iter().join(",")) {
                    count += 1;
                }
            }
        }
    }

    println!(
        "There are {} networks of length 3 with at least one computer starting with t",
        count
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
