use std::{fs, path::Path};

fn main() {
    let solution = load_solution("input.txt");
    let res1: u128 = solution
        .clone()
        .into_iter()
        .map(|line| find_biggest_combination(line, 2))
        .sum();
    println!("Solution 1: {res1}");

    let res2: u128 = solution
        .into_iter()
        .map(|line| find_biggest_combination(line, 12))
        .sum();

    println!("Solution 2: {res2}");
}

fn load_solution(file_name: &str) -> Vec<String> {
    let file = fs::read_to_string(Path::new(file_name)).unwrap();
    file.lines().map(|l| l.to_string()).collect()
}

fn find_biggest_combination(line: String, nums: usize) -> u128 {
    let chars = line.chars().collect::<Vec<_>>();
    let mut start_next = 0;

    let mut number = 0u128;

    for mut i in 0..nums {
        let orig_idx = i;

        let rev_i = nums - i - 1;
        let max_idx = chars.len() - rev_i - 1;

        let mut max_digit = (start_next, chars[start_next].to_digit(10).unwrap());
        i = start_next;

        loop {
            i += 1;

            if i > max_idx {
                break;
            }

            let digit = chars[i].to_digit(10).unwrap();
            if digit > max_digit.1 {
                max_digit = (i, digit);
            }
        }

        number += max_digit.1 as u128 * 10u128.pow((nums - orig_idx - 1) as u32);
        start_next = max_digit.0 + 1;
    }

    number
}
