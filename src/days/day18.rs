use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

const INPUT: &str = include_str!("../../input/day18.input");

type Coordinate = (usize, usize);

fn get_input() -> Vec<Coordinate> {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            let parts = line.split(',').collect::<Vec<_>>();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let walls = get_input()
        .into_iter()
        .take(1024)
        .collect::<HashSet<Coordinate>>();
    let end = (70, 70);
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut possible = VecDeque::<(Coordinate, usize)>::new();
    possible.push_front(((0, 0), 0));

    while let Some((coordinate, steps)) = possible.pop_front() {
        if !visited.insert(coordinate) {
            continue;
        }
        if coordinate == end {
            println!("You can escape the corrupted space in {} steps", steps);
            break;
        }

        let neighbors = [
            (coordinate.0.wrapping_sub(1), coordinate.1),
            (coordinate.0, coordinate.1 + 1),
            (coordinate.0 + 1, coordinate.1),
            (coordinate.0, coordinate.1.wrapping_sub(1)),
        ];

        for neighbor in neighbors {
            if neighbor.0 <= end.0 && neighbor.1 <= end.1 && !walls.contains(&neighbor) {
                possible.push_back((neighbor, steps + 1));
            }
        }
    }
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let walls_coordinates = get_input();
    let end = (70, 70);
    let mut walls: HashSet<Coordinate> = HashSet::new();

    for wall in walls_coordinates {
        walls.insert(wall);

        let mut visited: HashSet<Coordinate> = HashSet::new();
        let mut possible = VecDeque::<(Coordinate, usize)>::new();
        possible.push_front(((0, 0), 0));
        let mut escape = false;

        while let Some((coordinate, steps)) = possible.pop_front() {
            if !visited.insert(coordinate) {
                continue;
            }
            if coordinate == end {
                escape = true;
                break;
            }

            let neighbors = [
                (coordinate.0.wrapping_sub(1), coordinate.1),
                (coordinate.0, coordinate.1 + 1),
                (coordinate.0 + 1, coordinate.1),
                (coordinate.0, coordinate.1.wrapping_sub(1)),
            ];

            for neighbor in neighbors {
                if neighbor.0 <= end.0 && neighbor.1 <= end.1 && !walls.contains(&neighbor) {
                    possible.push_back((neighbor, steps + 1));
                }
            }
        }

        if !escape {
            println!(
                "Once {:?} is corrupted, there will be no more escape routes",
                wall
            );
            break;
        }
    }

    Ok(())
}
