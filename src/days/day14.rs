use regex::Regex;
use std::error::Error;
use std::io::{self};

const INPUT: &str = include_str!("../../input/day14.input");
const WIDTH: isize = 101;
const HEIGHT: isize = 103;

#[derive(Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn new(position: (isize, isize), velocity: (isize, isize)) -> Self {
        Robot { position, velocity }
    }
    fn next_position(&mut self, seconds: isize) {
        let total_x = (self.velocity.0 * seconds) % WIDTH;
        let total_y = (self.velocity.1 * seconds) % HEIGHT;

        self.position.0 = (self.position.0 + total_x) % WIDTH;
        self.position.1 = (self.position.1 + total_y) % HEIGHT;

        if self.position.0 < 0 {
            self.position.0 += WIDTH;
        }
        if self.position.1 < 0 {
            self.position.1 += HEIGHT;
        }
    }
}

fn get_input() -> Vec<Robot> {
    let reg = Regex::new(r"(\d+),(\d+).*v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = vec![];
    for caps in reg.captures_iter(INPUT) {
        let position = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
        let velocity = (caps[3].parse().unwrap(), caps[4].parse().unwrap());
        robots.push(Robot::new(position, velocity));
    }
    robots
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut robots = get_input();
    for robot in robots.iter_mut() {
        robot.next_position(100);
    }

    let mut quadrants = [0; 4];

    for robot in robots {
        if robot.position.0 < WIDTH / 2 && robot.position.1 < HEIGHT / 2 {
            quadrants[0] += 1;
        } else if robot.position.0 < WIDTH / 2 && robot.position.1 > HEIGHT / 2 {
            quadrants[1] += 1;
        } else if robot.position.0 > WIDTH / 2 && robot.position.1 < HEIGHT / 2 {
            quadrants[2] += 1;
        } else if robot.position.0 > WIDTH / 2 && robot.position.1 > HEIGHT / 2 {
            quadrants[3] += 1;
        }
    }

    println!(
        "The safety factor is: {}",
        quadrants.iter().product::<usize>()
    );

    Ok(())
}

fn display(robots: &[Robot]) {
    for line in 0..HEIGHT {
        for col in 0..WIDTH {
            let count = robots.iter().fold(0, |acc, robot| {
                if robot.position.0 == col && robot.position.1 == line {
                    acc + 1
                } else {
                    acc
                }
            });
            print!(
                "{}",
                if count == 0 {
                    String::from(".")
                } else {
                    count.to_string()
                }
            );
        }
        println!();
    }
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut robots = get_input();
    let mut seconds = HEIGHT + 1;
    // I expect it to only work with my input
    // While looking at the positions frame by frame, I've noticed some patterns reocuring every HEIGHT iteration and every WIDTH iterations
    let steps = HEIGHT;

    for robot in robots.iter_mut() {
        robot.next_position(1);
    }

    display(&robots);
    loop {
        let displaying = true;
        for robot in robots.iter_mut() {
            robot.next_position(steps);
        }

        if displaying {
            let mut buffer = String::new();
            println!("{}:", seconds);
            display(&robots);
            println!("Keep running? Y/N");
            match io::stdin().read_line(&mut buffer) {
                Ok(_) => {
                    let test = buffer.chars().next().unwrap_or('N');
                    if test == 'N' {
                        break;
                    }
                }
                Err(error) => {
                    println!("Error: {}", error);
                    break;
                }
            }
        }
        seconds += steps;
    }
    println!("Easter egg displaying at {} seconds", seconds);
    Ok(())
}
