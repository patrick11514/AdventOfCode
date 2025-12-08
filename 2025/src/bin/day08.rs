use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use advent_of_code_2025::load_lines;

fn main() {
    #[allow(non_snake_case)]
    let EXAMPLE = false;

    let lines = load_lines(if EXAMPLE { "example.txt" } else { "input.txt" });

    let mut boxes = Boxes::new(lines);

    let mut distances = boxes.get_distances();

    let mut boxes2 = boxes.clone();
    let distances2 = distances.clone();

    for _ in 0..(if EXAMPLE { 10 } else { 1000 }) {
        let (idx, _) = match distances
            .iter()
            .min_by(|f, s| f.1.partial_cmp(s.1).unwrap())
        {
            Some((idx, dist)) => (idx.clone(), dist.clone()),
            None => break,
        };

        distances.remove(&idx);

        boxes.assign_idx(idx.first, idx.second);

        //let first = boxes.get_box(idx.first).unwrap().clone();
        //let second = boxes.get_box(idx.second).unwrap().clone();

        //println!("{:?},{:?}: {}", first, second, dist);
    }

    let mut circuits = boxes.get_circuits();
    //println!("{circuits:?}");

    circuits.sort();
    circuits.reverse();

    println!("Star 1: {}", circuits[0] * circuits[1] * circuits[2]);

    //Star 2

    let mut last = None;

    loop {
        let (idx, _) = match distances2
            .iter()
            .filter(|pair| {
                //if both are connected, ignore them
                let idx = pair.0;

                let box1 = boxes2.get_box(idx.first).unwrap();
                let box2 = boxes2.get_box(idx.second).unwrap();

                !(box1.idx.is_some() && box2.idx.is_some() && box1.idx == box2.idx)
            })
            .min_by(|f, s| f.1.partial_cmp(s.1).unwrap())
        {
            Some((idx, dist)) => (idx.clone(), dist.clone()),
            None => break,
        };

        //distances2.remove(&idx);

        boxes2.assign_idx(idx.first, idx.second);

        last = Some(idx);

        /*let first = boxes.get_box(idx.first).unwrap().clone();
        let second = boxes.get_box(idx.second).unwrap().clone();

        println!("{:?},{:?}", first, second);*/
    }

    if let Some(idx) = last {
        println!(
            "Star 2: {}",
            boxes.get_box(idx.first).unwrap().x * boxes.get_box(idx.second).unwrap().x
        );
    }
}

#[derive(Debug, Clone)]
struct IdxPair {
    first: usize,
    second: usize,
}

impl Hash for IdxPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first.min(self.second).hash(state);
        self.first.max(self.second).hash(state);
    }
}

impl PartialEq for IdxPair {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
            || self.first == other.second && self.second == other.first
    }
}

impl Eq for IdxPair {}

impl From<(usize, usize)> for IdxPair {
    fn from(value: (usize, usize)) -> Self {
        Self {
            first: value.0,
            second: value.1,
        }
    }
}

#[derive(Debug, Clone)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
    idx: Option<usize>,
}

#[derive(Debug, Clone)]
struct Boxes {
    pub last_idx: usize,
    boxes: Vec<JunctionBox>,
}

fn euklid(x: &JunctionBox, y: &JunctionBox) -> f64 {
    (((x.x - y.x).pow(2) + (x.y - y.y).pow(2) + (x.z - y.z).pow(2)) as f64).sqrt()
}

impl Boxes {
    fn new(lines: Vec<String>) -> Self {
        Self {
            last_idx: 0,
            boxes: lines
                .into_iter()
                .map(|line| {
                    line.split(',')
                        .map(|str| str.parse().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|parts| JunctionBox {
                    x: parts[0],
                    y: parts[1],
                    z: parts[2],
                    idx: None,
                })
                .collect(),
        }
    }

    fn get_box(&self, idx: usize) -> Option<&JunctionBox> {
        if idx >= self.boxes.len() {
            None
        } else {
            Some(&self.boxes[idx])
        }
    }

    fn get_distances(&self) -> HashMap<IdxPair, f64> {
        let mut distances = HashMap::new();

        for x in 0..self.boxes.len() {
            for y in 0..self.boxes.len() {
                if x == y {
                    continue;
                }

                let pair = (x, y).into();

                if distances.contains_key(&pair) {
                    continue;
                }

                distances.insert(pair, euklid(&self.boxes[x], &self.boxes[y]));
            }
        }

        distances
    }

    fn assign_idx(&mut self, idx1: usize, idx2: usize) {
        assert!(idx1 != idx2);

        let idx = match self.boxes[idx1].idx {
            Some(idx) => idx,
            None => match self.boxes[idx2].idx {
                Some(idx) => idx,
                None => {
                    self.last_idx += 1;
                    self.last_idx
                }
            },
        };

        //both of them have ids, we need to migrate one id to another for all nodes
        let prev_idx2 = self.boxes[idx2].idx;

        self.boxes[idx1].idx = Some(idx);
        self.boxes[idx2].idx = Some(idx);

        if let Some(old_idx) = prev_idx2 {
            for _box in self.boxes.iter_mut() {
                if _box.idx == Some(old_idx) {
                    _box.idx = Some(idx);
                }
            }
        }
    }

    fn get_circuits(&self) -> Vec<usize> {
        let mut counts = HashMap::<usize, usize>::new();
        for _box in self.boxes.iter() {
            if let Some(idx) = _box.idx {
                *counts.entry(idx).or_default() += 1;
            }
        }

        counts.values().map(|v| *v).collect()
    }
}
