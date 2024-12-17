use std::error::Error;

const INPUT: &str = include_str!("../../input/day17.input");

fn get_input() -> ((usize, usize, usize), Vec<usize>) {
    let mut instructions = vec![];
    let mut registers_init = (0, 0, 0);

    for line in INPUT.trim().lines() {
        if line.is_empty() {
            continue;
        }
        let parts = line.split(": ").collect::<Vec<&str>>();

        match parts[0] {
            "Register A" => {
                registers_init.0 = parts[1].parse().unwrap();
            }
            "Register B" => {
                registers_init.1 = parts[1].parse().unwrap();
            }
            "Register C" => {
                registers_init.2 = parts[1].parse().unwrap();
            }
            "Program" => {
                instructions = parts[1]
                    .split(",")
                    .map(|elem| elem.parse())
                    .collect::<Result<Vec<usize>, _>>()
                    .unwrap();
            }
            _ => unreachable!(),
        }
    }

    (registers_init, instructions)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let ((mut reg_a, mut reg_b, mut reg_c), instructions) = get_input();
    let mut instruction_counter = 0;
    let mut output: Vec<String> = vec![];

    loop {
        let (op_code, operand) = (
            instructions[instruction_counter],
            instructions[instruction_counter + 1],
        );
        let mut counter_advanced = false;
        match op_code {
            0 => {
                // adv
                let param = match operand {
                    a @ 0..=3 => a,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    _ => unreachable!(),
                };
                reg_a /= 2usize.pow(param as u32);
            }
            1 => {
                // bxl
                reg_b ^= operand;
            }
            2 => {
                // bst
                reg_b = match operand {
                    a @ 0..=3 => a,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    _ => unreachable!(),
                } % 8;
            }
            3 => {
                // jnz
                if reg_a != 0 {
                    instruction_counter = operand;
                    counter_advanced = true;
                }
            }
            4 => {
                // bxc
                reg_b ^= reg_c;
            }
            5 => {
                // out
                output.push(
                    (match operand {
                        a @ 0..=3 => a,
                        4 => reg_a,
                        5 => reg_b,
                        6 => reg_c,
                        _ => unreachable!(),
                    } % 8)
                        .to_string(),
                );
            }
            6 => {
                // bdv
                let param = match operand {
                    a @ 0..=3 => a,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    _ => unreachable!(),
                };
                reg_b = reg_a / 2usize.pow(param as u32);
            }
            7 => {
                let param = match operand {
                    a @ 0..=3 => a,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    _ => unreachable!(),
                };
                reg_c = reg_a / 2usize.pow(param as u32);
            }
            _ => unreachable!(),
        }
        if !counter_advanced {
            instruction_counter += 2;
            if instruction_counter >= instructions.len() - 1 {
                break;
            }
        }
    }

    println!("{}", output.join(","));

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
