use std::error::Error;

use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day5.input");

fn get_input() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut parts = INPUT.trim().split("\r\n\r\n");
    let ordering_rules = parts
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            line.split('|')
                .map(|num| num.parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()
                .unwrap()
        })
        .collect::<Vec<Vec<usize>>>();
    let update_rules = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()
                .unwrap()
        })
        .collect::<Vec<Vec<usize>>>();
    (ordering_rules, update_rules)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (ordering_rules, updates) = get_input();
    let mut total_page = 0;

    'update: for update in updates.iter() {
        for ordering in ordering_rules.iter() {
            // For some reasons, it kinda worked with the whole tuple
            if let Some((before, _)) = update.iter().find_position(|&elem| *elem == ordering[0]) {
                if let Some((after, _)) = update.iter().find_position(|&elem| *elem == ordering[1])
                {
                    if before > after {
                        continue 'update;
                    }
                }
            }
        }
        total_page += update[update.len() / 2];
    }

    println!(
        "The sum of middle page value from every correctly ordered update is {}",
        total_page
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (ordering_rules, mut updates) = get_input();
    let mut total_page = 0;

    for update in updates.iter_mut() {
        let mut was_unordered = false;
        loop {
            let mut reorder = false;
            for ordering in ordering_rules.iter() {
                if let Some((before, _)) = update.iter().find_position(|&elem| *elem == ordering[0])
                {
                    if let Some((after, _)) =
                        update.iter().find_position(|&elem| *elem == ordering[1])
                    {
                        if before > after {
                            was_unordered = true;
                            reorder = true;
                            update[before] = ordering[1];
                            update[after] = ordering[0];
                        }
                    }
                }
            }
            if !reorder {
                break;
            }
        }
        if was_unordered {
            total_page += update[update.len() / 2];
        }
    }
    println!(
        "The sum of middle page value from every incorrectly ordered update after reordering is {}",
        total_page
    );
    Ok(())
}
