use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>,
}

impl<T: Display + Clone> Display for Grid<T> {
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

pub trait IntoGridInput<T> {
    fn to_grid(self) -> Vec<Vec<T>>;
}

impl<T> IntoGridInput<T> for Vec<Vec<T>> {
    fn to_grid(self) -> Vec<Vec<T>> {
        self
    }
}

impl IntoGridInput<char> for String {
    fn to_grid(self) -> Vec<Vec<char>> {
        self.lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect()
    }
}

impl<T> Grid<T> {
    pub fn new<I: IntoGridInput<T>>(input: I) -> Self {
        let data = input.to_grid();

        Self {
            width: if data.len() == 0 { 0 } else { data[0].len() },
            height: data.len(),
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_element(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.height || col >= self.width {
            None
        } else {
            Some(&self.data[row][col])
        }
    }

    pub fn set_element(&mut self, row: usize, col: usize, value: T) {
        if row >= self.height || col >= self.width {
            return;
        }
        self.data[row][col] = value;
    }

    pub fn get_neighbors<F>(&self, row: usize, col: usize, is_valid_cell: F) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        if row >= self.height || col >= self.width {
            return None;
        }

        if !is_valid_cell(&self.data[row][col]) {
            return None;
        }

        let mut count = 0;

        for r in row.saturating_sub(1)..=(row + 1).min(self.height - 1) {
            for c in col.saturating_sub(1)..=(col + 1).min(self.width - 1) {
                if r == row && c == col {
                    continue;
                }

                if is_valid_cell(&self.data[r][c]) {
                    count += 1;
                }
            }
        }

        Some(count)
    }

    pub fn get_row(&self, row: usize) -> Option<&Vec<T>> {
        if row >= self.height {
            None
        } else {
            Some(&self.data[row])
        }
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn get_neighbors_same_type<F>(
        &self,
        row: usize,
        col: usize,
        is_valid_cell: F,
    ) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        if row >= self.height || col >= self.width {
            return None;
        }

        if !is_valid_cell(&self.data[row][col]) {
            return None;
        }

        let mut count = 0;

        for r in row.saturating_sub(1)..=(row + 1).min(self.height - 1) {
            for c in col.saturating_sub(1)..=(col + 1).min(self.width - 1) {
                if r == row && c == col {
                    continue;
                }

                if &self.data[r][c] == &self.data[row][col] {
                    count += 1;
                }
            }
        }

        Some(count)
    }
}

pub struct GridIterator<T> {
    grid: Grid<T>,
    row: usize,
    col: usize,
}

pub struct GridItem<T> {
    pub item: T,
    pub row: usize,
    pub col: usize,
}

impl<T: Clone> IntoIterator for Grid<T> {
    type Item = GridItem<T>;

    type IntoIter = GridIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: self,
            row: 0,
            col: 0,
        }
    }
}

impl<T: Clone> Iterator for GridIterator<T> {
    type Item = GridItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.grid.height || self.col >= self.grid.width {
            None
        } else {
            let item = GridItem {
                item: self.grid.data[self.row][self.col].clone(),
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
