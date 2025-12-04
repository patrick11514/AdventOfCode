use std::{
    fmt::{Debug, Display},
    fs,
    path::Path,
};

pub fn load_file(file_name: &str) -> String {
    fs::read_to_string(Path::new(file_name)).unwrap()
}

pub fn load_lines(file_name: &str) -> Vec<String> {
    load_file(file_name)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn load_grid(file_name: &str) -> Grid {
    let lines = load_lines(file_name);
    let data = lines
        .into_iter()
        .map(|str| str.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    Grid::new(data)
}

#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
    empty: char,
    occupied: char,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self
            .data
            .clone()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .concat()
            })
            .collect::<Vec<String>>()
            .join("\n");
        f.write_str(&text)
    }
}

impl Grid {
    pub fn new(data: Vec<Vec<char>>) -> Self {
        Self {
            width: if data.len() == 0 { 0 } else { data[0].len() },
            height: data.len(),
            data,
            empty: '.',
            occupied: '@',
        }
    }

    pub fn set_chars(&mut self, empty: char, occupued: char) {
        self.empty = empty;
        self.occupied = occupued;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_element(&self, row: usize, col: usize) -> Option<char> {
        if row >= self.height || col >= self.width {
            None
        } else {
            Some(self.data[row][col])
        }
    }

    pub fn set_element(&mut self, row: usize, col: usize, char: char) {
        if row >= self.height || col >= self.width {
            return;
        }
        self.data[row][col] = char;
    }

    pub fn get_neighbors(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.height || col >= self.width {
            return None;
        }

        if self.data[row][col] != self.occupied {
            return None;
        }

        let mut count = 0;

        for r in row.saturating_sub(1)..=(row + 1).min(self.height - 1) {
            for c in col.saturating_sub(1)..=(col + 1).min(self.width - 1) {
                if r == row && c == col {
                    continue;
                }

                if self.data[r][c] == self.occupied {
                    count += 1;
                }
            }
        }

        Some(count)
    }
}

pub struct GridIterator {
    grid: Grid,
    row: usize,
    col: usize,
}

pub struct GridItem {
    pub ch: char,
    pub row: usize,
    pub col: usize,
}

impl IntoIterator for Grid {
    type Item = GridItem;

    type IntoIter = GridIterator;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: self,
            row: 0,
            col: 0,
        }
    }
}

impl Iterator for GridIterator {
    type Item = GridItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.grid.height || self.col >= self.grid.width {
            None
        } else {
            let item = GridItem {
                ch: self.grid.data[self.row][self.col],
                row: self.row,
                col: self.col,
            };

            self.col += 1;
            if self.col >= self.grid.height {
                self.col = 0;
                self.row += 1;
            }

            Some(item)
        }
    }
}
