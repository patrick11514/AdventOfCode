use advent_of_code_2025::{grid::GridItem, load_grid};

fn main() {
    let grid = load_grid("input.txt");

    let mut valid_rolls = 0;
    for GridItem { col, row, .. } in grid.clone().into_iter() {
        if let Some(v) = grid.get_neighbors(row, col, |ch| ch == &'@')
            && v < 4
        {
            valid_rolls += 1;
        }
    }
    println!("Star 1: {valid_rolls}");

    let mut removed_rolls = 0;

    let mut next_grid = grid.clone();

    loop {
        let mut to_remove = Vec::new();

        for GridItem { col, row, .. } in next_grid.clone().into_iter() {
            if let Some(v) = next_grid.get_neighbors(row, col, |ch| ch == &'@')
                && v < 4
            {
                to_remove.push((row, col));
            }
        }

        if to_remove.len() == 0 {
            break;
        }

        removed_rolls += to_remove.len();

        for (row, col) in to_remove {
            next_grid.set_element(row, col, '.');
        }
    }

    println!("Star 2: {removed_rolls}");
}
