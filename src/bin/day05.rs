use advent_of_code_2025::load_lines;

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}-{}", self.start, self.end))
    }
}

fn main() {
    let mut ranges = Vec::new();
    let mut ingrediens = Vec::new();

    let lines = load_lines("input.txt");

    let mut to_ranges = true;

    for line in lines.into_iter() {
        if line.is_empty() {
            to_ranges = false;
            continue;
        }

        if to_ranges {
            let (left, right) = line.split_once('-').unwrap();
            ranges.push(Range {
                start: left.parse::<usize>().unwrap(),
                end: right.parse::<usize>().unwrap(),
            });
        } else {
            ingrediens.push(line.parse::<usize>().unwrap());
        }
    }

    let mut fresh = 0;

    for ingredient in ingrediens.into_iter() {
        for range in &ranges {
            if range.includes(&ingredient) {
                fresh += 1;
                break;
            }
        }
    }

    println!("Star 1: {fresh}");

    let mut new_ranges = ranges;
    loop {
        let prev = new_ranges.len();
        new_ranges = reduce_ranges(new_ranges);

        if prev == new_ranges.len() {
            break;
        }
    }

    println!(
        "Star 2: {}",
        new_ranges.into_iter().map(|r| r.len()).sum::<usize>()
    );
}

#[derive(PartialEq, Debug)]
enum Overlap {
    Start,
    End,
    Inner,
    Outer,
    Equals,
    No,
}

impl Into<bool> for Overlap {
    fn into(self) -> bool {
        match self {
            Overlap::No => false,
            _ => true,
        }
    }
}

impl Range {
    fn len(&self) -> usize {
        (self.start..=self.end).count()
    }

    fn includes(&self, num: &usize) -> bool {
        num >= &self.start && num <= &self.end
    }

    fn overlap(&self, other: &Range) -> Overlap {
        if other.start == self.start && other.end == self.end {
            Overlap::Equals
        } else if other.start >= self.start && other.end <= self.end {
            Overlap::Inner
        } else if self.start >= other.start && self.end <= other.end {
            Overlap::Outer
        } else if self.start >= other.start && other.end >= self.start && other.end <= self.end {
            Overlap::Start
        } else if other.start >= self.start && other.start <= self.end && other.end >= self.end {
            Overlap::End
        } else {
            Overlap::No
        }
    }

    fn combine(&self, other: &Range) -> Self {
        //They must overlap :)
        assert_ne!(self.overlap(&other), Overlap::No);

        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

fn reduce_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    let mut new_ranges = Vec::new();

    let mut size = ranges.len();

    let mut i = 0;
    while i < size {
        if i >= size {
            break; //because size will change
        }

        let mut found = false;

        for l in i + 1..size {
            if ranges[i].overlap(&ranges[l]).into() {
                new_ranges.push(ranges[i].combine(&ranges[l]));
                ranges.remove(l);
                size = ranges.len();

                found = true;
                break;
            }
        }

        if !found {
            new_ranges.push(ranges[i].clone());
        }

        i += 1;
    }

    new_ranges
}
