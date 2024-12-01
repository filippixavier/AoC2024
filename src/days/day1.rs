use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn get_input() -> (Vec<isize>, Vec<isize>) {
    let input = fs::read_to_string(Path::new("./input/day1.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .fold((vec![], vec![]), |mut acc, line| {
            let numbers = line.split_whitespace().collect_vec();
            acc.0.push(numbers[0].parse().unwrap());
            acc.1.push(numbers[1].parse().unwrap());
            acc
        })
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut first_list, mut second_list) = get_input();
    let mut total_distance = 0;
    first_list.sort();
    second_list.sort();
    for (first_val, second_val) in first_list.iter().zip(second_list.iter()) {
        total_distance += first_val.abs_diff(*second_val);
    }
    println!("Total distance between the two lists: {:?}", total_distance);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut first_list, mut second_list) = get_input();
    let mut similarity_score = 0;
    first_list.sort();
    second_list.sort_by(|a, b| b.cmp(a));
    println!("{}, {:?}", first_list[0], second_list.last());
    for first_num in first_list {
        let mut count = 0;
        while !second_list.is_empty() && second_list.last().unwrap() <= &first_num {
            let second_num = second_list.pop().unwrap();
            if second_num == first_num {
                count += 1;
            }
        }
        similarity_score += count * first_num;
    }
    println!("Total Similarity Score is {:?}", similarity_score);
    Ok(())
}
