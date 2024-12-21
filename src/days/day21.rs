use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

const INPUT: &str = include_str!("../../input/day21.input");

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Keys {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyUp,
    KeyRight,
    KeyDown,
    KeyLeft,
    Empty,
}

use Keys::*;

fn get_input() -> Vec<(Vec<Keys>, usize)> {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            (
                line.chars()
                    .map(|c| match c {
                        '0' => Key0,
                        '1' => Key1,
                        '2' => Key2,
                        '3' => Key3,
                        '4' => Key4,
                        '5' => Key5,
                        '6' => Key6,
                        '7' => Key7,
                        '8' => Key8,
                        '9' => Key9,
                        'A' => KeyA,
                        _ => unreachable!(),
                    })
                    .collect(),
                line[0..3].parse().unwrap(),
            )
        })
        .collect()
}

fn find_shortests(
    start: (usize, usize),
    keypad: &HashMap<(usize, usize), Keys>,
) -> HashMap<Keys, Vec<Keys>> {
    let mut paths: HashMap<Keys, Vec<Keys>> = HashMap::new();

    let mut to_do: VecDeque<((usize, usize), Vec<Keys>)> = vec![(start, vec![])].into();

    while let Some((coordinate, path)) = to_do.pop_front() {
        let current_key = keypad.get(&coordinate).unwrap();

        let shortest = paths.entry(*current_key).or_insert(path.clone());

        let mut deduped_shortest = shortest.clone();
        deduped_shortest.dedup();
        let mut deduped_path = path.clone();
        deduped_path.dedup();

        if deduped_shortest.len() >= deduped_path.len() {
            *shortest = path.clone();
        } else {
            continue;
        }

        let neighbors = [
            ((coordinate.0.wrapping_sub(1), coordinate.1), KeyUp),
            ((coordinate.0, coordinate.1 + 1), KeyRight),
            ((coordinate.0 + 1, coordinate.1), KeyDown),
            ((coordinate.0, coordinate.1.wrapping_sub(1)), KeyLeft),
        ];

        for (neighbor, direction) in neighbors {
            if let Some(key) = keypad.get(&neighbor) {
                if *key != Empty {
                    let mut next_path = path.clone();
                    next_path.push(direction);
                    to_do.push_back((neighbor, next_path));
                }
            }
        }
    }
    paths
}

fn find_all_shortests(
    keypad: &HashMap<(usize, usize), Keys>,
) -> HashMap<Keys, HashMap<Keys, Vec<Keys>>> {
    keypad
        .iter()
        .map(|(coord, key)| (*key, find_shortests(*coord, keypad)))
        .collect()
}

fn find_shortest(code: &[Keys], shortests: &HashMap<Keys, HashMap<Keys, Vec<Keys>>>) -> Vec<Keys> {
    let mut current = KeyA;
    code.iter()
        .flat_map(move |key| {
            let mut path = shortests.get(&current).unwrap().get(key).unwrap().clone();
            path.push(KeyA);
            current = *key;
            path
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let numpad: HashMap<(usize, usize), Keys> = vec![
        ((0, 0), Key7),
        ((0, 1), Key8),
        ((0, 2), Key9),
        ((1, 0), Key4),
        ((1, 1), Key5),
        ((1, 2), Key6),
        ((2, 0), Key1),
        ((2, 1), Key2),
        ((2, 2), Key3),
        ((3, 0), Empty),
        ((3, 1), Key0),
        ((3, 2), KeyA),
    ]
    .into_iter()
    .collect();
    let control: HashMap<(usize, usize), Keys> = vec![
        ((0, 0), Empty),
        ((0, 1), KeyUp),
        ((0, 2), KeyA),
        ((1, 0), KeyLeft),
        ((1, 1), KeyDown),
        ((1, 2), KeyRight),
    ]
    .into_iter()
    .collect();

    let codes = get_input();
    let mut complexities = 0;
    let shortests_paths_numpad = find_all_shortests(&numpad);
    let shortests_paths_control = find_all_shortests(&control);

    for (code, value) in codes {
        let keypad_code = find_shortest(&code, &shortests_paths_numpad);
        let drone_1 = find_shortest(&keypad_code, &shortests_paths_control);
        let drone_2 = find_shortest(&drone_1, &shortests_paths_control);
        complexities += value * drone_2.len();
    }

    println!("Total complexities is {}", complexities);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
