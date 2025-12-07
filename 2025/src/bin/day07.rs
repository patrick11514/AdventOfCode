use std::{cell::RefCell, collections::HashMap, rc::Rc};

use advent_of_code_2025::{grid::Grid, load_file};

type Storage = HashMap<usize, Rc<RefCell<Node>>>;

fn main() {
    let string = load_file("example.txt");
    let mut grid = Grid::new(string);

    let mut splits = 0usize;
    let mut node_storage = Storage::new();

    let mut root = None;

    for row in 1..grid.height() {
        let mut row_storage = Storage::new();

        for col in 0..grid.width() {
            let up = grid.get_element(row - 1, col).unwrap();
            if up == &'S' {
                let char = grid.get_element_mut(row, col).unwrap();
                *char = '|';

                let root_node = Rc::new(RefCell::new(Node::Leaf));

                node_storage.insert(col, root_node.clone());
                root = Some(root_node);

                continue;
            }

            if up == &'|' {
                let char = grid.get_element_mut(row, col).unwrap();
                if char != &'^' {
                    *char = '|';
                    continue;
                }

                splits += 1;

                let left = grid.get_element_mut(row, col - 1).unwrap();
                *left = '|';
                let right = grid.get_element_mut(row, col + 1).unwrap();
                *right = '|';

                let left_node = get_or_new(&row_storage, &mut node_storage, col - 1);
                let right_node = get_or_new(&row_storage, &mut node_storage, col + 1);

                let node = node_storage.get(&col).unwrap();

                *node.borrow_mut() = Node::Branch {
                    left: left_node.clone(),
                    right: right_node.clone(),
                };

                row_storage.insert(col + 1, right_node);

                continue;
            }
        }
    }

    println!("Star 1: {splits}");

    let graph = Graph {
        root: root.unwrap(),
    };

    let mut timelines = 0usize;

    traverse(&mut timelines, graph.root);

    println!("Star 2: {timelines}");
}

fn traverse(timelines: &mut usize, node: Rc<RefCell<Node>>) {
    match node.borrow().clone() {
        Node::Branch { left, right } => {
            traverse(timelines, left);
            traverse(timelines, right);
        }
        Node::Leaf => {
            *timelines += 1;
        }
    };
}

fn get_or_new(local: &Storage, glob: &mut Storage, idx: usize) -> Rc<RefCell<Node>> {
    match local.get(&idx) {
        Some(v) => v.clone(),
        None => {
            let new = Rc::new(RefCell::new(Node::default()));
            glob.insert(idx, new.clone());
            new
        }
    }
}

#[derive(Clone, Default)]
enum Node {
    Branch {
        left: Rc<RefCell<Node>>,
        right: Rc<RefCell<Node>>,
    },
    #[default]
    Leaf,
}

#[derive(Clone)]
struct Graph {
    root: Rc<RefCell<Node>>,
}
