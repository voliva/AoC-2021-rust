use lazy_static::__Deref;

use super::Solver;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::{Rc, Weak};
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<(Vec<Node>, usize)>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                let mut tree = Vec::new();
                let (r, _) = parse_subnode(&line, &mut tree);
                (tree, r)
            })
            .collect()
    }

    fn solve_first(&self, lines: &Self::Input) -> Result<Self::Output1, String> {
        // let result = reduce(&root[0], 0);
        let mut cloned_lines = lines.clone();

        let (tree, id) = &mut cloned_lines[0];

        println!("{}", print(tree, *id));

        reduce(tree, *id, 0);

        println!("{}", print(tree, *id));

        todo!()
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}

// Tree: Vec<Node>
#[derive(Debug, Clone)]
struct Pair {
    left: usize,
    right: usize,
}
#[derive(Debug, Clone)]
enum NodeType {
    Regular(u8),
    Pair(Pair),
}
#[derive(Debug, Clone)]
pub struct Node {
    parent: Option<usize>,
    node_type: NodeType,
}

fn parse_subnode<'a>(s: &'a str, tree: &mut Vec<Node>) -> (usize, &'a str) {
    if s.starts_with("[") {
        let (left, s_left) = parse_subnode(&s[1..], tree);
        let (right, s_right) = parse_subnode(&s_left[1..], tree);

        let id = sum(tree, left, right);

        (id, &s_right[1..])
    } else {
        let value = s[..1].parse().unwrap();

        let id = tree.len();
        let node = Node {
            parent: None,
            node_type: NodeType::Regular(value),
        };
        tree.push(node);

        (id, &s[1..])
    }
}

fn insert_node(tree: &mut Vec<Node>, node_type: NodeType) -> usize {
    let id = tree.len();
    let node = Node {
        parent: None,
        node_type,
    };
    tree.push(node);
    id
}

fn sum(tree: &mut Vec<Node>, left: usize, right: usize) -> usize {
    let id = tree.len();
    tree[left].parent = Some(id);
    tree[right].parent = Some(id);
    let node = Node {
        parent: None,
        node_type: NodeType::Pair(Pair { left, right }),
    };
    tree.push(node);
    id
}

fn reduce(tree: &mut Vec<Node>, root: usize, level: usize) -> bool {
    let node_type = tree[root].node_type.clone();

    match node_type {
        NodeType::Pair(p) => {
            if level == 4 {
                explode(tree, root, p.clone());
                return true;
            }
            if reduce(tree, p.left, level + 1) {
                return true;
            }
            if reduce(tree, p.right, level + 1) {
                return true;
            }
            false
        }
        NodeType::Regular(v) if v >= 10 => {
            split(tree, root, v);
            true
        }
        _ => false,
    }
}

fn split(tree: &mut Vec<Node>, id: usize, value: u8) {
    let left_value = value / 2;
    let right_value = if value % 2 == 0 {
        left_value
    } else {
        left_value + 1
    };

    let left_id = insert_node(tree, NodeType::Regular(left_value));
    let right_id = insert_node(tree, NodeType::Regular(right_value));
    tree[left_id].parent = Some(id);
    tree[right_id].parent = Some(id);

    tree[id].node_type = NodeType::Pair(Pair {
        left: left_id,
        right: right_id,
    });
}

fn explode(tree: &mut Vec<Node>, id: usize, pair: Pair) {
    let left_value = (if let NodeType::Regular(x) = tree[pair.left].node_type {
        Some(x)
    } else {
        None
    })
    .unwrap();
    let right_value = (if let NodeType::Regular(x) = tree[pair.right].node_type {
        Some(x)
    } else {
        None
    })
    .unwrap();

    sum_left_up(tree, tree[id].parent.unwrap(), left_value, id);
    sum_right_up(tree, tree[id].parent.unwrap(), right_value, id);

    tree[id].node_type = NodeType::Regular(0);
}

