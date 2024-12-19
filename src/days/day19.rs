use std::error::Error;

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

    while let Some(position) = start_index.pop() {
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
                }
                start_index.push(position + sub_pattern.len());
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

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
