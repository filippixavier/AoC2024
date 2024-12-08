use std::collections::{HashMap, HashSet};
use std::error::Error;

use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day8.input");

fn get_input() -> Vec<Vec<char>> {
    INPUT
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn display_map(map: &[Vec<char>], antennas: &HashSet<(usize, usize)>) {
    for (i, line) in map.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if ch.is_ascii_alphanumeric() {
                print!("{}", ch);
            } else if antennas.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let radio_map = get_input();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let bottom_right_coord = (radio_map.len(), radio_map[0].len());

    for (i, map_line) in radio_map.iter().enumerate() {
        for (j, map_val) in map_line.iter().enumerate() {
            if map_val.is_ascii_alphanumeric() {
                let coordinates = antennas.entry(*map_val).or_default();
                coordinates.push((i, j));
            }
        }
    }

    for coordinates in antennas.values() {
        for couple in coordinates.iter().combinations(2) {
            let (left, right) = (couple[0].min(couple[1]), couple[0].max(couple[1]));
            let (diff_x, diff_y) = (left.0.abs_diff(right.0), left.1.abs_diff(right.1));

            if diff_x <= left.0 {
                if left.1 <= right.1 && diff_y <= left.1 {
                    antinodes.insert((left.0 - diff_x, left.1 - diff_y));
                }
                if left.1 > right.1 && left.1 + diff_y < bottom_right_coord.1 {
                    antinodes.insert((left.0 - diff_x, left.1 + diff_y));
                }
            }
            if right.0 + diff_x < bottom_right_coord.0 {
                if right.1 <= left.1 && diff_y <= right.1 {
                    antinodes.insert((right.0 + diff_x, right.1 - diff_y));
                }
                if right.1 > left.1 && right.1 + diff_y < bottom_right_coord.1 {
                    antinodes.insert((right.0 + diff_x, right.1 + diff_y));
                }
            };
        }
    }

    display_map(&radio_map, &antinodes);

    println!("There are {:?} antinodes within the map", antinodes.len());

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let radio_map = get_input();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let bottom_right_coord = (radio_map.len(), radio_map[0].len());

    for (i, map_line) in radio_map.iter().enumerate() {
        for (j, map_val) in map_line.iter().enumerate() {
            if map_val.is_ascii_alphanumeric() {
                let coordinates = antennas.entry(*map_val).or_default();
                coordinates.push((i, j));
            }
        }
    }

    for coordinates in antennas.values() {
        for couple in coordinates.iter().combinations(2) {
            let (left, right) = (couple[0], couple[1]);

            let a: (isize, isize) = (
                right.0 as isize - left.0 as isize,
                right.1 as isize - left.1 as isize,
            );

            let b: (isize, isize) = (left.0 as isize * a.1 - a.0 * left.1 as isize, a.1);

            if a.1 == 0 {
                for y in 0..bottom_right_coord.0 {
                    antinodes.insert((y, left.1));
                }
            } else {
                for x in 0..bottom_right_coord.1 {
                    let point = (a.0 * x as isize + b.0, a.1);
                    if point.0 % point.1 == 0 {
                        let y = (point.0 / point.1) as usize;
                        if y < bottom_right_coord.0 {
                            antinodes.insert((y, x));
                        }
                    }
                }
            }
        }
    }

    println!();

    display_map(&radio_map, &antinodes);

    println!(
        "Counting harmonic resonances, there are {:?} antinodes within the map",
        antinodes.len()
    );

    Ok(())
}