fn sum_right_up(tree: &mut Vec<Node>, id: usize, value: u8, origin: usize) {
    match &tree[id].node_type {
        NodeType::Regular(v) => {
            tree[id].node_type = NodeType::Regular(*v + value);
        }
        NodeType::Pair(pair) => {
            if pair.right == origin {
                match tree[id].parent {
                    Some(parent) => sum_right_up(tree, parent, value, id),
                    _ => {}
                }
            } else {
                let right = pair.right;
                sum_left_down(tree, right, value);
            }
        }
    }
}
fn sum_left_down(tree: &mut Vec<Node>, id: usize, value: u8) {
    match &tree[id].node_type {
        NodeType::Regular(v) => {
            tree[id].node_type = NodeType::Regular(*v + value);
        }
        NodeType::Pair(pair) => {
            let left = pair.left;
            sum_left_down(tree, left, value);
        }
    }
}
fn sum_left_up(tree: &mut Vec<Node>, id: usize, value: u8, origin: usize) {
    match &tree[id].node_type {
        NodeType::Regular(v) => {
            tree[id].node_type = NodeType::Regular(*v + value);
        }
        NodeType::Pair(pair) => {
            if pair.left == origin {
                match tree[id].parent {
                    Some(parent) => sum_left_up(tree, parent, value, id),
                    _ => {}
                }
            } else {
                let left = pair.left;
                sum_right_down(tree, left, value);
            }
        }
    }
}
fn sum_right_down(tree: &mut Vec<Node>, id: usize, value: u8) {
    match &tree[id].node_type {
        NodeType::Regular(v) => {
            tree[id].node_type = NodeType::Regular(*v + value);
        }
        NodeType::Pair(pair) => {
            let right = pair.right;
            sum_right_down(tree, right, value);
        }
    }
}

fn print(tree: &Vec<Node>, id: usize) -> String {
    match &tree[id].node_type {
        NodeType::Regular(v) => format!("{}", v),
        NodeType::Pair(pair) => format!("[{},{}]", print(tree, pair.left), print(tree, pair.right)),
    }
}

// fn sum(left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
//     let result = Rc::new(RefCell::new(Node {
//         parent: None,
//         side: None,
//         value: NodeValue::Pair {
//             left: Rc::clone(&left),
//             right: Rc::clone(&right),
//         },
//     }));

//     left.borrow_mut().parent = Some(Rc::downgrade(&result));
//     left.borrow_mut().side = Some(Side::Left);
//     right.borrow_mut().parent = Some(Rc::downgrade(&result));
//     right.borrow_mut().side = Some(Side::Right);

//     result
// }

// fn reduce(root: &Rc<RefCell<Node>>, level: usize) -> bool {
//     let value = &root.borrow().value;
//     match value {
//         NodeValue::Pair { left, right } => {
//             if level == 4 {
//                 explode(root);
//                 return true;
//             }
//             if reduce(left, level + 1) {
//                 return true;
//             }
//             if reduce(right, level + 1) {
//                 return true;
//             }
//             false
//         }
//         NodeValue::Regular(v) if *v >= 10 => {
//             split(root, *v);
//             true
//         }
//         _ => false,
//     }
// }

// fn split(node: &Rc<RefCell<Node>>, value: u8) {
//     let left_value = value / 2;
//     let right_value = if value % 2 == 0 {
//         left_value
//     } else {
//         left_value + 1
//     };

//     let new_node = sum(
//         create_regular_node(left_value),
//         create_regular_node(right_value),
//     );

//     let borrowed = node.borrow();
//     let parent_ref = borrowed.parent.as_ref().unwrap();

//     if let Some(parent) = parent_ref.upgrade() {
//         match borrowed.side {
//             Some(Side::Left) => {
//                 let parent_value =
//                 if let NodeValue::Pair { left, right } = &parent.borrow_mut().value {

//                 }
//             }
//             Some(Side::Right) => {}
//             _ => unreachable!(),
//         }
//     } else {
//         panic!("Splitting a node with no parent");
//     }
// }

// fn explode(node: &Rc<RefCell<Node>>) {}

// #[derive(Debug)]
// enum Side {
//     Left,
//     Right,
// }

// #[derive(Debug)]
// pub struct Node {
//     parent: Option<Weak<RefCell<Node>>>,
//     side: Option<Side>,
//     value: NodeValue,
// }

// fn parse_subnode(s: &str) -> (Rc<RefCell<Node>>, &str) {
//     if s.starts_with("[") {
//         let (left, s_left) = parse_subnode(&s[1..]);
//         let (right, s_right) = parse_subnode(&s_left[1..]);

//         let result = sum(left, right);

//         (result, &s_right[1..])
//     } else {
//         let max = s.find(|c| c < '0' || c > '9').unwrap_or(s.len());
//         let result = create_regular_node(s[..max].parse().unwrap());

//         (result, &s[max..])
//     }
// }

// fn create_regular_node(value: u8) -> Rc<RefCell<Node>> {
//     Rc::new(RefCell::new(Node {
//         parent: None,
//         side: None,
//         value: NodeValue::Regular(value),
//     }))
// }

// #[derive(Debug)]
// enum NodeValue {
//     Regular(u8),
//     Pair {
//         left: Rc<RefCell<Node>>,
//         right: Rc<RefCell<Node>>,
//     },
// }
