use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = (Vec<Node>, Vec<usize>);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let mut all_nodes = Vec::new();

        let root_nodes = file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                let (r, _) = parse_subnode(&line, &mut all_nodes);
                r
            })
            .collect();

        (all_nodes, root_nodes)
    }

    fn solve_first(&self, (all_nodes, root_nodes): &Self::Input) -> Result<Self::Output1, String> {
        let mut all_nodes = all_nodes.clone();

        let mut root_id = root_nodes[0];

        for i in 1..root_nodes.len() {
            let id = root_nodes[i];
            root_id = sum(&mut all_nodes, root_id, id);

            while reduce(&mut all_nodes, root_id) {}
        }

        Ok(magnitude(&all_nodes, root_id))
    }

    fn solve_second(&self, (all_nodes, root_nodes): &Self::Input) -> Result<Self::Output2, String> {
        let total_nodes = root_nodes.len();

        let mut result = 0;

        for i in 0..total_nodes {
            for j in 0..total_nodes {
                if i == j {
                    continue;
                }

                let mut all_nodes = all_nodes.clone();
                let id = sum(&mut all_nodes, root_nodes[i], root_nodes[j]);
                while reduce(&mut all_nodes, id) {}
                result = result.max(magnitude(&all_nodes, id));
            }
        }

        Ok(result)
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

fn reduce(tree: &mut Vec<Node>, root: usize) -> bool {
    if try_explode(tree, root, 0) {
        return true;
    }
    return try_split(tree, root);
}

fn try_explode(tree: &mut Vec<Node>, root: usize, level: usize) -> bool {
    let node_type = tree[root].node_type.clone();

    match node_type {
        NodeType::Pair(p) => {
            if level == 4 {
                explode(tree, root, p.clone());
                return true;
            }
            if try_explode(tree, p.left, level + 1) {
                return true;
            }
            if try_explode(tree, p.right, level + 1) {
                return true;
            }
            false
        }
        _ => false,
    }
}
fn try_split(tree: &mut Vec<Node>, root: usize) -> bool {
    let node_type = tree[root].node_type.clone();

    match node_type {
        NodeType::Pair(p) => {
            if try_split(tree, p.left) {
                return true;
            }
            if try_split(tree, p.right) {
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

fn _print(tree: &Vec<Node>, id: usize) -> String {
    match &tree[id].node_type {
        NodeType::Regular(v) => format!("{}", v),
        NodeType::Pair(pair) => {
            format!("[{},{}]", _print(tree, pair.left), _print(tree, pair.right))
        }
    }
}

fn magnitude(tree: &Vec<Node>, id: usize) -> usize {
    match &tree[id].node_type {
        NodeType::Regular(v) => *v as usize,
        NodeType::Pair(pair) => 3 * magnitude(tree, pair.left) + 2 * magnitude(tree, pair.right),
    }
}
