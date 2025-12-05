use aoc_2024::load_file;

fn get_sum(data: String, check_do: bool) -> i32 {
    let mut total_sum = 0;
    let mut enabled = true;

    for i in 0..data.len() {
        let slice = &data[i..];

        if check_do {
            if slice.starts_with("do()") {
                enabled = true;
                continue;
            }

            if slice.starts_with("don't()") {
                enabled = false;
                continue;
            }

            if !enabled {
                continue;
            }
        }

        if !slice.starts_with("mul(")  {continue;}
      
        let slice = &slice[4..];

        let num1_index = match slice.find(',') {
            Some(index) => index,
            None => continue,
        };
        let num1 = &slice[..num1_index];

        let num1 =   match num1.parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let after = &slice[num1_index + 1..];

        let bracket = match after.find(')') {
            Some(index) => index,
            None => continue 
        };

        let num2 = &after[..bracket];

        let num2 =   match num2.parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        
        total_sum += num1 * num2;

    }

    total_sum
}

fn main() -> anyhow::Result<()> {
    let data = load_file(3);

    println!("1. star {}", get_sum(data.clone(), false));
    println!("2. star {}", get_sum(data, true));
    Ok(())
}
