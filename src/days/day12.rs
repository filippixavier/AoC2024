use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day12.input");

fn get_input() -> Vec<Vec<char>> {
    INPUT
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_region_data(
    coord: (usize, usize),
    garden_plot: &mut Vec<Vec<(char, bool)>>,
    sides: &mut [HashMap<usize, HashSet<usize>>],
) -> (usize, usize) {
    let (mut perimeter, mut area) = (0, 1);

    if garden_plot[coord.0][coord.1].1 {
        (0, 0)
    } else {
        garden_plot[coord.0][coord.1].1 = true;
        let plot = garden_plot[coord.0][coord.1].0;
        let neighbors = [
            (coord.0.wrapping_sub(1), coord.1), // Up
            (coord.0, coord.1 + 1),             // Right
            (coord.0 + 1, coord.1),             // Down
            (coord.0, coord.1.wrapping_sub(1)), // Left
        ];
        for (i, neighbor) in neighbors.iter().enumerate() {
            let mut add_perimeter = false;
            if let Some(line) = garden_plot.get(neighbor.0) {
                if let Some((n_plot, _)) = line.get(neighbor.1) {
                    if n_plot == &plot {
                        let (n_perimeter, n_area) = get_region_data(*neighbor, garden_plot, sides);
                        perimeter += n_perimeter;
                        area += n_area;
                    } else {
                        add_perimeter = true;
                    }
                } else {
                    add_perimeter = true;
                }
            } else {
                add_perimeter = true;
            }

            if add_perimeter {
                perimeter += 1;
                match i {
                    0 | 2 => {
                        let line = sides[i].entry(coord.0).or_default();
                        line.insert(coord.1);
                    }
                    1 | 3 => {
                        let col = sides[i].entry(coord.1).or_default();
                        col.insert(coord.0);
                    }
                    _ => unreachable!(),
                };
            }
        }
        (perimeter, area)
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut garden_plot: Vec<Vec<(char, bool)>> = get_input()
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|elem| (elem, false))
                .collect::<Vec<(char, bool)>>()
        })
        .collect();
    let (height, width) = (garden_plot.len(), garden_plot[0].len());
    let mut price = 0;

    for line in 0..height {
        for col in 0..width {
            let (_, visited) = garden_plot[line][col];
            if visited {
                continue;
            } else {
                let mut sides = [
                    HashMap::<usize, HashSet<usize>>::new(),
                    HashMap::<usize, HashSet<usize>>::new(),
                    HashMap::<usize, HashSet<usize>>::new(),
                    HashMap::<usize, HashSet<usize>>::new(),
                ];
                let (region_perimeter, region_area) =
                    get_region_data((line, col), &mut garden_plot, &mut sides);
                price += region_area * region_perimeter;
            }
        }
    }
    println!("Fence price is: {}", price);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut garden_plot: Vec<Vec<(char, bool)>> = get_input()
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|elem| (elem, false))
                .collect::<Vec<(char, bool)>>()
        })
        .collect();
    let (height, width) = (garden_plot.len(), garden_plot[0].len());
    let mut price = 0;

    for line in 0..height {
        for col in 0..width {
            let (_, visited) = garden_plot[line][col];
            if visited {
                continue;
            } else {
                let mut sides = [
                    HashMap::<usize, HashSet<usize>>::new(),
                    HashMap::<usize, HashSet<usize>>::new(),
                    HashMap::<usize, HashSet<usize>>::new(),
                    HashMap::<usize, HashSet<usize>>::new(),
                ];
                let (_, region_area) = get_region_data((line, col), &mut garden_plot, &mut sides);
                let mut sides_count = 0;
                for side in sides {
                    for bordered_tile in side.values() {
                        let ordered = bordered_tile.iter().sorted().collect::<Vec<&usize>>();
                        let initial = ordered[0];
                        sides_count += ordered
                            .iter()
                            .skip(1)
                            .fold((initial, 1), |(previous, count), current| {
                                (
                                    current,
                                    if previous + 1 != **current {
                                        count + 1
                                    } else {
                                        count
                                    },
                                )
                            })
                            .1
                    }
                }
                price += region_area * sides_count;
            }
        }
    }
    println!("Using the bulk discount, fence price is: {}", price);
    Ok(())
}
