use aoc_2024::load_file;

fn star1(lines: &Vec<&str>) -> u32 {
    let increment: Vec<(i32, i32)> = vec![
        (1,0), // forward
        (-1, 0), //backward
        (0,1), //down
        (0,-1), // up
        (1,1), //forward-bottom
        (-1,-1), //backward-up
        (-1,1), //backward-bottom
        (1,-1), //forward-up
    ];

    let word = "XMAS";
    let word_size = word.len() as i32;

    let mut words = 0;

    for y in 0..lines.len() {
        let width = lines[y].len() as i32;
        for x in 0..width {
            for inc in &increment {
                //check if possible
                let last_x = x + ((word_size - 1) * inc.0);
                let last_y = y as i32 + ((word_size - 1) * inc.1);
                if last_x < 0 || last_x >= lines[y].len() as i32 || last_y < 0 || last_y >= lines.len() as i32 {
                    continue; 
                }

                let mut curr_x = x as i32;
                let mut curr_y = y as i32;

                //try search
                let mut found = true;
                for c in word.chars() {
                    if let Some(char) = lines[curr_y as usize].chars().nth(curr_x as usize) {
                        if char != c {
                            found = false;
                            break;
                        }

                        curr_x += inc.0 as i32;
                        curr_y += inc.1 as i32;
                        continue;
                    }
                    break;
                }

                if !found {
                    continue;
                }  

                words += 1;
            }    
        }
    }

    words
}

fn star2(lines: &Vec<&str>) -> u32 {
    let offsets: Vec<Vec<(i32, i32, char)>> = vec![
        vec![
            (-1, -1, 'M'),
            (1, 1, 'S'),
            (1, -1, 'M'),
            (-1, 1, 'S')
        ],
        vec![
            (-1, -1, 'M'),
            (1, 1, 'S'),
            (1, -1, 'S'),
            (-1, 1, 'M')
        ],
        vec![
            (-1, -1, 'S'),
            (1, 1, 'M'),
            (1, -1, 'M'),
            (-1, 1, 'S')
        ],
        vec![
            (-1, -1, 'S'),
            (1, 1, 'M'),
            (1, -1, 'S'),
            (-1, 1, 'M')
        ]
    ];

    let height = lines.len() as i32;
    
    let mut x_mases = 0;

    for y in 0..lines.len() {
        let width = lines[y].len() as i32;

        for x in 0..lines[y].len() {
            if let Some(char) = lines[y].chars().nth(x) {
                if char != 'A' {
                    continue;
                }

                let x = x as i32;
                let y = y as i32;

                //check
                if x - 1 < 0 || x + 1 >= width || y - 1 < 0 || y + 1 >= height  {
                    continue;
                }

                for offset_list in &offsets {
                    let mut valid = true;
                    for offset in offset_list {
                        if let Some(c) = lines[(y + offset.1) as usize].chars().nth((x + offset.0) as usize) {
                            if c != offset.2 {
                                valid = false;
                                break;
                            }
                            continue;
                        }
                        valid = false;
                        break;
                    }

                    if !valid {
                        continue;
                    }

                    x_mases += 1;
                }
                
            } 
        }
    }

    x_mases
}

fn main() -> anyhow::Result<()> {
    let load_file = load_file(4);
    let lines: Vec<&str> = load_file.lines().collect();

    
    println!("1. star {}", star1(&lines));
    println!("2. star {}", star2(&lines));

    Ok(())
}
