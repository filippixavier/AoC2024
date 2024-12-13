use std::error::Error;

use regex::Regex;

const INPUT: &str = include_str!("../../input/day13.input");

struct Machine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    fn gcd(a: isize, b: isize) -> isize {
        if b == 0 {
            a
        } else {
            Machine::gcd(b, a % b)
        }
    }
    fn new(values: &[isize]) -> Self {
        Machine {
            button_a: (values[0], values[1]),
            button_b: (values[2], values[3]),
            prize: (values[4], values[5]),
        }
    }
    fn gcds(&self) -> (isize, isize) {
        (
            Machine::gcd(
                self.button_a.0.max(self.button_a.1),
                self.button_a.0.min(self.button_a.1),
            ),
            Machine::gcd(
                self.button_b.0.max(self.button_b.1),
                self.button_b.0.min(self.button_b.1),
            ),
        )
    }
    fn solve(&self) -> Option<(isize, isize)> {
        let gcds = self.gcds();
        let prize_without_a =
            (self.prize.0 * self.button_a.1 / gcds.0) - (self.prize.1 * self.button_a.0 / gcds.0);
        let pressed_b = (self.button_b.0 * self.button_a.1) / gcds.0
            - (self.button_b.1 * self.button_a.0 / gcds.0);
        let prize_without_b =
            (self.prize.0 * self.button_b.1 / gcds.1) - (self.prize.1 * self.button_b.0 / gcds.1);
        let pressed_a = (self.button_a.0 * self.button_b.1) / gcds.1
            - (self.button_a.1 * self.button_b.0 / gcds.1);
        if prize_without_a % pressed_b == 0 && prize_without_b % pressed_a == 0 {
            Some((prize_without_b / pressed_a, prize_without_a / pressed_b))
        } else {
            None
        }
    }
}

fn get_input() -> Vec<Machine> {
    // Somehow, r".*?(\d+).*?(\d+)\s.*?(\d+).*?(\d+)\s.*?(\d+).*?(\d+)" does not work because \s doesn't include windows line ending
    let reg = Regex::new(r".*?(\d+).*?(\d+)\r\n.*?(\d+).*?(\d+)\r\n.*?(\d+).*?(\d+)").unwrap();
    let mut machines = vec![];
    for caps in reg.captures_iter(INPUT.trim()) {
        let values: Vec<isize> = vec![
            caps[1].parse().unwrap(),
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
            caps[4].parse().unwrap(),
            caps[5].parse().unwrap(),
            caps[6].parse().unwrap(),
        ];
        machines.push(Machine::new(&values));
    }
    machines
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let machines = get_input();
    let mut tokens = 0;
    for machine in machines {
        if let Some((button_a, button_b)) = machine.solve() {
            if button_a <= 100 && button_b <= 100 {
                tokens += button_a * 3 + button_b;
            }
        }
    }

    println!(
        "To win all possible prices, we need to spend at least {} tokens",
        tokens
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
