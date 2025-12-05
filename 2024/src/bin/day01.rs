use aoc_2024::load_file;

fn main() -> anyhow::Result<()> {
    let mut first: Vec<i32> = vec![];
    let mut second: Vec<i32> = vec![];

    for line in load_file(1).lines() {
        if line.len() == 0 { continue;}

        let cols = line.split("  ").collect::<Vec<&str>>(); 

        first.push(cols[0].trim().parse::<i32>()?);
        second.push(cols[1].trim().parse::<i32>()?);
    }

    first.sort();
    second.sort();

    let mut sum = 0;

    for i in 0..first.len() {
        let diff = first[i] - second[i];
        sum += diff.abs();
    }
    println!("1. star: {}", sum);

    let mut similarity = 0;

    for left in first {
        let count = second.iter().filter(|item| **item == left).count() as i32;
        similarity += left * count;
    }

    println!("2. star: {}", similarity);

    Ok(())
}
