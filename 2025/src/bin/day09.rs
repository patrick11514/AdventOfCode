use advent_of_code_2025::load_file;

fn main() {
    let text = load_file("example.txt");
    let points = text
        .clone()
        .lines()
        .into_iter()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|vec| Point {
            x: vec[0],
            y: vec[1],
        })
        .collect::<Vec<_>>();

    let mut max = 0;

    for x in 0..points.len() {
        for y in (x + 1)..points.len() {
            let area = get_area(&points[x], &points[y]);
            if area > max {
                max = area;
            }
        }
    }

    println!("Star 1: {max}");

    let mut max = 0;

    for x in 0..points.len() {
        for y in (x + 1)..points.len() {
            let p1 = &points[x];
            let p2 = &points[y];

            //Somehow figure out, how to check if out of shape

            let area = get_area(p1, p2);

            println!("{p1:?} {p2:?} {area}");
            if area > max {
                max = area;
            }
        }
    }

    println!("Star 2: {max}");
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
}

fn get_area(p1: &Point, p2: &Point) -> i64 {
    ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1)
}
