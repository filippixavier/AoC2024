use std::error::Error;

const INPUT: &str = include_str!("../../input/day12.input");

fn get_input() -> Vec<Vec<char>> {
    INPUT
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_region_data(
    coord: (usize, usize),
    garden_plot: &mut Vec<Vec<(char, bool)>>,
) -> (usize, usize) {
    let (mut perimeter, mut area) = (0, 1);

    if garden_plot[coord.0][coord.1].1 {
        (0, 0)
    } else {
        garden_plot[coord.0][coord.1].1 = true;
        let plot = garden_plot[coord.0][coord.1].0;
        let neighbors = [
            (coord.0.wrapping_sub(1), coord.1),
            (coord.0, coord.1 + 1),
            (coord.0 + 1, coord.1),
            (coord.0, coord.1.wrapping_sub(1)),
        ];
        for neighbor in neighbors {
            if let Some(line) = garden_plot.get(neighbor.0) {
                if let Some((n_plot, _)) = line.get(neighbor.1) {
                    if n_plot == &plot {
                        let (n_perimeter, n_area) = get_region_data(neighbor, garden_plot);
                        perimeter += n_perimeter;
                        area += n_area;
                    } else {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
        (perimeter, area)
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut garden_plot: Vec<Vec<(char, bool)>> = get_input()
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|elem| (elem, false))
                .collect::<Vec<(char, bool)>>()
        })
        .collect();
    let (height, width) = (garden_plot.len(), garden_plot[0].len());
    let mut price = 0;

    for line in 0..height {
        for col in 0..width {
            let (_, visited) = garden_plot[line][col];
            if visited {
                continue;
            } else {
                let (region_area, region_perimeter) =
                    get_region_data((line, col), &mut garden_plot);
                price += region_area * region_perimeter;
            }
        }
    }
    println!("Fence price is: {}", price);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
