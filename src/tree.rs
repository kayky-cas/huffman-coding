use std::{fmt::Display, str::FromStr};

struct Node {
    name: Option<char>,
    value: f32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: f32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self {
            name: None,
            value,
            left,
            right,
        }
    }

    fn new_char(name: char, value: f32) -> Self {
        Self {
            name: Some(name),
            value,
            left: None,
            right: None,
        }
    }
}

pub struct Conding {
    buffer: [u8],
    root: Box<Node>,
}

impl From<&[u8]> for Conding {
    fn from(buffer: &[u8]) -> Self {
        let mut arr = [0usize; 256];

        for &x in buffer {
            arr[x as usize] += 1;
        }

        let mut nodes: Vec<_> = arr
            .iter()
            .enumerate()
            .flat_map(|(index, &value)| {
                if value == 0 {
                    None
                } else {
                    Some(Box::new(Node::new_char(
                        char::from_u32(index as u32).unwrap(),
                        (value as f32 * 100.0) / buffer.len() as f32,
                    )))
                }
            })
            .collect();

        nodes.sort_by(|curr, oth| oth.value.total_cmp(&curr.value));

        for node in &nodes {
            print!("{:?} ", node.value);
        }

        println!();

        let left = nodes.pop().unwrap();
        let right = nodes.pop().unwrap();

        let root = Self::populate_tree(
            Box::new(Node::new(left.value + right.value, Some(left), Some(right))),
            &mut nodes,
        );

        Self {
            root,
            buffer: buffer.clone(),
        }
    }
}

impl FromStr for Conding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.as_bytes().into())
    }
}

impl Conding {
    fn populate_tree(mut root: Box<Node>, arr: &mut Vec<Box<Node>>) -> Box<Node> {
        if arr.len() < 2 {
            let left = arr.pop().unwrap();

            if root.value > left.value {
                return Box::new(Node::new(root.value + left.value, Some(left), Some(root)));
            } else {
                return Box::new(Node::new(root.value + left.value, Some(root), Some(left)));
            }
        }

        let left = arr.pop().unwrap();
        let right = arr.pop().unwrap();

        if root.value + left.value < left.value + right.value {
            arr.push(right);
            if root.value > left.value {
                root = Self::populate_tree(
                    Box::new(Node::new(root.value + left.value, Some(left), Some(root))),
                    arr,
                );
            } else {
                root = Self::populate_tree(
                    Box::new(Node::new(root.value + left.value, Some(root), Some(left))),
                    arr,
                );
            }
        } else {
            arr.push(Box::new(Node::new(
                left.value + right.value,
                Some(left),
                Some(right),
            )));

            arr.sort_by(|curr, oth| oth.value.total_cmp(&curr.value));
            root = Self::populate_tree(root, arr);
        }
        root
    }

    pub fn encode(&self) -> String {
        String::new()
    }
}
