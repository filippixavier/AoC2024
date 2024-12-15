use std::error::Error;

const INPUT: &str = include_str!("../../input/day15.input");

type Warehouse = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Crate,
}

use Tile::*;

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

use Move::*;

fn get_input() -> (Warehouse, (usize, usize), Vec<Move>) {
    let mut robot_pos = (0, 0);

    let mut parts = INPUT.trim().split("\r\n\r\n");
    let raw_warehouse = parts.next().unwrap();
    let raw_movements = parts.next().unwrap();

    let warehouse = raw_warehouse
        .trim()
        .lines()
        .enumerate()
        .map(|(line_no, line)| {
            line.chars()
                .enumerate()
                .map(|(col_no, c)| match c {
                    '#' => Wall,
                    '.' => Empty,
                    'O' => Crate,
                    '@' => {
                        robot_pos = (line_no, col_no);
                        Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let movements = raw_movements
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|e| match e {
                    '<' => Left,
                    '^' => Up,
                    'v' => Down,
                    '>' => Right,
                    _ => unreachable!(),
                })
                .collect::<Vec<Move>>()
        })
        .collect();

    (warehouse, robot_pos, movements)
}

fn try_move(movement: &Move, position: (usize, usize), warehouse: &mut Warehouse) -> bool {
    let neighbor_pos = match movement {
        Up => (position.0 - 1, position.1),
        Down => (position.0 + 1, position.1),
        Left => (position.0, position.1 - 1),
        Right => (position.0, position.1 + 1),
    };
    match warehouse[neighbor_pos.0][neighbor_pos.1] {
        Empty => {
            warehouse[neighbor_pos.0][neighbor_pos.1] = warehouse[position.0][position.1];
            warehouse[position.0][position.1] = Empty;
            true
        }
        Crate => {
            if try_move(movement, neighbor_pos, warehouse) {
                warehouse[neighbor_pos.0][neighbor_pos.1] = warehouse[position.0][position.1];
                warehouse[position.0][position.1] = Empty;
                true
            } else {
                false
            }
        }
        Wall => false,
    }
}

fn display(warehouse: &Warehouse, robot: (usize, usize)) {
    for (line_no, line) in warehouse.iter().enumerate() {
        for (col_no, elem) in line.iter().enumerate() {
            if (line_no, col_no) == robot {
                print!("@");
            } else {
                print!(
                    "{}",
                    match elem {
                        Wall => '#',
                        Crate => 'O',
                        Empty => '.',
                    }
                );
            }
        }
        println!();
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut warehouse, mut position, movements) = get_input();

    for movement in movements {
        if try_move(&movement, position, &mut warehouse) {
            position = match movement {
                Up => (position.0 - 1, position.1),
                Down => (position.0 + 1, position.1),
                Left => (position.0, position.1 - 1),
                Right => (position.0, position.1 + 1),
            };
        }
        // display(&warehouse, position);
    }

    let mut gps = 0;

    for (line_no, line) in warehouse.iter().enumerate() {
        for (col_no, elem) in line.iter().enumerate() {
            if let Crate = elem {
                gps += 100 * line_no + col_no
            }
        }
    }

    println!("The sum of all gps coordinates is: {}", gps);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
