use std::collections::HashSet;
use std::error::Error;

const INPUT: &str = include_str!("../../input/day10.input");
fn get_input() -> Vec<Vec<u32>> {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|num| num.to_digit(10))
                .collect::<Option<Vec<u32>>>()
                .unwrap()
        })
        .collect()
}

fn hike(
    topographic_map: &[Vec<u32>],
    max: (usize, usize),
    current_pos: (usize, usize),
    reached: &mut HashSet<(usize, usize)>,
    use_reached: bool,
) -> usize {
    let mut total = 0;

    let current_height = topographic_map[current_pos.0][current_pos.1];

    if current_height == 9 {
        return if !use_reached || reached.insert(current_pos) {
            1
        } else {
            0
        };
    }

    if current_pos.0 > 0 {
        let next_pos = (current_pos.0 - 1, current_pos.1);
        let next_height = topographic_map[next_pos.0][next_pos.1];
        if current_height + 1 == next_height {
            total += hike(topographic_map, max, next_pos, reached, use_reached);
        }
    }

    if current_pos.0 < max.0 {
        let next_pos = (current_pos.0 + 1, current_pos.1);
        let next_height = topographic_map[next_pos.0][next_pos.1];
        if current_height + 1 == next_height {
            total += hike(topographic_map, max, next_pos, reached, use_reached);
        }
    }

    if current_pos.1 > 0 {
        let next_pos = (current_pos.0, current_pos.1 - 1);
        let next_height = topographic_map[next_pos.0][next_pos.1];
        if current_height + 1 == next_height {
            total += hike(topographic_map, max, next_pos, reached, use_reached);
        }
    }

    if current_pos.1 < max.1 {
        let next_pos = (current_pos.0, current_pos.1 + 1);
        let next_height = topographic_map[next_pos.0][next_pos.1];
        if current_height + 1 == next_height {
            total += hike(topographic_map, max, next_pos, reached, use_reached);
        }
    }

    total
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let topographic_map = get_input();
    let (max_line, max_col) = (topographic_map.len() - 1, topographic_map[0].len() - 1);
    let mut score = 0;

    let starting_points: Vec<(usize, usize)> = topographic_map
        .iter()
        .enumerate()
        .flat_map(|(line_no, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, val)| **val == 0)
                .map(|(col_no, _)| (line_no, col_no))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    for (line, col) in starting_points {
        score += hike(
            &topographic_map,
            (max_line, max_col),
            (line, col),
            &mut HashSet::new(),
            true,
        );
    }

    println!("The sum of score of all trailheads is {}", score);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let topographic_map = get_input();
    let (max_line, max_col) = (topographic_map.len() - 1, topographic_map[0].len() - 1);
    let mut score = 0;

    let starting_points: Vec<(usize, usize)> = topographic_map
        .iter()
        .enumerate()
        .flat_map(|(line_no, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, val)| **val == 0)
                .map(|(col_no, _)| (line_no, col_no))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    for (line, col) in starting_points {
        score += hike(
            &topographic_map,
            (max_line, max_col),
            (line, col),
            &mut HashSet::new(),
            false,
        );
    }

    println!("The sum of ratings of all trailheads is {}", score);

    Ok(())
}
