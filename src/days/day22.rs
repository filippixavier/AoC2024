use std::error::Error;

const INPUT: &str = include_str!("../../input/day22.input");
const PRUNE: usize = 16777216;

fn get_input() -> Vec<usize> {
    INPUT
        .trim()
        .lines()
        .map(|elem| elem.parse())
        .collect::<Result<Vec<usize>, _>>()
        .unwrap()
}

fn next_number(previous_secret: usize) -> usize {
    let mut step_1 = previous_secret * 64;
    step_1 ^= previous_secret;
    step_1 %= PRUNE;

    let mut step_2 = step_1 / 32;
    step_2 ^= step_1;
    step_2 %= PRUNE;

    let mut final_step = step_2 * 2_048;
    final_step ^= step_2;

    final_step % PRUNE
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let starting_secrets = get_input();
    let mut total = 0;

    for secret in starting_secrets {
        let mut temp = secret;
        for _ in 0..2_000 {
            temp = next_number(temp);
        }
        total += temp;
    }

    println!(" The sum of all secret buyer's secret is: {}", total);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
