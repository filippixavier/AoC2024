use std::{collections::HashMap, error::Error};

const INPUT: &str = include_str!("../../input/day11.input");

fn get_input() -> Vec<usize> {
    INPUT
        .split_whitespace()
        .map(|value| value.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut stones = get_input();
    for _ in 0..25 {
        let mut next_stones = vec![];
        for stone in stones {
            if stone == 0 {
                next_stones.push(1);
                continue;
            }
            let stone_str = stone.to_string();

            if stone_str.len() % 2 == 0 {
                let split = stone_str.split_at(stone_str.len() / 2);
                let left_stone = split.0.parse::<usize>().unwrap();
                let right_stone = split.1.parse::<usize>().unwrap();
                next_stones.push(left_stone);
                next_stones.push(right_stone);
                continue;
            }

            next_stones.push(stone * 2024);
        }

        stones = next_stones;
    }
    println!("After blinking 25 times, there are {} stones", stones.len());
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut stones: HashMap<usize, usize> = get_input()
        .into_iter()
        .map(|value| (value, 1))
        .collect::<HashMap<usize, usize>>();

    for _ in 0..75 {
        let mut next_stones = HashMap::new();
        for (stone, count) in stones {
            let stone_str = stone.to_string();
            let blinked_stones = if stone == 0 {
                vec![1]
            } else if stone_str.len() % 2 == 0 {
                let split = stone_str.split_at(stone_str.len() / 2);
                let left_stone = split.0.parse::<usize>().unwrap();
                let right_stone = split.1.parse::<usize>().unwrap();
                vec![left_stone, right_stone]
            } else {
                vec![stone * 2024]
            };
            for blink in blinked_stones {
                let next_count = next_stones.entry(blink).or_default();
                *next_count += count;
            }
        }
        stones = next_stones;
    }

    let score: usize = stones.values().sum();
    println!("After blinking *75* times, there are {} stones", score);
    Ok(())
}
