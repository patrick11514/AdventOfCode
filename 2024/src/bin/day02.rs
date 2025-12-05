use core::panic;

use aoc_2024::load_file;

enum State {
    Unset,
    Decreasing,
    Incresing
}

fn is_safe(numbers: &Vec<i32>) -> bool {
    let mut state = State::Unset;

    for index in 0..(numbers.len() - 1) {
        let diff = numbers[index] - numbers[index + 1];
        if index == 0 {
            state = if diff < 0 {
                State::Incresing
            } else {
                State::Decreasing
            };
        }

        let abs_diff = diff.abs();

        if abs_diff.abs() == 0 || abs_diff > 3 {
            state = State::Unset;
            break;
        }

        match state {
            State::Unset => {},
            State::Incresing => {
                if diff > 0 {
                    state = State::Unset;
                    break;
                }
            },
            State::Decreasing => {
                if diff < 0 {
                    state = State::Unset;
                    break;
                }
            },
        }

    }

    match state {
        State::Unset => false,
        State::Decreasing | State::Incresing => true
    }
}

fn try_variants(nums: &Vec<i32>) -> bool {
    let mut result = is_safe(nums);
    if result {
        return true;
    }

    for index in 0..nums.len() {
       let mut sliced = nums.clone();
        sliced.remove(index);
        result = is_safe(&sliced);
        if result {
            return true;
        }
    }

    false
}

fn main() -> anyhow::Result<()> {
    let mut safe = 0;
    let mut safe_module = 0;

    for line in load_file(2).lines() {
        if line.len() == 0 {
            continue;
        }

        let nums: Vec<i32> = line.split(' ').map(|item|  match item.parse::<i32>() {
            Ok(num) => num,
            Err(_) => panic!("Unable to convert {item}"),
        }).collect();

        if is_safe(&nums) {
            safe += 1;
        } 

        if try_variants(&nums) {
            safe_module += 1;
        } 
    }

    println!("1. star {safe}");
    println!("2. star {safe_module}");

    Ok(())
}
