use std::cmp::Ordering;

use aoc_2024::{load_file, split_to_number};

fn check(deps: &Vec<(i32, i32)>, row: &[i32]) -> bool {
    if row.len() == 1 {
        return true;
    }

    let mut itter = row.into_iter();
    let first = itter.next().unwrap();

    let item_deps: Vec<&(i32, i32)> = deps.iter().filter(|item| item.0 == *first).collect();

    for i in itter {
        if item_deps.iter().find(|item| item.1 == *i) == None {
            return false;
        }
    }

    check(deps, &row[1..])
}

fn main() -> anyhow::Result<()> {
    let load_file = load_file(5);
    let mut deps: Vec<(i32, i32)> = vec![];
    
    let mut loading_deps = true;
    let mut final_sum = 0;
    let mut incorrect_sum = 0;

    for line in load_file.lines() {
        if line.len() == 0 {
            loading_deps = false;
            continue;
        }

        if loading_deps {
            let split = split_to_number(line, '|');

            if split.len() == 2 {
                deps.push((split[0], split[1]));
            }
            continue;
        }
            
        let nums = split_to_number(line, ',');

        if !check(&deps, &nums) {
            //fix
            let mut sorted = nums.clone();
            sorted.sort_by(|a, b| {
                if deps.iter().find(|item| item.0 == *a && item.1 == *b) != None {
                    return Ordering::Less;
                }

                if deps.iter().find(|item| item.1 == *a && item.0 == *b) != None {
                    return Ordering::Greater;
                }

                Ordering::Equal
            });

    
            let center = sorted.len() / 2;
            incorrect_sum += sorted[center];

           continue;
        }
        
        let center = nums.len() / 2;
        final_sum += nums[center];
    }
    println!("1. star {}", final_sum);
    println!("2. star {}", incorrect_sum);

    Ok(())
}
