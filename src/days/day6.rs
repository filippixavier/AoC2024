use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

const INPUT: &str = include_str!("../../input/day6.input");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Object,
    Up,
    Down,
    Left,
    Right,
}

fn get_input() -> HashMap<(isize, isize), Tile> {
    HashMap::from_iter(
        INPUT
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(line_no, line)| {
                line.chars().enumerate().map(move |(tile_no, tile_char)| {
                    let tile = match tile_char {
                        '.' => Tile::Empty,
                        '#' => Tile::Object,
                        '^' => Tile::Up,
                        '>' => Tile::Right,
                        '<' => Tile::Left,
                        'v' => Tile::Down,
                        _ => unreachable!(),
                    };
                    ((line_no as isize, tile_no as isize), tile)
                })
            }),
    )
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut guard_route = get_input();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let (mut guard_pos, mut guard_facing) = guard_route
        .iter()
        .find_map(|(pos, tile)| match tile {
            Tile::Up | Tile::Down | Tile::Left | Tile::Right => Some((*pos, *tile)),
            _ => None,
        })
        .unwrap();
    guard_route.insert(guard_pos, Tile::Empty);

    while guard_route.contains_key(&guard_pos) {
        visited.insert(guard_pos);
        let next_pos = match guard_facing {
            Tile::Up => (guard_pos.0 - 1, guard_pos.1),
            Tile::Down => (guard_pos.0 + 1, guard_pos.1),
            Tile::Left => (guard_pos.0, guard_pos.1 - 1),
            Tile::Right => (guard_pos.0, guard_pos.1 + 1),
            _ => unreachable!(),
        };
        if let Some(tile) = guard_route.get(&next_pos) {
            if *tile != Tile::Empty {
                guard_facing = match guard_facing {
                    Tile::Up => Tile::Right,
                    Tile::Down => Tile::Left,
                    Tile::Left => Tile::Up,
                    Tile::Right => Tile::Down,
                    _ => unreachable!(),
                };
            } else {
                guard_pos = next_pos;
            }
        } else {
            guard_pos = next_pos;
        }
    }
    println!(
        "During her shift, the guard will visit {} tiles",
        visited.len()
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
