use std::fs;

pub fn load_file(day: u32) -> String {
    let path = if day < 10 {
        format!("inputs/day0{day}.txt")
    } else {
        format!("inputs/day{day}.txt")
    };

    match fs::read_to_string(path.clone()) {
        Ok(res) => res,
        Err(_) => panic!("Unable to load {path}"),
    }
}

pub fn split_to_number(str: &str, ch: char) -> Vec<i32> {
    str.split(ch).map(|item| item.parse::<i32>().unwrap()).collect()
}
