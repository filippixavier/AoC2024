use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input() -> Vec<Vec<usize>> {
    let input = fs::read_to_string(Path::new("./input/day2.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|elem| elem.parse())
                .collect::<Result<Vec<usize>, _>>()
                .unwrap()
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = get_input();
    let mut safe_report_count = 0;
    for report in input {
        let mut previous_sign: Option<bool> = None;
        let mut unsafe_report = false;
        for i in 0..report.len() - 1 {
            let (a, b) = (report[i], report[i + 1]);
            let sign = a < b;
            if let Some(is_increasing) = previous_sign {
                if is_increasing != sign {
                    unsafe_report = true;
                    break;
                }
            } else {
                previous_sign = Some(sign);
            }

            let step = a.abs_diff(b);
            if !(1..=3).contains(&step) {
                unsafe_report = true;
                break;
            }
        }
        if !unsafe_report {
            safe_report_count += 1;
        }
    }
    println!("There is {} safe reports", safe_report_count);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
