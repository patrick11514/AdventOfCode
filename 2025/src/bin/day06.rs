use core::panic;

use advent_of_code_2025::{grid::Grid, load_file, load_lines};

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Multiply,
}

impl From<&String> for Operator {
    fn from(s: &String) -> Self {
        s.as_str().into()
    }
}

impl From<String> for Operator {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "+" => Operator::Plus,
            "*" => Operator::Multiply,
            _ => panic!(),
        }
    }
}

impl From<&[char]> for Operator {
    fn from(value: &[char]) -> Self {
        let str = value.iter().map(|c| c.clone()).collect::<String>();
        let trimmed = str.trim();
        trimmed.into()
    }
}

#[derive(Debug, Clone)]
struct MathProblem {
    numbers: Vec<u128>,
    operator: Operator,
}

impl MathProblem {
    fn solve(self) -> u128 {
        self.numbers
            .into_iter()
            .reduce(|a, b| match self.operator {
                Operator::Plus => a + b,
                Operator::Multiply => a * b,
            })
            .unwrap_or_default()
    }
}

fn main() {
    let str = load_file("input.txt");

    let solution = str
        .lines()
        .map(|line| {
            line.split(' ')
                .filter_map(|item| {
                    if item.len() > 0 {
                        Some(item.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut problems = Vec::new();

    for x in 0..solution[0].len() {
        let mut numbers = Vec::new();

        for y in 0..(solution.len() - 1) {
            numbers.push(solution[y][x].clone());
        }

        problems.push(MathProblem {
            numbers: numbers
                .into_iter()
                .map(|n| n.parse::<u128>().unwrap())
                .collect(),
            operator: (&solution[solution.len() - 1][x]).into(),
        });
    }

    println!(
        "Star 1: {}",
        problems.into_iter().map(|p| p.solve()).sum::<u128>()
    );

    let grid = Grid::new(str);

    let mut problems_2 = Vec::new();

    let h = grid.height();
    let mut i = 0;
    while i < grid.width() {
        let mut end = i + 1;

        loop {
            let el = grid.get_element(h - 1, end).unwrap();

            if el != &' ' {
                end = end - 1;
                break;
            }

            end += 1;

            if end >= grid.width() {
                break;
            }
        }

        let mut numbers = Vec::new();

        for col in i..end {
            let mut digit = 0;
            let mut number = 0u128;

            for i in 0..(grid.height() - 1) {
                let char = grid.get_element(grid.height() - 2 - i, col).unwrap();

                if char == &' ' {
                    continue;
                }

                number += (char.to_digit(10).unwrap() as u128) * 10u128.pow(digit);

                digit += 1;
            }

            numbers.push(number);
        }

        problems_2.push(MathProblem {
            numbers,
            operator: (&grid.get_row(grid.height() - 1).unwrap()[i..end]).into(),
        });

        i = end + 1;
    }

    println!(
        "Star 2: {}",
        problems_2.into_iter().map(|p| p.solve()).sum::<u128>()
    );
}
