use std::error::Error;

const INPUT: &str = include_str!("../../input/day7.input");

fn get_input() -> Vec<(usize, Vec<usize>)> {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse::<usize>().unwrap();
            let equation = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|elem| elem.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            (result, equation)
        })
        .collect()
}

fn calc(target: usize, current: usize, remaining: &[usize], use_concat: bool) -> bool {
    if remaining.is_empty() {
        return target == current;
    }

    let next = remaining[0];
    let mut result = false;

    if next + current <= target {
        result = result || calc(target, next + current, &remaining[1..], use_concat);
    }

    if !result && next * current <= target {
        result = result || calc(target, next * current, &remaining[1..], use_concat);
    }

    if use_concat {
        let concat = current * 10usize.pow(usize::ilog10(next) + 1) + next;

        if !result && concat <= target {
            result = result || calc(target, concat, &remaining[1..], use_concat);
        }
    }

    result
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = get_input();
    let mut total = 0;
    for (target, values) in input {
        if calc(target, values[0], &values[1..], false) {
            total += target;
        }
    }
    println!("Total calibration result is {}", total);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = get_input();
    let mut total = 0;
    for (target, values) in input {
        if calc(target, values[0], &values[1..], true) {
            total += target;
        }
    }
    println!(
        "Including concatenation, the total calibration result is {}",
        total
    );
    Ok(())
}
