use std::{fs, path::Path};

use crate::grid::Grid;

pub mod grid;

pub fn load_file(file_name: &str) -> String {
    fs::read_to_string(Path::new(file_name)).unwrap()
}

pub fn load_lines(file_name: &str) -> Vec<String> {
    load_file(file_name)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn load_grid(file_name: &str) -> Grid<char> {
    let lines = load_file(file_name);

    Grid::new(lines)
}
