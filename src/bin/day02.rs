use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

fn main() {
    let solution = load_solution("input.txt");

    let res: u128 = solution.into_iter().map(|range| range.is_valid()).sum();

    println!("RES: {res}");
}

fn load_solution(file_name: &str) -> Vec<IdRange> {
    let line = fs::read_to_string(Path::new(file_name)).unwrap();
    line.split(',')
        .filter_map(|range| match range.split_once('-') {
            Some((l, r)) => Some(IdRange {
                start: l.to_string(),
                end: r.to_string(),
            }),
            None => None,
        })
        .collect()
}

#[derive(Debug)]
struct IdRange {
    start: String,
    end: String,
}

impl IdRange {
    fn is_valid(&self) -> u128 {
        let start = self.start.parse::<u128>().unwrap();
        let end = self.end.parse::<u128>().unwrap();

        let mut total_invalid = HashSet::new();

        for num in start..=end {
            if !IdRange::check_valid(&num.to_string()) {
                total_invalid.insert(num.to_string());
            }
        }

        if total_invalid.len() == 0 {
            0
        } else {
            total_invalid
                .into_iter()
                .map(|v| v.parse::<u128>().unwrap())
                .sum()
        }
    }

    fn check_valid(number: &str) -> bool {
        let len = number.len();

        for window_size in 1..=len {
            if len / window_size * window_size != len {
                continue; //quick check if division is full number
            }

            let mut occurances = HashMap::<&str, usize>::new();

            for idx in 1..=(len / window_size) {
                let from = (idx - 1) * window_size;
                let to = idx * window_size;

                let current = &number[from..to];

                *occurances.entry(current).or_default() += 1;
            }

            // STAR 1 = if occurances.values().any(|v| v == &2) && occurances.len() == 1 {
            if occurances.values().any(|v| v >= &2) && occurances.len() == 1 {
                return false;
            }
        }

        true
    }
}
