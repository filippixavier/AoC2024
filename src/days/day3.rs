use std::error::Error;
use std::fs;
use std::path::Path;

use regex::Regex;

fn get_input() -> String {
    fs::read_to_string(Path::new("./input/day3.input"))
        .expect("Something went wrong with the input")
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let instructions = get_input();
    let correct_instruction = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;

    for (_, [num1, num2]) in correct_instruction
        .captures_iter(&instructions)
        .map(|c| c.extract())
    {
        total += num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap();
    }

    println!("Result of all added valid multiplications: {}", total);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
