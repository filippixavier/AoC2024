use std::{collections::HashMap, collections::HashSet, error::Error};

const INPUT: &str = include_str!("../../input/day19.input");

fn get_input() -> (Vec<String>, Vec<String>) {
    let mut parts = INPUT.trim().split("\r\n\r\n");

    let towels = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|elem| elem.to_string())
        .collect();
    let patterns = parts
        .next()
        .unwrap()
        .lines()
        .map(|pattern| pattern.to_string())
        .collect();
    (towels, patterns)
}

fn is_possible(pattern: String, towels: &[String], max_towel_size: usize) -> bool {
    let mut start_index: Vec<usize> = vec![0];

    let mut covered: HashSet<usize> = HashSet::new();

    while let Some(position) = start_index.pop() {
        if !covered.insert(position) {
            continue;
        }
        let sub_patterns = (0..max_towel_size)
            .filter_map(|step| {
                if step + position < pattern.len() {
                    Some(pattern[position..=position + step].to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for sub_pattern in sub_patterns {
            if towels.contains(&sub_pattern) {
                if position + sub_pattern.len() == pattern.len() {
                    return true;
                } else {
                    start_index.push(position + sub_pattern.len());
                }
            }
        }
    }

    false
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (towels, patterns) = get_input();
    let max_stripes = towels
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();

    let mut possible_counts = 0;

    for pattern in patterns {
        if is_possible(pattern, &towels, max_stripes) {
            possible_counts += 1;
        }
    }

    println!("There are {} designs that are possible", possible_counts);

    Ok(())
}

fn recurse_possible(
    pattern: &String,
    position: usize,
    towels: &[String],
    max_towel_size: usize,
    memoize: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(memoized) = memoize.get(&position) {
        return *memoized;
    }

    let mut count = 0;
    let sub_patterns = (0..max_towel_size)
        .filter_map(|step| {
            if step + position < pattern.len() {
                Some(pattern[position..=position + step].to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    for sub_pattern in sub_patterns {
        if towels.contains(&sub_pattern) {
            if position + sub_pattern.len() == pattern.len() {
                count += 1;
            } else {
                count += recurse_possible(
                    pattern,
                    position + sub_pattern.len(),
                    towels,
                    max_towel_size,
                    memoize,
                );
            }
        }
    }

    memoize.insert(position, count);
    count
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (towels, patterns) = get_input();
    let max_stripes = towels
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();

    let mut possible_counts = 0;

    for pattern in patterns {
        possible_counts += recurse_possible(&pattern, 0, &towels, max_stripes, &mut HashMap::new());
    }

    println!(
        "There are {} designs that are possible, all permutations included",
        possible_counts
    );

    Ok(())
}
