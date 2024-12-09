use std::error::Error;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day9.input");

fn get_input() -> Vec<u32> {
    INPUT
        .trim()
        .chars()
        .map(|elem| elem.to_digit(10))
        .collect::<Option<Vec<_>>>()
        .unwrap()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let memblocks = get_input();
    let mut memory_representation: Vec<String> = memblocks
        .iter()
        .enumerate()
        .flat_map(|(i, val)| {
            let id = if i % 2 == 0 {
                (i / 2).to_string()
            } else {
                String::from(".")
            };
            (0..*val).map(|_| id.clone()).collect::<Vec<String>>()
        })
        .collect();

    let (mut head, mut tail) = (0, memory_representation.len() - 1);

    while head != tail {
        if &memory_representation[head] == "." {
            memory_representation.swap(head, tail);
            while &memory_representation[tail] == "." && tail > head {
                tail -= 1;
            }
        }
        head += 1;
    }

    let checksum = memory_representation
        .iter()
        .enumerate()
        .fold_while(0, |acc, (index, id)| {
            if id == "." {
                Done(acc)
            } else {
                let id_parsed = id.parse::<usize>().unwrap();
                Continue(acc + (index * id_parsed))
            }
        })
        .into_inner();

    println!("Filesystem checksum is: {}", checksum);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
