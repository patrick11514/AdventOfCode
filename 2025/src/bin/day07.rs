use std::{fmt::Write, ops::Add};

use advent_of_code_2025::{grid::Grid, load_file};

fn main() {
    let string = load_file("input.txt");
    let grid = Grid::new(string);

    println!("Star 1: {}", star_1(grid.clone()));
    println!("Star 2: {}", star_2(grid));
}

fn star_1(mut grid: Grid<char>) -> usize {
    let mut splits = 0;

    for row in 1..grid.height() {
        for col in 0..grid.width() {
            let up = grid.get_element(row - 1, col).unwrap();
            if up == &'S' {
                let char = grid.get_element_mut(row, col).unwrap();
                *char = '|';
                continue;
            }

            if up == &'|' {
                let char = grid.get_element_mut(row, col).unwrap();
                if char != &'^' {
                    *char = '|';
                    continue;
                }

                splits += 1;

                let left = grid.get_element_mut(row, col - 1).unwrap();
                *left = '|';
                let right = grid.get_element_mut(row, col + 1).unwrap();
                *right = '|';

                continue;
            }
        }
    }

    splits
}

#[derive(Debug, Clone, Copy)]
enum GridEl {
    Char(char),
    Number(usize),
}

impl std::fmt::Display for GridEl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridEl::Char(ch) => f.write_char(*ch),
            GridEl::Number(num) => f.write_str(&format!("{num}")),
        }
    }
}

impl PartialEq<char> for GridEl {
    fn eq(&self, other: &char) -> bool {
        match self {
            GridEl::Char(ch) => ch == other,
            GridEl::Number(_) => false,
        }
    }
}

impl From<char> for GridEl {
    fn from(value: char) -> Self {
        GridEl::Char(value)
    }
}

impl From<usize> for GridEl {
    fn from(value: usize) -> Self {
        GridEl::Number(value)
    }
}

impl Add<usize> for GridEl {
    type Output = GridEl;

    fn add(self, rhs: usize) -> Self::Output {
        match self {
            GridEl::Number(num) => GridEl::Number(num + rhs),
            v => v,
        }
    }
}

fn star_2(grid: Grid<char>) -> usize {
    let mut grid = grid.convert(|char| GridEl::Char(char));

    for row in 1..grid.height() {
        for col in 0..grid.width() {
            let up = *grid.get_element(row - 1, col).unwrap();
            if up == 'S' {
                let char = grid.get_element_mut(row, col).unwrap();
                *char = 1.into();
                continue;
            }

            if up != '.' && up != '^' {
                let char = grid.get_element_mut(row, col).unwrap();
                if char != &'^' {
                    match char {
                        GridEl::Char(_) => *char = up,
                        GridEl::Number(num) => *char = up + *num,
                    }
                    continue;
                }

                let left = grid.get_element_mut(row, col - 1).unwrap();
                match left {
                    GridEl::Char(_) => *left = up,
                    GridEl::Number(num) => *left = up + *num,
                }
                let right = grid.get_element_mut(row, col + 1).unwrap();
                match right {
                    GridEl::Char(_) => *right = up,
                    GridEl::Number(num) => *right = up + *num,
                }

                continue;
            }
        }
    }

    grid.get_row(grid.height() - 1)
        .unwrap()
        .into_iter()
        .fold(0usize, |acc, item| match item {
            GridEl::Char(_) => acc,
            GridEl::Number(num) => acc + num,
        })
}
