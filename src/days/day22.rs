use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
};

const INPUT: &str = include_str!("../../input/day22.input");
const PRUNE: isize = 16777216;

fn get_input() -> Vec<isize> {
    INPUT
        .trim()
        .lines()
        .map(|elem| elem.parse())
        .collect::<Result<Vec<isize>, _>>()
        .unwrap()
}

fn next_number(previous_secret: isize) -> isize {
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
    let starting_secrets = get_input();
    let all_secrets: Vec<Vec<isize>> = starting_secrets
        .into_iter()
        .map(|code| {
            let mut secrets = vec![code];
            for _ in 0..2_000 {
                secrets.push(next_number(*secrets.last().unwrap()));
            }
            secrets.into_iter().map(|elem| elem % 10).collect()
        })
        .collect();

    let mut changes_values: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();

    for secrets in all_secrets {
        let mut changes: VecDeque<isize> = VecDeque::new();
        let mut already_changed: HashSet<(isize, isize, isize, isize)> = HashSet::new();
        for (i, code) in secrets.iter().enumerate().skip(1) {
            changes.push_back(*code - secrets[i - 1]);

            if changes.len() == 4 {
                let variation = (changes[0], changes[1], changes[2], changes[3]);
                if already_changed.insert(variation) {
                    let total = changes_values.entry(variation).or_default();
                    *total += code;
                }
                changes.pop_front();
            }
        }
    }

    let (code, total) = changes_values
        .iter()
        .max_by(|(_, &total_a), (_, total_b)| total_a.cmp(total_b))
        .unwrap();

    println!(
        "Using the variations {:?}, we can gain {} bananas",
        code, total
    );

    Ok(())
}
