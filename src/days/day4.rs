use std::error::Error;

const INPUT: &str = include_str!("../../input/day4.input");

fn get_input() -> Vec<Vec<char>> {
    INPUT
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let word_search = get_input();

    let mut xmas_count = 0;

    let checker = |coordinate: (usize, usize), target: char| {
        if let Some(line) = word_search.get(coordinate.0) {
            if let Some(character) = line.get(coordinate.1) {
                if *character == target {
                    return true;
                }
            }
        }
        false
    };

    for (i, line) in word_search.iter().enumerate() {
        for (j, character) in line.iter().enumerate() {
            if *character != 'X' {
                continue;
            }
            let mut directions = vec![
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ];
            for x in 1..=3 {
                directions.retain(|dir| {
                    let coordinates = (
                        match dir.0 {
                            -1 => i.wrapping_sub(x),
                            0 => i,
                            1 => i + x,
                            _ => unreachable!(),
                        },
                        match dir.1 {
                            -1 => j.wrapping_sub(x),
                            0 => j,
                            1 => j + x,
                            _ => unreachable!(),
                        },
                    );
                    checker(
                        coordinates,
                        match x {
                            1 => 'M',
                            2 => 'A',
                            3 => 'S',
                            _ => unreachable!(),
                        },
                    )
                });
            }
            xmas_count += directions.len();
        }
    }

    println!("There are {} XMAS in the input", xmas_count);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let word_search = get_input();

    let mut xmas_count = 0;

    let checker = |coordinate: (usize, usize), target: char| {
        if let Some(line) = word_search.get(coordinate.0) {
            if let Some(character) = line.get(coordinate.1) {
                if *character == target {
                    return true;
                }
            }
        }
        false
    };

    for (i, line) in word_search.iter().enumerate() {
        for (j, character) in line.iter().enumerate() {
            if *character != 'A' {
                continue;
            }
            let mut directions = vec![(1, 1), (-1, -1), (1, -1), (-1, 1)];
            directions.retain(|dir| {
                let previous = (-dir.0, -dir.1);
                let m_coord = (
                    match previous.0 {
                        -1 => i.wrapping_sub(1),
                        1 => i + 1,
                        _ => unreachable!(),
                    },
                    match previous.1 {
                        -1 => j.wrapping_sub(1),
                        1 => j + 1,
                        _ => unreachable!(),
                    },
                );
                let s_coord = (
                    match dir.0 {
                        -1 => i.wrapping_sub(1),
                        1 => i + 1,
                        _ => unreachable!(),
                    },
                    match dir.1 {
                        -1 => j.wrapping_sub(1),
                        1 => j + 1,
                        _ => unreachable!(),
                    },
                );
                checker(m_coord, 'M') && checker(s_coord, 'S')
            });
            if directions.len() == 2 {
                xmas_count += 1;
            }
        }
    }

    println!("There are {} X-MAS in the input", xmas_count);

    Ok(())
}
